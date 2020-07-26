#[macro_use]
extern crate quick_error;

use locate_cargo_manifest::{locate_manifest, LocateManifestError};

mod error;
mod format_changeset;

use bytesize::ByteSize;
use criner_waste_report::{
    tar_path_to_utf8_str, CargoConfig, Fix, Patterns, Report, TarHeader, TarPackage, WastedFile,
};
pub use error::Error;
use format_changeset::format_changeset;
use std::{
    fs,
    io::{BufRead, BufReader, Cursor},
    path::{Path, PathBuf},
    str::FromStr,
};

pub type Result<T> = std::result::Result<T, Error>;

fn report_lean_crate(mut out: impl std::io::Write) -> std::io::Result<()> {
    writeln!(out, "Your crate is perfectly lean!")
}

fn report_savings(
    total_size_in_bytes: u64,
    total_files: u64,
    wasted_files: Vec<WastedFile>,
    mut out: impl std::io::Write,
) -> std::io::Result<()> {
    if wasted_files.is_empty() {
        writeln!(
            out,
            "Include or exclude directives were optimized, without affecting the crate's size."
        )?;
        return Ok(());
    }

    let file_len = wasted_files.len();
    let wasted_bytes = entries_to_table(wasted_files, &mut out, None)?;

    writeln!(
        out,
        "Saved {:.0}% or {} in {} files (of {} and {} files in entire crate)",
        (wasted_bytes as f32 / total_size_in_bytes as f32) * 100.0,
        ByteSize(wasted_bytes),
        file_len,
        ByteSize(total_size_in_bytes),
        total_files
    )?;
    Ok(())
}

fn entries_to_table(
    mut entries: Vec<WastedFile>,
    mut out: impl std::io::Write,
    items: Option<usize>,
) -> std::io::Result<u64> {
    if entries.is_empty() {
        return Ok(0);
    }
    use ascii_table::{Align, AsciiTable, Column};
    use std::fmt::Display;

    let mut ascii_table = AsciiTable::default();
    ascii_table.max_width = termsize::get()
        .unwrap_or(termsize::Size { rows: 20, cols: 80 })
        .cols as usize;
    ascii_table.columns.insert(
        0,
        Column {
            header: "File".to_string(),
            align: Align::Left,
            ..Default::default()
        },
    );
    ascii_table.columns.insert(
        1,
        Column {
            header: "Size (Byte)".to_string(),
            align: Align::Right,
            ..Default::default()
        },
    );

    entries.sort_by(|x, y| y.1.cmp(&x.1));
    let bytes: u64 = entries
        .iter()
        .take(items.unwrap_or(entries.len()))
        .rev()
        .map(|(_, size)| size)
        .sum();
    let data: Vec<Vec<&dyn Display>> = entries
        .iter()
        .take(items.unwrap_or(entries.len()))
        .rev()
        .map(|(path, size)| vec![path as &dyn Display, size as &dyn Display])
        .collect();
    out.write_all(ascii_table.format(data).as_bytes())?;
    Ok(bytes)
}

fn edit(
    mut doc: toml_edit::Document,
    package: TarPackage,
    output: impl std::io::Write,
) -> Result<toml_edit::Document> {
    let report = Report::from_package(
        "crate-name does not matter",
        "crate version does not matter",
        package,
    );
    match report {
        Report::Version { total_size_in_bytes, total_files, wasted_files, suggested_fix, .. } => {
            if let Some(fix) = suggested_fix {
                report_savings(total_size_in_bytes, total_files, wasted_files, output).ok();
                match fix {
                    Fix::EnrichedExclude { exclude, .. } => set_exclude(&mut doc, exclude),
                    Fix::NewInclude { include, ..} | Fix::ImprovedInclude { include, .. } => set_include(&mut doc, include),
                    Fix::RemoveExcludeAndUseInclude { include, .. }=> {
                        remove_exclude(&mut doc);
                        set_include(&mut doc, include);
                    }
                    Fix::RemoveExclude => {
                        remove_exclude(&mut doc);
                    }
                };
            } else {
                report_lean_crate(output).ok();
            }
        },
        _ => unreachable!("Reports should always start out as Versions - this should probably not be necessary here"),
    };
    Ok(doc)
}

fn set_package_array(doc: &mut toml_edit::Document, key: &str, patterns: Patterns) {
    doc["package"][key] = toml_edit::value({
        let mut v = toml_edit::Array::default();
        for pattern in patterns.into_iter() {
            v.push(pattern).unwrap();
        }
        v
    });
}

fn set_exclude(doc: &mut toml_edit::Document, exclude: Patterns) {
    set_package_array(doc, "exclude", exclude)
}

fn set_include(doc: &mut toml_edit::Document, include: Patterns) {
    set_package_array(doc, "include", include)
}

fn remove_exclude(doc: &mut toml_edit::Document) {
    doc.as_table_mut()
        .entry("package")
        .as_table_mut()
        .expect("table")
        .remove("exclude");
}

fn into_manifest_location_error(err: LocateManifestError) -> Error {
    if let LocateManifestError::CargoExecution { stderr } = err {
        Error::LocateManifestExecution(std::str::from_utf8(&stderr).unwrap().to_owned())
    } else {
        Error::LocateManifest(err)
    }
}

fn tar_package_from_paths(lines: Vec<u8>) -> Result<TarPackage> {
    let mut entries = Vec::new();
    let mut meta_entries = Vec::new();
    let paths = BufReader::new(Cursor::new(lines))
        .lines()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let cargo_manifest = CargoConfig::from(
        fs::read(
            paths
                .iter()
                .find(|p| *p == "Cargo.toml")
                .expect("cargo manifest"),
        )?
        .as_slice(),
    );
    let interesting_paths = {
        let mut v = vec!["Cargo.toml".to_string()];
        v.push(
            cargo_manifest
                .actual_or_expected_build_script_path()
                .to_owned(),
        );
        v.push(cargo_manifest.lib_path().to_owned());
        v.extend(cargo_manifest.bin_paths().into_iter().map(|s| s.to_owned()));
        v
    };

    let root = Path::new("r");
    for path in paths {
        if path == "Cargo.toml.orig" || path == "Cargo.lock" {
            continue;
        }

        const REGULAR_FILE: u8 = b'0';
        let meta = fs::metadata(&path).map_err(|err| Error::FileMetadata(err, path.to_owned()))?;
        meta_entries.push(TarHeader {
            path: root.join(&path).to_string_lossy().as_bytes().to_owned(),
            size: meta.len(),
            entry_type: REGULAR_FILE,
        });

        if interesting_paths.iter().any(|p| p == &path) {
            entries.push((
                TarHeader {
                    path: root.join(&path).to_string_lossy().as_bytes().to_owned(),
                    size: meta.len(),
                    entry_type: REGULAR_FILE,
                },
                fs::read(path)?,
            ))
        }
    }
    Ok(TarPackage {
        entries_meta_data: meta_entries,
        entries,
    })
}

fn cargo_package_content() -> Result<TarPackage> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = std::process::Command::new(cargo)
        .arg("package")
        .arg("--no-verify")
        .arg("--offline")
        .arg("--allow-dirty")
        .arg("--quiet")
        .arg("--list")
        .output()?;
    if !output.status.success() {
        Err(Error::CargoPackageError(
            std::str::from_utf8(&output.stderr)
                .map(|s| s.to_owned())
                .unwrap_or_else(|_| String::from_utf8_lossy(&output.stderr).to_string()),
        )
        .into())
    } else {
        tar_package_from_paths(output.stdout)
    }
}

fn clear_includes_and_excludes(doc: &mut toml_edit::Document) {
    let package = doc
        .as_table_mut()
        .entry("package")
        .as_table_mut()
        .expect("table");
    package.remove("exclude");
    package.remove("include");
}

fn write_manifest(
    manifest_path: &Path,
    document: toml_edit::Document,
    original_manifest_content: String,
    dry_run: bool,
    mut output: impl std::io::Write,
    with_color: bool,
) -> Result<()> {
    let edit = document.to_string_in_original_order();
    if !dry_run {
        std::fs::write(manifest_path, &edit)?;
    }
    if original_manifest_content == edit {
        paint!(
            output,
            with_color,
            ansi_term::Style::new().dimmed(),
            "{}\n",
            if dry_run {
                "There would be no change."
            } else {
                "Nothing changed."
            }
        );
    } else {
        writeln!(output)?;
        paint!(
            output,
            with_color,
            ansi_term::Style::new().dimmed(),
            "{}\n",
            if dry_run {
                "The following change WOULD be made to Cargo.toml:"
            } else {
                "The following change was made to Cargo.toml:"
            }
        );
        format_changeset(
            output,
            with_color,
            &difference::Changeset::new(&original_manifest_content, &edit, "\n"),
        )?
    };
    Ok(())
}

#[derive(Debug, Default)]
pub struct Options {
    pub reset: bool,
    pub dry_run: bool,
    pub colored_output: bool,
    pub list: Option<usize>,
    pub package_size_limit: Option<u64>,
    #[cfg(feature = "dev-support")]
    pub save_package_for_unit_test: Option<PathBuf>,
}

fn check_package_size(
    package: TarPackage,
    package_size_limit: u64,
    mut output: impl std::io::Write,
    with_color: bool,
) -> Result<()> {
    struct ByteCounter(u64);
    impl std::io::Write for ByteCounter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0 += buf.len() as u64;
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    };

    let actual_estimated_package_size = {
        let byte_counter = ByteCounter(0);
        let mut builder = tar::Builder::new(
            flate2::GzBuilder::new().write(byte_counter, flate2::Compression::best()),
        );
        // NOTE: we are not taking generated files into consideration, but assume the space required for them is negligble
        // The reason we do things in memory is to avoid having side-effects, like dropping files to disk by invoking `cargo package` directly.
        let base = PathBuf::from("cratename-v0.1.0");
        for entry in package.entries_meta_data.into_iter() {
            let mut header = tar::Header::new_ustar();
            let path_without_root = tar_path_to_utf8_str(&entry.path);
            header.set_path(&base.join(path_without_root))?;

            let mut file = std::fs::File::open(path_without_root)?;
            let metadata = file.metadata()?;
            header.set_metadata(&metadata);
            header.set_cksum();
            builder.append(&header, &mut file)?;
        }
        let encoder = builder.into_inner()?;
        encoder.finish()?.0
    };

    if actual_estimated_package_size > package_size_limit {
        return Err(Error::PackageSizeLimitExceeded(
            actual_estimated_package_size,
            package_size_limit,
        ));
    }
    paint!(
        output,
        with_color,
        ansi_term::Colour::Green,
        "{}\n",
        format!(
            "The estimated actual package size of {} is within the limit of {}.",
            ByteSize(actual_estimated_package_size),
            ByteSize(package_size_limit)
        )
    );
    Ok(())
}

#[cfg(feature = "dev-support")]
fn write_package_for_unit_tests(path: Option<PathBuf>, package: &TarPackage) -> Result<()> {
    if let Some(path) = path {
        std::fs::write(
            path,
            rmp_serde::to_vec(package).expect("rmp-serde to always work here"),
        )?;
    };
    Ok(())
}

pub fn execute(options: Options, mut output: impl std::io::Write) -> Result<()> {
    let manifest_path = locate_manifest().map_err(into_manifest_location_error)?;

    let cargo_manifest_original_content = std::fs::read_to_string(&manifest_path)?;
    let mut document = toml_edit::Document::from_str(&cargo_manifest_original_content)?;

    if options.reset {
        clear_includes_and_excludes(&mut document);
        std::fs::write(&manifest_path, document.to_string_in_original_order())?;
    }
    let package = cargo_package_content()?;

    #[cfg(feature = "dev-support")]
    write_package_for_unit_tests(options.save_package_for_unit_test, &package)?;
    let package_size_limit = options.package_size_limit.map(|s| (package.clone(), s));
    // In dry-run mode, reset the manifest to its original state right after we obtained the package content
    // that saw the reset manifest file, i.e. one without includes or excludes
    if options.reset && options.dry_run {
        std::fs::write(&manifest_path, &cargo_manifest_original_content)?;
    }
    let list = options
        .list
        .map(|list| (list, package.entries_meta_data.clone()));
    let document = edit(document, package, &mut output)?;
    write_manifest(
        &manifest_path,
        document,
        cargo_manifest_original_content,
        options.dry_run,
        &mut output,
        options.colored_output,
    )?;

    if let Some((package, package_size_limit)) = package_size_limit {
        check_package_size(
            package,
            package_size_limit,
            &mut output,
            options.colored_output,
        )?;
    }
    if let Some((list, entries)) = list {
        list_entries(list, &mut output, entries)?;
    }
    Ok(())
}

fn list_entries(
    num_entries: usize,
    mut out: impl std::io::Write,
    entries: Vec<TarHeader>,
) -> Result<()> {
    let num_entries = if num_entries == 0 {
        entries.len()
    } else {
        num_entries.min(entries.len())
    };
    let bytes = entries_to_table(
        entries
            .into_iter()
            .map(|tf| (tar_path_to_utf8_str(&tf.path).into(), tf.size))
            .collect(),
        &mut out,
        Some(num_entries),
    )?;
    writeln!(
        out,
        "Crate contains a total of {} files and {}",
        num_entries,
        ByteSize(bytes)
    )?;
    Ok(())
}

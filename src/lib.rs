#[macro_use]
extern crate quick_error;

use locate_cargo_manifest::{locate_manifest, LocateManifestError};

mod error;
mod format_changeset;

use bytesize::ByteSize;
use criner_waste_report::{CargoConfig, Fix, Patterns, Report, TarHeader, TarPackage, WastedFile};
pub use error::Error;
use format_changeset::format_changeset;
use std::{
    fs,
    io::{BufRead, BufReader, Cursor},
    path::Path,
    str::FromStr,
};

pub type Result<T> = std::result::Result<T, Error>;

fn report_lean_crate(mut out: impl std::io::Write) -> std::io::Result<()> {
    writeln!(out, "Your crate is perfectly lean!")
}

fn report_savings(
    total_size_in_bytes: u64,
    total_files: u64,
    mut wasted_files: Vec<WastedFile>,
    mut out: impl std::io::Write,
) -> std::io::Result<()> {
    if wasted_files.is_empty() {
        writeln!(
            out,
            "Include or exclude directives were optimized, without affecting the crate's size."
        )?;
        return Ok(());
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

    wasted_files.sort_by(|x, y| x.1.cmp(&y.1));
    let wasted_bytes: u64 = wasted_files.iter().map(|(_, size)| size).sum();
    let data: Vec<Vec<&dyn Display>> = wasted_files
        .iter()
        .map(|(path, size)| vec![path as &dyn Display, size as &dyn Display])
        .collect();
    ascii_table.print(data);
    writeln!(
        out,
        "Saved {:.0}% or {} in {} files (of {} and {} files in entire crate)",
        (wasted_bytes as f32 / total_size_in_bytes as f32) * 100.0,
        ByteSize(wasted_bytes),
        wasted_files.len(),
        ByteSize(total_size_in_bytes),
        total_files
    )?;
    Ok(())
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
            v.push(pattern);
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
    let cargo = std::env::var("CARGO").unwrap_or("cargo".to_owned());
    let output = std::process::Command::new(cargo)
        .arg("package")
        .arg("--no-verify")
        .arg("--offline")
        .arg("--allow-dirty")
        .arg("--quiet")
        .arg("--list")
        .output()?;
    if !output.status.success() {
        Err(LocateManifestError::CargoExecution {
            stderr: output.stderr,
        }
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
    manifest_path: std::path::PathBuf,
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
    // In dry-run mode, reset the manifest to its original state right after we obtained the package content
    // that saw the reset manifest file, i.e. one without includes or excludes
    if options.reset && options.dry_run {
        std::fs::write(&manifest_path, &cargo_manifest_original_content)?;
    }
    let document = edit(document, package, &mut output)?;
    write_manifest(
        manifest_path,
        document,
        cargo_manifest_original_content,
        options.dry_run,
        output,
        options.colored_output,
    )?;
    Ok(())
}

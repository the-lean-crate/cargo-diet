#[macro_use]
extern crate quick_error;

use locate_cargo_manifest::{locate_manifest, LocateManifestError};

mod error;
mod format_changeset;

use criner_waste_report::{CargoConfig, Fix, Patterns, Report, TarHeader, TarPackage, WastedFile};
pub use error::Error;
use format_changeset::format_changeset;
use std::path::Path;
use std::{
    fs,
    io::{BufRead, BufReader, Cursor},
    str::FromStr,
};

pub type Result<T> = std::result::Result<T, Error>;

fn report_lean_crate(mut out: impl std::io::Write) -> std::io::Result<()> {
    writeln!(out, "Your crate is perfectly lean!")
}

fn report_savings(
    total_size_in_bytes: u64,
    wasted_files: Vec<WastedFile>,
    mut out: impl std::io::Write,
) -> std::io::Result<()> {
    writeln!(
        out,
        "We saved you {} in {} files",
        bytesize::ByteSize(total_size_in_bytes),
        wasted_files.len()
    )?;
    for (path, _size) in wasted_files {
        writeln!(out, "{}", path)?;
    }
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
        Report::Version { total_size_in_bytes, wasted_files, suggested_fix, ..} => {
            if let Some(fix) = suggested_fix {
                report_savings(total_size_in_bytes, wasted_files, output).ok();
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
    original_content_on_dry_run: Option<String>,
    mut output: impl std::io::Write,
) -> Result<()> {
    let edit = document.to_string_in_original_order();
    match original_content_on_dry_run {
        None => std::fs::write(manifest_path, edit)?,
        Some(original_content) => {
            if original_content == edit {
                writeln!(output, "There would be no change.")?;
            } else {
                writeln!(output, "WOULD apply the following change:")?;
                format_changeset(
                    output,
                    &difference::Changeset::new(&original_content, &edit, ""),
                )?
            }
        }
    };
    Ok(())
}

#[derive(Debug, Default)]
pub struct Options {
    pub reset: bool,
    pub dry_run: bool,
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
        if options.dry_run {
            Some(cargo_manifest_original_content)
        } else {
            None
        },
        output,
    )?;
    Ok(())
}

#[macro_use]
extern crate quick_error;

use locate_cargo_manifest::{locate_manifest, LocateManifestError};
use std::str::FromStr;

mod error;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

fn edit(mut doc: toml_edit::Document) -> Result<toml_edit::Document> {
    doc["package"]["include"] = toml_edit::value({
        let mut v = toml_edit::Array::default();
        v.push("src/*");
        v
    });
    Ok(doc)
}

fn into_manifest_location_error(err: LocateManifestError) -> Error {
    if let LocateManifestError::CargoExecution { stderr } = err {
        Error::LocateManifestExecution(std::str::from_utf8(&stderr).unwrap().to_owned())
    } else {
        Error::LocateManifest(err)
    }
}

pub fn execute() -> Result<()> {
    let manifest_path = locate_manifest().map_err(into_manifest_location_error)?;
    let document = edit(toml_edit::Document::from_str(&std::fs::read_to_string(
        &manifest_path,
    )?)?)?;
    std::fs::write(manifest_path, document.to_string_in_original_order())?;
    Ok(())
}

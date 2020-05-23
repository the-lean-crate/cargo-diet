#[macro_use]
extern crate quick_error;

mod error {
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Bug(d: &'static str) {
                display("{}", d)
            }
            Message(d: String) {
                display("{}", d)
            }
            LocateManifest(err: locate_cargo_manifest::LocateManifestError) {
                from()
                cause(err)
            }
            LocateManifestExecution(msg: String) {
                display("{}", msg)
            }
        }
    }
}

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub fn fun() -> Result<()> {
    let _ = locate_cargo_manifest::locate_manifest().map_err(|err| {
        if let locate_cargo_manifest::LocateManifestError::CargoExecution { stderr } = err {
            Error::LocateManifestExecution(std::str::from_utf8(&stderr).unwrap().to_owned())
        } else {
            Error::LocateManifest(err)
        }
    })?;
    Ok(())
}

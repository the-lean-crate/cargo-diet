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
            Io(err: std::io::Error) {
                from()
                cause(err)
            }
        }
    }
}

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub fn fun(path: &std::path::Path) -> Result<()> {
    std::fs::read(path)?;
    Ok(())
}

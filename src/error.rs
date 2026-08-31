use bytesize::ByteSize;
use std::path::PathBuf;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("{0}")]
    Bug(&'static str),
    #[error("{0}")]
    Message(String),
    #[error(
        "{0}\n\nTo try fixing this, run 'cargo package' by hand before running 'cargo diet' again."
    )]
    CargoPackageError(String),
    #[error(transparent)]
    TomlParse(#[from] toml_edit::TomlError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(
        "The actual estimated package size of {} exceeded the limit of {} by {}",
        ByteSize(*.0).display().si(),
        ByteSize(*.1).display().si(),
        ByteSize(.0.saturating_sub(*.1)).display().si()
    )]
    PackageSizeLimitExceeded(u64, u64),
    #[error("Could not open {1:?} for reading file meta-data")]
    FileMetadata(#[source] std::io::Error, String),
    #[error("{0}")]
    LocateManifestExecution(String),
    #[error("{0}")]
    CargoMetadataError(String),
    #[error(
        "`cargo metadata` missing expected `{0}` field. This may mean an outdated cargo version (found: {1})"
    )]
    ExpectedMetadataField(&'static str, String),
    #[error("`{0}` did not match any packages")]
    PackageSpecNotFound(String),
    #[error("{0:?} is missing the required `package.name` field")]
    MissingPackageName(PathBuf),
    #[error(transparent)]
    JsonParse(#[from] json::Error),
}

impl Error {
    pub(crate) fn message(msg: impl Into<String>) -> Error {
        Error::Message(msg.into())
    }

    pub fn describe_with_chain(&self) -> String {
        use std::error::Error as _;
        let mut description = self.to_string();
        let mut source = self.source();
        while let Some(err) = source {
            description.push_str("\n\nCaused by:\n    ");
            description.push_str(&err.to_string());
            source = err.source();
        }
        description
    }
}

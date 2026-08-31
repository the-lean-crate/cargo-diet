use bytesize::ByteSize;
use quick_error::quick_error;
use std::path::PathBuf;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Bug(d: &'static str) {
            display("{}", d)
        }
        Message(d: String) {
            display("{}", d)
        }
        CargoPackageError(msg: String) {
            display("{}\n\nTo try fixing this, run 'cargo package' by hand before running 'cargo diet' again.", msg)
        }
        TomlParse(err: toml_edit::TomlError) {
            from()
            source(err)
        }
        Io(err: std::io::Error) {
            from()
            source(err)
        }
        PackageSizeLimitExceeded(actual_in_bytes: u64, limit_in_bytes: u64) {
            display("The actual estimated package size of {} exceeded the limit of {} by {}", ByteSize(*actual_in_bytes).display().si(), ByteSize(*limit_in_bytes).display().si(), ByteSize(actual_in_bytes.saturating_sub(*limit_in_bytes)).display().si())
        }
        FileMetadata(err: std::io::Error, path: String) {
            display("Could not open {:?} for reading file meta-data", path)
            source(err)
        }
        LocateManifestExecution(msg: String) {
            display("{}", msg)
        }
        CargoMetadataError(msg: String) {
            display("{}", msg)
        }
        ExpectedMetadataField(field: &'static str, cargo_version: String) {
            display("`cargo metadata` missing expected `{}` field. This may mean an outdated cargo version (found: {})", field, cargo_version)
        }
        PackageSpecNotFound(spec: String) {
            display("`{}` did not match any packages", spec)
        }
        MissingPackageName(manifest_path: PathBuf) {
            display("{:?} is missing the required `package.name` field", manifest_path)
        }
        JsonParse(err: serde_json::Error) {
            from()
            source(err)
        }
    }
}

impl Error {
    pub(crate) fn message(msg: impl Into<String>) -> Error {
        Error::Message(msg.into())
    }

    pub(crate) fn describe_with_chain(&self) -> String {
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

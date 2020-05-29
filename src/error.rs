use bytesize::ByteSize;
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
            display("{}", msg)
        }
        TomlParse(err: toml_edit::TomlError) {
            from()
            cause(err)
        }
        Io(err: std::io::Error) {
            from()
            cause(err)
        }
        PackageSizeLimitExceeded(actual_in_bytes: u64, limit_in_bytes: u64) {
            display("The actual estimated package size of {} exceeded the limit of {} by {}", ByteSize(*actual_in_bytes), ByteSize(*limit_in_bytes), ByteSize(actual_in_bytes.saturating_sub(*limit_in_bytes)))
        }
        FileMetadata(err: std::io::Error, path: String) {
            display("Could not open {:?} for reading file meta-data", path)
            cause(err)
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

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
            display("The actual estimated package size of {} exceeded the limit of {} by {}", ByteSize(*actual_in_bytes), ByteSize(*limit_in_bytes), ByteSize(actual_in_bytes.saturating_sub(*limit_in_bytes)))
        }
        FileMetadata(err: std::io::Error, path: String) {
            display("Could not open {:?} for reading file meta-data", path)
            source(err)
        }
        LocateManifest(err: locate_cargo_manifest::LocateManifestError) {
            from()
            source(err)
        }
        LocateManifestExecution(msg: String) {
            display("{}", msg)
        }
    }
}

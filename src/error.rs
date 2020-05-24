quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Bug(d: &'static str) {
            display("{}", d)
        }
        Message(d: String) {
            display("{}", d)
        }
        TomlParse(err: toml_edit::TomlError) {
            from()
            cause(err)
        }
        Io(err: std::io::Error) {
            from()
            cause(err)
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

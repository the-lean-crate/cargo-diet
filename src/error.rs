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
        LocateManifest(err: locate_cargo_manifest::LocateManifestError) {
            from()
            cause(err)
        }
        LocateManifestExecution(msg: String) {
            display("{}", msg)
        }
    }
}

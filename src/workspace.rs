use crate::{Error, Options, Result};
use locate_cargo_manifest::{LocateManifestError, locate_manifest};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: String,
    pub manifest_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub packages: Vec<Package>,
    pub default_member_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Target {
    pub name: String,
    pub manifest_path: PathBuf,
}

pub fn select_targets(options: &Options) -> Result<Vec<Target>> {
    if !options.exclude.is_empty() && !options.workspace {
        return Err(Error::message(
            "--exclude can only be used together with --workspace",
        ));
    }

    let manifest_path = locate_manifest().map_err(into_manifest_location_error)?;
    let document = toml_edit::DocumentMut::from_str(&std::fs::read_to_string(&manifest_path)?)?;
    let is_virtual_manifest = !document.contains_key("package");

    let targets = if options.workspace
        || !options.packages.is_empty()
        || is_virtual_manifest
        || document.contains_key("workspace")
    {
        let metadata = fetch(&manifest_path)?;
        resolve_targets(options, &metadata)?
    } else {
        let name = document["package"]["name"]
            .as_str()
            .ok_or_else(|| Error::MissingPackageName(manifest_path.clone()))?
            .to_owned();
        vec![Target {
            name,
            manifest_path: manifest_path.clone(),
        }]
    };

    if targets.is_empty() {
        return Err(Error::message("No packages matched"));
    }

    Ok(targets)
}

fn into_manifest_location_error(err: LocateManifestError) -> Error {
    if let LocateManifestError::CargoExecution { stderr } = err {
        Error::LocateManifestExecution(String::from_utf8_lossy(&stderr).into_owned())
    } else {
        Error::LocateManifest(err)
    }
}

pub fn fetch(manifest_path: &Path) -> Result<Metadata> {
    let output = crate::cargo_command()
        .arg("metadata")
        .arg("--no-deps")
        .arg("--format-version")
        .arg("1")
        .arg("--manifest-path")
        .arg(manifest_path)
        .output()?;
    if !output.status.success() {
        return Err(Error::CargoMetadataError(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ));
    }
    parse_metadata(&String::from_utf8_lossy(&output.stdout))
}

fn parse_metadata(text: &str) -> Result<Metadata> {
    let parsed = json::parse(text)?;

    let packages = parsed["packages"]
        .members()
        .map(|p| {
            Ok(Package {
                id: require_str(&p["id"], "packages[].id")?.to_owned(),
                name: require_str(&p["name"], "packages[].name")?.to_owned(),
                version: require_str(&p["version"], "packages[].version")?.to_owned(),
                manifest_path: PathBuf::from(require_str(
                    &p["manifest_path"],
                    "packages[].manifest_path",
                )?),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let workspace_default_members = &parsed["workspace_default_members"];
    if !workspace_default_members.is_array() {
        return Err(expected_metadata_field("workspace_default_members")?);
    }
    let default_member_ids = workspace_default_members
        .members()
        .map(|v| require_str(v, "workspace_default_members[]").map(str::to_owned))
        .collect::<Result<Vec<_>>>()?;

    Ok(Metadata {
        packages,
        default_member_ids,
    })
}

fn require_str<'a>(value: &'a json::JsonValue, field: &'static str) -> Result<&'a str> {
    match value.as_str() {
        Some(s) => Ok(s),
        None => Err(expected_metadata_field(field)?),
    }
}

fn expected_metadata_field(field: &'static str) -> Result<Error> {
    Ok(Error::ExpectedMetadataField(field, cargo_version()?))
}

fn cargo_version() -> Result<String> {
    let output = crate::cargo_command().arg("--version").output()?;
    if !output.status.success() {
        return Err(Error::message(format!(
            "`cargo --version` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn parse_spec(spec: &str) -> (&str, Option<&str>) {
    match spec.split_once('@') {
        Some((name, version)) => (name, Some(version)),
        None => (spec, None),
    }
}

fn version_matches(version: &str, spec: &str) -> bool {
    version == spec
        || version
            .strip_prefix(spec)
            .is_some_and(|rest| rest.starts_with('.'))
}

fn sorted(mut targets: Vec<Target>) -> Vec<Target> {
    targets.sort();
    targets
}

pub fn resolve_targets(options: &Options, metadata: &Metadata) -> Result<Vec<Target>> {
    if options.workspace {
        return Ok(sorted(
            metadata
                .packages
                .iter()
                .filter(|p| !options.exclude.iter().any(|e| e == &p.name))
                .map(|p| Target {
                    name: p.name.clone(),
                    manifest_path: p.manifest_path.clone(),
                })
                .collect(),
        ));
    }

    if !options.packages.is_empty() {
        let packages = options
            .packages
            .iter()
            .map(|spec| {
                let (name, version) = parse_spec(spec);
                metadata
                    .packages
                    .iter()
                    .find(|p| {
                        p.name == name && version.is_none_or(|v| version_matches(&p.version, v))
                    })
                    .ok_or_else(|| Error::PackageSpecNotFound(spec.clone()))
            })
            .collect::<Result<Vec<_>>>()?;
        return Ok(sorted(
            packages
                .into_iter()
                .map(|p| Target {
                    name: p.name.clone(),
                    manifest_path: p.manifest_path.clone(),
                })
                .collect(),
        ));
    }

    Ok(sorted(
        metadata
            .packages
            .iter()
            .filter(|p| metadata.default_member_ids.contains(&p.id))
            .map(|p| Target {
                name: p.name.clone(),
                manifest_path: p.manifest_path.clone(),
            })
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TWO_PACKAGES: &str = r#"{
        "packages": [
            {"id": "crate-a-id", "name": "crate-a", "version": "0.1.0", "manifest_path": "/ws/crate-a/Cargo.toml"},
            {"id": "crate-b-id", "name": "crate-b", "version": "0.1.0", "manifest_path": "/ws/crate-b/Cargo.toml"}
        ],
        "workspace_default_members": ["crate-a-id"]
    }"#;

    fn package(name: &str) -> Package {
        Package {
            id: format!("{name}-id"),
            name: name.to_owned(),
            version: "0.1.0".to_owned(),
            manifest_path: PathBuf::from(format!("/ws/{name}/Cargo.toml")),
        }
    }

    fn target(name: &str) -> Target {
        Target {
            name: name.to_owned(),
            manifest_path: PathBuf::from(format!("/ws/{name}/Cargo.toml")),
        }
    }

    fn metadata(default_member_ids: Vec<&str>) -> Metadata {
        Metadata {
            packages: vec![package("crate-a"), package("crate-b")],
            default_member_ids: default_member_ids.into_iter().map(str::to_owned).collect(),
        }
    }

    #[test]
    fn parses_packages_and_default_members() {
        let parsed = parse_metadata(TWO_PACKAGES).unwrap();
        assert_eq!(parsed, metadata(vec!["crate-a-id"]));
    }

    #[test]
    fn errors_clearly_when_default_members_field_is_missing() {
        let err =
            parse_metadata(r#"{"packages": [], "workspace_default_members": null}"#).unwrap_err();
        assert!(matches!(
            err,
            Error::ExpectedMetadataField("workspace_default_members", _)
        ));
    }

    #[test]
    fn explicit_empty_default_members_selects_nothing() {
        let parsed =
            parse_metadata(r#"{"packages": [], "workspace_default_members": []}"#).unwrap();
        assert_eq!(parsed.default_member_ids, Vec::<String>::new());
    }

    #[test]
    fn errors_clearly_when_a_required_package_field_is_missing() {
        let err = parse_metadata(
            r#"{"packages": [{"id": "a-id", "version": "0.1.0", "manifest_path": "/ws/a/Cargo.toml"}], "workspace_default_members": null}"#,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            Error::ExpectedMetadataField("packages[].name", _)
        ));
    }

    #[test]
    fn errors_clearly_when_a_default_member_entry_is_not_a_string() {
        let err =
            parse_metadata(r#"{"packages": [], "workspace_default_members": [42]}"#).unwrap_err();
        assert!(matches!(
            err,
            Error::ExpectedMetadataField("workspace_default_members[]", _)
        ));
    }

    fn options_with(packages: Vec<&str>, workspace: bool, exclude: Vec<&str>) -> Options {
        Options {
            packages: packages.into_iter().map(str::to_owned).collect(),
            workspace,
            exclude: exclude.into_iter().map(str::to_owned).collect(),
            ..Options::default()
        }
    }

    #[test]
    fn workspace_flag_silently_overrides_dash_p() {
        let targets = resolve_targets(
            &options_with(vec!["crate-a"], true, vec![]),
            &metadata(vec![]),
        )
        .unwrap();
        assert_eq!(targets, vec![target("crate-a"), target("crate-b")]);
    }

    #[test]
    fn exclude_applies_on_top_of_workspace() {
        let targets = resolve_targets(
            &options_with(vec![], true, vec!["crate-b"]),
            &metadata(vec![]),
        )
        .unwrap();
        assert_eq!(targets, vec![target("crate-a")]);
    }

    #[test]
    fn dash_p_sorts_by_name_rather_than_cli_order() {
        let targets = resolve_targets(
            &options_with(vec!["crate-b", "crate-a"], false, vec![]),
            &metadata(vec![]),
        )
        .unwrap();
        assert_eq!(targets, vec![target("crate-a"), target("crate-b")]);
    }

    #[test]
    fn dash_p_errors_on_unknown_spec() {
        let err = resolve_targets(
            &options_with(vec!["nope"], false, vec![]),
            &metadata(vec![]),
        )
        .unwrap_err();
        assert!(matches!(err, Error::PackageSpecNotFound(spec) if spec == "nope"));
    }

    #[test]
    fn dash_p_accepts_partial_versions() {
        for spec in ["crate-a@0", "crate-a@0.1", "crate-a@0.1.0"] {
            assert_eq!(
                resolve_targets(&options_with(vec![spec], false, vec![]), &metadata(vec![]))
                    .unwrap(),
                vec![target("crate-a")]
            );
        }
    }

    #[test]
    fn no_flags_falls_back_to_default_members() {
        let targets = resolve_targets(
            &options_with(vec![], false, vec![]),
            &metadata(vec!["crate-a-id"]),
        )
        .unwrap();
        assert_eq!(targets, vec![target("crate-a")]);
    }

    #[test]
    fn no_flags_with_explicit_empty_default_members_selects_nothing() {
        let targets =
            resolve_targets(&options_with(vec![], false, vec![]), &metadata(vec![])).unwrap();
        assert!(targets.is_empty());
    }
}

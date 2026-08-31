use crate::{Error, Options, Result};
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
    pub is_private: bool,
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

    let manifest_path = locate_manifest()?;
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

fn locate_manifest() -> Result<PathBuf> {
    let output = crate::cargo_command().arg("locate-project").output()?;
    if !output.status.success() {
        return Err(Error::LocateManifestExecution(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ));
    }
    let parsed: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let root = parsed["root"].as_str().ok_or_else(|| {
        Error::message("`cargo locate-project` output is missing the \"root\" field")
    })?;
    Ok(PathBuf::from(root))
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
    let parsed: serde_json::Value = serde_json::from_str(text)?;

    let packages = parsed["packages"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|p| {
            Ok(Package {
                id: require_str(&p["id"], "packages[].id")?.to_owned(),
                name: require_str(&p["name"], "packages[].name")?.to_owned(),
                version: require_str(&p["version"], "packages[].version")?.to_owned(),
                manifest_path: PathBuf::from(require_str(
                    &p["manifest_path"],
                    "packages[].manifest_path",
                )?),
                is_private: {
                    let publish = &p["publish"];
                    publish.as_array().is_some_and(|a| a.is_empty())
                },
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let workspace_default_members = &parsed["workspace_default_members"];
    let Some(workspace_default_members) = workspace_default_members.as_array() else {
        return Err(expected_metadata_field("workspace_default_members")?);
    };
    let default_member_ids = workspace_default_members
        .iter()
        .map(|v| require_str(v, "workspace_default_members[]").map(str::to_owned))
        .collect::<Result<Vec<_>>>()?;

    Ok(Metadata {
        packages,
        default_member_ids,
    })
}

fn require_str<'a>(value: &'a serde_json::Value, field: &'static str) -> Result<&'a str> {
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

fn package_matches_spec(package: &Package, spec: &str) -> bool {
    let (name, version) = parse_spec(spec);
    package.name == name && version.is_none_or(|v| version_matches(&package.version, v))
}

fn sorted(mut targets: Vec<Target>) -> Vec<Target> {
    targets.sort();
    targets
}

fn filter_targets<'a>(
    options: &Options,
    packages: impl IntoIterator<Item = &'a Package>,
) -> Vec<Target> {
    packages
        .into_iter()
        .filter(|p| !(options.ignore_private && p.is_private))
        .map(|p| Target {
            name: p.name.clone(),
            manifest_path: p.manifest_path.clone(),
        })
        .collect()
}

pub fn resolve_targets(options: &Options, metadata: &Metadata) -> Result<Vec<Target>> {
    if options.workspace {
        for spec in &options.exclude {
            if !metadata
                .packages
                .iter()
                .any(|package| package_matches_spec(package, spec))
            {
                return Err(Error::PackageSpecNotFound(spec.clone()));
            }
        }
        return Ok(sorted(filter_targets(
            options,
            metadata.packages.iter().filter(|p| {
                !options
                    .exclude
                    .iter()
                    .any(|spec| package_matches_spec(p, spec))
            }),
        )));
    }

    if !options.packages.is_empty() {
        let packages = options
            .packages
            .iter()
            .map(|spec| {
                metadata
                    .packages
                    .iter()
                    .find(|p| package_matches_spec(p, spec))
                    .ok_or_else(|| Error::PackageSpecNotFound(spec.clone()))
            })
            .collect::<Result<Vec<_>>>()?;
        return Ok(sorted(filter_targets(options, packages)));
    }

    Ok(sorted(filter_targets(
        options,
        metadata
            .packages
            .iter()
            .filter(|p| metadata.default_member_ids.contains(&p.id)),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CRATE_A: &str = "crate-a";
    const CRATE_B: &str = "crate-b";
    const CRATE_A_ID: &str = "crate-a-id";
    const CRATE_B_ID: &str = "crate-b-id";

    const TWO_PACKAGES: &str = r#"{
        "packages": [
            {"id": "crate-a-id", "name": "crate-a", "version": "0.1.0", "manifest_path": "/ws/crate-a/Cargo.toml"},
            {"id": "crate-b-id", "name": "crate-b", "version": "0.1.0", "manifest_path": "/ws/crate-b/Cargo.toml"}
        ],
        "workspace_default_members": ["crate-a-id"]
    }"#;

    const EVERY_PUBLISH_VALUE: &str = r#"{
        "packages": [
            {"id": "a-id", "name": "a", "version": "0.1.0", "manifest_path": "/ws/a/Cargo.toml", "publish": []},
            {"id": "b-id", "name": "b", "version": "0.1.0", "manifest_path": "/ws/b/Cargo.toml", "publish": null},
            {"id": "c-id", "name": "c", "version": "0.1.0", "manifest_path": "/ws/c/Cargo.toml", "publish": ["my-registry"]},
            {"id": "d-id", "name": "d", "version": "0.1.0", "manifest_path": "/ws/d/Cargo.toml"}
        ],
        "workspace_default_members": []
    }"#;

    fn package(name: &str) -> Package {
        Package {
            id: format!("{name}-id"),
            name: name.to_owned(),
            version: "0.1.0".to_owned(),
            manifest_path: PathBuf::from(format!("/ws/{name}/Cargo.toml")),
            is_private: false,
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
            packages: vec![package(CRATE_A), package(CRATE_B)],
            default_member_ids: default_member_ids.into_iter().map(str::to_owned).collect(),
        }
    }

    #[test]
    fn parses_packages_and_default_members() {
        let parsed = parse_metadata(TWO_PACKAGES).unwrap();
        assert_eq!(parsed, metadata(vec![CRATE_A_ID]));
    }

    #[test]
    fn only_an_empty_publish_array_marks_a_package_private() {
        let parsed = parse_metadata(EVERY_PUBLISH_VALUE).unwrap();
        let is_private: Vec<bool> = parsed.packages.iter().map(|p| p.is_private).collect();
        assert_eq!(is_private, vec![true, false, false, false]);
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
            &options_with(vec![CRATE_A], true, vec![]),
            &metadata(vec![]),
        )
        .unwrap();
        assert_eq!(targets, vec![target(CRATE_A), target(CRATE_B)]);
    }

    #[test]
    fn exclude_applies_on_top_of_workspace() {
        let targets = resolve_targets(
            &options_with(vec![], true, vec![CRATE_B]),
            &metadata(vec![]),
        )
        .unwrap();
        assert_eq!(targets, vec![target(CRATE_A)]);
    }

    #[test]
    fn exclude_resolves_package_specs_and_rejects_unknown_ones() {
        let targets = resolve_targets(
            &options_with(vec![], true, vec!["crate-b@0.1"]),
            &metadata(vec![]),
        )
        .unwrap();
        assert_eq!(targets, vec![target(CRATE_A)]);

        let err = resolve_targets(&options_with(vec![], true, vec!["nope"]), &metadata(vec![]))
            .unwrap_err();
        assert!(matches!(err, Error::PackageSpecNotFound(spec) if spec == "nope"));
    }

    #[test]
    fn dash_p_sorts_by_name_rather_than_cli_order() {
        let targets = resolve_targets(
            &options_with(vec![CRATE_B, CRATE_A], false, vec![]),
            &metadata(vec![]),
        )
        .unwrap();
        assert_eq!(targets, vec![target(CRATE_A), target(CRATE_B)]);
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
                vec![target(CRATE_A)]
            );
        }
    }

    #[test]
    fn no_flags_falls_back_to_default_members() {
        let targets = resolve_targets(
            &options_with(vec![], false, vec![]),
            &metadata(vec![CRATE_A_ID]),
        )
        .unwrap();
        assert_eq!(targets, vec![target(CRATE_A)]);
    }

    fn metadata_with_private_crate_b(default_member_ids: Vec<&str>) -> Metadata {
        let mut metadata = metadata(default_member_ids);
        metadata
            .packages
            .iter_mut()
            .find(|p| p.name == CRATE_B)
            .expect("crate-b is part of the fixture")
            .is_private = true;
        metadata
    }

    fn with_ignore_private(mut options: Options) -> Options {
        options.ignore_private = true;
        options
    }

    #[test]
    fn ignore_private_skips_private_members_with_workspace_flag() {
        let targets = resolve_targets(
            &with_ignore_private(options_with(vec![], true, vec![])),
            &metadata_with_private_crate_b(vec![]),
        )
        .unwrap();
        assert_eq!(targets, vec![target(CRATE_A)]);
    }

    #[test]
    fn ignore_private_skips_private_default_members() {
        let targets = resolve_targets(
            &with_ignore_private(options_with(vec![], false, vec![])),
            &metadata_with_private_crate_b(vec![CRATE_A_ID, CRATE_B_ID]),
        )
        .unwrap();
        assert_eq!(targets, vec![target(CRATE_A)]);
    }

    #[test]
    fn ignore_private_filters_explicitly_selected_private_packages() {
        let targets = resolve_targets(
            &with_ignore_private(options_with(vec![CRATE_B], false, vec![])),
            &metadata_with_private_crate_b(vec![]),
        )
        .unwrap();
        assert!(targets.is_empty());
    }

    #[test]
    fn private_members_are_kept_without_ignore_private() {
        let targets = resolve_targets(
            &options_with(vec![], true, vec![]),
            &metadata_with_private_crate_b(vec![]),
        )
        .unwrap();
        assert_eq!(targets, vec![target(CRATE_A), target(CRATE_B)]);
    }

    #[test]
    fn no_flags_with_explicit_empty_default_members_selects_nothing() {
        let targets =
            resolve_targets(&options_with(vec![], false, vec![]), &metadata(vec![])).unwrap();
        assert!(targets.is_empty());
    }
}

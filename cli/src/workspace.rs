use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use cargo_metadata::MetadataCommand;

pub struct Layout {
    pub features_manifest: PathBuf,
    pub scan_roots: Vec<PathBuf>,
    pub bindings_root: PathBuf,
    pub check_manifest: PathBuf,
}

pub fn detect(start: &std::path::Path) -> Result<Layout> {
    let metadata = MetadataCommand::new().current_dir(start).exec().context("running cargo metadata")?;

    let bindings = metadata
        .packages
        .iter()
        .find(|p| p.name.as_str() == "engage-il2cpp")
        .ok_or_else(|| anyhow!("no `engage-il2cpp` dependency in this project's cargo metadata"))?;

    let bindings_root: PathBuf = bindings.manifest_path.parent().unwrap().into();

    let workspace_member_ids: std::collections::HashSet<_> = metadata.workspace_members.iter().collect();

    let dependents: Vec<_> = metadata
        .packages
        .iter()
        .filter(|p| workspace_member_ids.contains(&p.id))
        .filter(|p| p.dependencies.iter().any(|d| d.name.as_str() == "engage-il2cpp"))
        .collect();

    if dependents.is_empty() {
        return Err(anyhow!(
            "no workspace member depends on `engage-il2cpp`. Run from within a project that uses it"
        ));
    }

    let owner = pick_owner(&metadata, &dependents);
    let features_manifest: PathBuf = owner.manifest_path.clone().into();

    let scan_roots: Vec<PathBuf> = dependents
        .iter()
        .map(|p| {
            let dir: PathBuf = p.manifest_path.parent().unwrap().into();
            dir.join("src")
        })
        .filter(|p| p.exists())
        .collect();

    Ok(Layout {
        features_manifest,
        scan_roots,
        bindings_root,
        check_manifest: owner.manifest_path.clone().into(),
    })
}

fn pick_owner<'a>(metadata: &'a cargo_metadata::Metadata, dependents: &[&'a cargo_metadata::Package]) -> &'a cargo_metadata::Package {
    let ws_root = metadata.workspace_root.as_std_path();
    let ws_toml = ws_root.join("Cargo.toml");

    if let Ok(raw) = std::fs::read_to_string(&ws_toml) {
        if let Ok(doc) = raw.parse::<toml_edit::DocumentMut>() {
            let has_ws_dep = doc
                .get("workspace")
                .and_then(|t| t.get("dependencies"))
                .and_then(|t| t.get("engage-il2cpp"))
                .is_some();

            if has_ws_dep {
                if let Some(p) = metadata.packages.iter().find(|p| p.manifest_path.as_std_path() == ws_toml) {
                    return p;
                }
            }
        }
    }

    dependents[0]
}

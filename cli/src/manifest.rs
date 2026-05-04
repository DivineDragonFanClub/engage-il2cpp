use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub schema_version: u32,
    #[serde(rename = "crate")]
    pub crate_name: String,
    #[allow(dead_code)]
    pub version: String,
    pub paths: BTreeMap<String, String>,
    pub dependencies: BTreeMap<String, Vec<String>>,
}

const SCHEMA_VERSION: u32 = 1;

impl Manifest {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path).with_context(|| format!("reading manifest at {}", path.display()))?;
        let m: Manifest = serde_json::from_str(&raw).with_context(|| format!("parsing manifest at {}", path.display()))?;

        if m.schema_version != SCHEMA_VERSION {
            return Err(anyhow!(
                "manifest at {} is schema_version {}, but this CLI expects {}",
                path.display(),
                m.schema_version,
                SCHEMA_VERSION
            ));
        }

        Ok(m)
    }

    pub fn crate_ident(&self) -> String {
        self.crate_name.replace('-', "_")
    }

    pub fn paths_with_leaf(&self, leaf: &str) -> Vec<(&String, &String)> {
        self.paths.iter().filter(|(path, _)| path.split("::").last() == Some(leaf)).collect()
    }

    pub fn closure(&self, feature: &str) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        let mut stack = vec![feature.to_string()];

        while let Some(f) = stack.pop() {
            if !out.insert(f.clone()) {
                continue;
            }

            if let Some(deps) = self.dependencies.get(&f) {
                for d in deps {
                    if !out.contains(d) {
                        stack.push(d.clone());
                    }
                }
            }
        }

        out
    }
}

use std::collections::BTreeSet;
use std::io::{self, Write};

use anyhow::{anyhow, Context, Result};

use crate::manifest::Manifest;
use crate::{cargo_check, cover, imports, scan, toml_writer, workspace};

pub struct ScanResult {
    pub layout: workspace::Layout,
    pub manifest: Manifest,
    pub referenced_paths: BTreeSet<String>,
    pub referenced_features: BTreeSet<String>,
}

pub fn scan_workspace(use_cargo_check_fallback: bool) -> Result<ScanResult> {
    try_scan_workspace(use_cargo_check_fallback)?
        .ok_or_else(|| anyhow!("no workspace member depends on `engage-il2cpp`. Run from within a project that uses it"))
}

pub fn try_scan_workspace(use_cargo_check_fallback: bool) -> Result<Option<ScanResult>> {
    let cwd = std::env::current_dir()?;

    let Some(layout) = workspace::try_detect(&cwd)? else {
        return Ok(None);
    };

    scan_layout(&layout, use_cargo_check_fallback).map(Some)
}

pub fn scan_all_layouts(use_cargo_check_fallback: bool) -> Result<Vec<ScanResult>> {
    let cwd = std::env::current_dir()?;
    let layouts = workspace::detect_all(&cwd)?;
    let mut out = Vec::with_capacity(layouts.len());

    for layout in layouts {
        out.push(scan_layout(&layout, use_cargo_check_fallback)?);
    }

    Ok(out)
}

fn scan_layout(layout: &workspace::Layout, use_cargo_check_fallback: bool) -> Result<ScanResult> {
    let manifest = Manifest::load(&layout.bindings_root.join("features.json"))?;

    let listed = toml_writer::read_features(&layout.features_manifest).unwrap_or_default();
    let mut enabled: BTreeSet<String> = BTreeSet::new();
    for f in &listed {
        enabled.extend(manifest.closure(f));
    }

    let mut referenced_paths = BTreeSet::new();

    for root in &layout.scan_roots {
        let found = scan::collect_paths(root, &manifest, &[], &[], &enabled)?;
        referenced_paths.extend(found);
    }

    let bindings_src = layout.bindings_root.join("src");

    if bindings_src.exists() {
        let found = scan::collect_paths(
            &bindings_src,
            &manifest,
            &["crate"],
            &["generated"],
            &enabled,
        )?;
        referenced_paths.extend(found);
    }

    if use_cargo_check_fallback {
        match cargo_check::collect_unresolved_paths(&layout.check_manifest, &manifest) {
            Ok(extra) => referenced_paths.extend(extra),
            Err(e) => eprintln!("warning: cargo-check fallback skipped: {e}"),
        }
    }

    let referenced_features = scan::paths_to_features(&referenced_paths, &manifest);

    Ok(ScanResult {
        layout: layout.clone(),
        manifest,
        referenced_paths,
        referenced_features,
    })
}

pub fn check() -> Result<()> {
    let scans = scan_all_layouts(true)?;

    if scans.is_empty() {
        return Err(anyhow!("no workspace member depends on `engage-il2cpp`. Run from within a project that uses it"));
    }

    let multi_crate = scans.len() > 1;
    let mut total_missing = 0usize;

    for scanned in &scans {
        let listed = toml_writer::read_features(&scanned.layout.features_manifest)?;
        let required = cover::minimum_cover(&scanned.referenced_features, &scanned.manifest);
        let (missing, stale) = diff_features(&listed, &required, &scanned.manifest);

        if multi_crate {
            println!("=== {}", scanned.layout.features_manifest.display());
        }

        let count = scanned.referenced_paths.len();
        println!("scanned {count} engage-il2cpp path reference{}", plural(count));

        if missing.is_empty() && stale.is_empty() {
            println!("Cargo.toml is in sync");

            if multi_crate {
                println!();
            }

            continue;
        }

        if !missing.is_empty() {
            println!("\nmissing ({}):", missing.len());

            for f in &missing {
                println!("  + {f}");
            }

            total_missing += missing.len();
        }

        if !stale.is_empty() {
            println!("\nstale ({}):", stale.len());

            for f in &stale {
                println!("  - {f}");
            }
        }

        if multi_crate {
            println!();
        }
    }

    if total_missing > 0 {
        std::process::exit(1);
    }

    Ok(())
}

pub fn apply(yes: bool) -> Result<()> {
    let scans = scan_all_layouts(true)?;

    if scans.is_empty() {
        return Err(anyhow!("no workspace member depends on `engage-il2cpp`. Run from within a project that uses it"));
    }

    let mut pending: Vec<(workspace::Layout, BTreeSet<String>, BTreeSet<String>)> = Vec::new();

    for scanned in scans {
        let listed = toml_writer::read_features(&scanned.layout.features_manifest)?;
        let required = cover::minimum_cover(&scanned.referenced_features, &scanned.manifest);
        let (missing, _) = diff_features(&listed, &required, &scanned.manifest);

        if !missing.is_empty() {
            pending.push((scanned.layout, listed, missing));
        }
    }

    if pending.is_empty() {
        println!("nothing to do, every Cargo.toml already covers every referenced path");
        return Ok(());
    }

    for (layout, _, missing) in &pending {
        println!("Will add {} feature{} to {}:", missing.len(), plural(missing.len()), layout.features_manifest.display());

        for f in missing {
            println!("  + {f}");
        }
    }

    if !yes && !confirm("Apply?")? {
        println!("aborted");
        return Ok(());
    }

    for (layout, listed, missing) in pending {
        let mut updated = listed.clone();
        updated.extend(missing);

        toml_writer::write_features(&layout.features_manifest, &updated)?;
        println!("updated {}", layout.features_manifest.display());
    }

    Ok(())
}

pub fn explain(path: &str) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let layout = workspace::detect(&cwd)?;
    let manifest = Manifest::load(&layout.bindings_root.join("features.json"))?;

    let normalized = path.replace("r#", "");

    let Some(feature) = manifest.paths.get(&normalized) else {
        return Err(anyhow!(
            "no path `{path}` in features.json. The type may be from unity2 or filtered out of the bindings"
        ));
    };

    println!("{path}");
    println!("  feature: {feature}");

    let closure = manifest.closure(feature);
    let mut deps: Vec<&String> = closure.iter().filter(|f| f.as_str() != feature).collect();
    deps.sort();

    if !deps.is_empty() {
        println!("  enabling `{feature}` transitively activates {} features:", deps.len());

        for d in deps.iter().take(20) {
            println!("    - {d}");
        }

        if deps.len() > 20 {
            println!("    ... and {} more", deps.len() - 20);
        }
    }

    Ok(())
}

pub fn prune(yes: bool) -> Result<()> {
    let scans = scan_all_layouts(true)?;

    if scans.is_empty() {
        return Err(anyhow!("no workspace member depends on `engage-il2cpp`. Run from within a project that uses it"));
    }

    let mut dead_imports: Vec<imports::DeadImport> = Vec::new();
    let mut pending_features: Vec<(workspace::Layout, BTreeSet<String>, BTreeSet<String>)> = Vec::new();

    for scanned in &scans {
        let dead = imports::find_dead_imports(&scanned.layout.scan_roots, &scanned.manifest)?;

        let dead_paths: BTreeSet<String> = dead.iter().map(|d| d.path.clone()).collect();
        let effective_paths: BTreeSet<String> = scanned.referenced_paths.difference(&dead_paths).cloned().collect();
        let effective_features = scan::paths_to_features(&effective_paths, &scanned.manifest);

        let listed = toml_writer::read_features(&scanned.layout.features_manifest)?;
        let required = cover::minimum_cover(&effective_features, &scanned.manifest);
        let (_, stale) = diff_features(&listed, &required, &scanned.manifest);

        dead_imports.extend(dead);

        if !stale.is_empty() {
            pending_features.push((scanned.layout.clone(), listed, stale));
        }
    }

    if dead_imports.is_empty() && pending_features.is_empty() {
        println!("nothing to prune, every listed feature covers something referenced");
        return Ok(());
    }

    if !dead_imports.is_empty() {
        println!("Will remove {} unused use statement{}:", dead_imports.len(), plural(dead_imports.len()));

        for d in &dead_imports {
            println!("  - {} ({}:{})", d.ident, d.file.display(), d.start_line + 1);
        }
    }

    for (layout, _, stale) in &pending_features {
        if !dead_imports.is_empty() {
            println!();
        }

        println!("Will remove {} stale feature{} from {}:", stale.len(), plural(stale.len()), layout.features_manifest.display());

        for f in stale {
            println!("  - {f}");
        }
    }

    if !yes && !confirm("Apply?")? {
        println!("aborted");
        return Ok(());
    }

    let mut by_file: std::collections::BTreeMap<std::path::PathBuf, Vec<(u32, u32)>> = std::collections::BTreeMap::new();

    for d in &dead_imports {
        by_file.entry(d.file.clone()).or_default().push((d.start_line, d.end_line));
    }

    for (file, ranges) in by_file {
        imports::remove_lines(&file, &ranges)?;
        println!("cleaned {}", file.display());
    }

    for (layout, listed, stale) in pending_features {
        let mut updated = listed.clone();

        for f in &stale {
            updated.remove(f);
        }

        toml_writer::write_features(&layout.features_manifest, &updated)?;
        println!("updated {}", layout.features_manifest.display());
    }

    Ok(())
}

pub fn add_feature(name: &str) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let layouts = workspace::detect_all(&cwd)?;

    if layouts.is_empty() {
        return Err(anyhow!("no workspace member depends on `engage-il2cpp`. Run from within a project that uses it"));
    }

    let manifest_path = layouts[0].bindings_root.join("features.json");
    let manifest = Manifest::load(&manifest_path)?;

    if !manifest.dependencies.contains_key(name) {
        return Err(anyhow!("`{name}` is not a known feature in {}", manifest_path.display()));
    }

    let feature_provides_path = manifest.paths.values().any(|f| f == name);

    if !feature_provides_path && !name.contains("-") {
        return Err(anyhow!("`{name}` does not match any known feature shape"));
    }

    for layout in layouts {
        let listed = toml_writer::read_features(&layout.features_manifest)?;

        if listed.contains(name) {
            continue;
        }

        let mut updated = listed;
        updated.insert(name.to_string());

        toml_writer::write_features(&layout.features_manifest, &updated)?;
        println!("added `{name}` to {}", layout.features_manifest.display());

        return Ok(());
    }

    println!("`{name}` already listed in every Cargo.toml");

    Ok(())
}

pub fn diff_features(listed: &BTreeSet<String>, required: &BTreeSet<String>, manifest: &Manifest) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut covered: BTreeSet<String> = BTreeSet::new();
    let mut stale: BTreeSet<String> = BTreeSet::new();

    for f in listed {
        let closure = manifest.closure(f);

        if closure.is_disjoint(required) {
            stale.insert(f.clone());
        } else {
            covered.extend(closure.intersection(required).cloned());
        }
    }

    let missing: BTreeSet<String> = required.difference(&covered).cloned().collect();

    (missing, stale)
}

fn confirm(prompt: &str) -> Result<bool> {
    print!("{prompt} [y/N] ");
    io::stdout().flush().context("flushing stdout")?;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).context("reading stdin")?;

    Ok(matches!(buf.trim().to_ascii_lowercase().as_str(), "y" | "yes"))
}

fn plural(n: usize) -> &'static str {
    if n == 1 {
        ""
    } else {
        "s"
    }
}

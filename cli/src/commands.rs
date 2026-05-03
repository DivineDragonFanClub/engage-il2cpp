use std::collections::BTreeSet;
use std::io::{self, Write};

use anyhow::{anyhow, Context, Result};

use crate::manifest::Manifest;
use crate::{cargo_check, cover, scan, toml_writer, workspace};

struct ScanResult {
    layout: workspace::Layout,
    manifest: Manifest,
    referenced_paths: BTreeSet<String>,
    referenced_features: BTreeSet<String>,
}

fn scan_workspace(use_cargo_check_fallback: bool) -> Result<ScanResult> {
    let cwd = std::env::current_dir()?;
    let layout = workspace::detect(&cwd)?;
    let manifest = Manifest::load(&layout.bindings_root.join("features.json"))?;

    let mut referenced_paths = BTreeSet::new();

    for root in &layout.scan_roots {
        let found = scan::collect_paths(root, &manifest, &[], &[])?;
        referenced_paths.extend(found);
    }

    let bindings_src = layout.bindings_root.join("src");

    if bindings_src.exists() {
        let found = scan::collect_paths(&bindings_src, &manifest, &["crate"], &["generated"])?;
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
        layout,
        manifest,
        referenced_paths,
        referenced_features,
    })
}

pub fn check() -> Result<()> {
    let scanned = scan_workspace(true)?;
    let listed = toml_writer::read_features(&scanned.layout.features_manifest)?;
    let required = cover::minimum_cover(&scanned.referenced_features, &scanned.manifest);

    let (missing, stale) = diff_features(&listed, &required, &scanned.manifest);

    let count = scanned.referenced_paths.len();
    println!("scanned {count} engage-il2cpp path reference{}", plural(count));

    if missing.is_empty() && stale.is_empty() {
        println!("Cargo.toml is in sync");
        return Ok(());
    }

    if !missing.is_empty() {
        println!("\nmissing ({}):", missing.len());
        for f in &missing {
            println!("  + {f}");
        }
    }

    if !stale.is_empty() {
        println!("\nstale ({}):", stale.len());
        for f in &stale {
            println!("  - {f}");
        }
    }

    if !missing.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}

pub fn apply(yes: bool) -> Result<()> {
    let scanned = scan_workspace(true)?;
    let listed = toml_writer::read_features(&scanned.layout.features_manifest)?;
    let required = cover::minimum_cover(&scanned.referenced_features, &scanned.manifest);

    let (missing, _) = diff_features(&listed, &required, &scanned.manifest);

    if missing.is_empty() {
        println!("nothing to do, Cargo.toml already covers every referenced path");
        return Ok(());
    }

    let target = scanned.layout.features_manifest.display();
    println!("Will add {} feature{} to {target}:", missing.len(), plural(missing.len()));

    for f in &missing {
        println!("  + {f}");
    }

    if !yes && !confirm("Apply?")? {
        println!("aborted");
        return Ok(());
    }

    let mut updated = listed.clone();
    updated.extend(missing);

    toml_writer::write_features(&scanned.layout.features_manifest, &updated)?;
    println!("updated {target}");

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
    let scanned = scan_workspace(true)?;
    let listed = toml_writer::read_features(&scanned.layout.features_manifest)?;
    let required = cover::minimum_cover(&scanned.referenced_features, &scanned.manifest);

    let (_, stale) = diff_features(&listed, &required, &scanned.manifest);

    if stale.is_empty() {
        println!("nothing to prune, every listed feature covers something referenced");
        return Ok(());
    }

    let target = scanned.layout.features_manifest.display();
    println!("Will remove {} stale feature{} from {target}:", stale.len(), plural(stale.len()));

    for f in &stale {
        println!("  - {f}");
    }

    if !yes && !confirm("Apply?")? {
        println!("aborted");
        return Ok(());
    }

    let mut updated = listed.clone();

    for f in &stale {
        updated.remove(f);
    }

    toml_writer::write_features(&scanned.layout.features_manifest, &updated)?;
    println!("updated {target}");

    Ok(())
}

fn diff_features(listed: &BTreeSet<String>, required: &BTreeSet<String>, manifest: &Manifest) -> (BTreeSet<String>, BTreeSet<String>) {
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

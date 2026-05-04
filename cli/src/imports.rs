use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Item, UseTree};
use walkdir::WalkDir;

use crate::manifest::Manifest;

pub struct DeadImport {
    pub file: PathBuf,
    pub start_line: u32,
    pub end_line: u32,
    pub ident: String,
    pub path: String,
    pub feature: String,
}

pub fn find_dead_imports(roots: &[PathBuf], manifest: &Manifest) -> Result<Vec<DeadImport>> {
    let mut out = Vec::new();

    for root in roots {
        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().and_then(|s| s.to_str()) != Some("rs") {
                continue;
            }

            let raw = match std::fs::read_to_string(entry.path()) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let dead = find_dead_imports_in_text(&raw, manifest);

            for d in dead {
                out.push(DeadImport {
                    file: entry.path().to_path_buf(),
                    start_line: d.start_line,
                    end_line: d.end_line,
                    ident: d.ident,
                    path: d.path,
                    feature: d.feature,
                });
            }
        }
    }

    Ok(out)
}

struct DeadImportInFile {
    start_line: u32,
    end_line: u32,
    ident: String,
    path: String,
    feature: String,
}

fn find_dead_imports_in_text(text: &str, manifest: &Manifest) -> Vec<DeadImportInFile> {
    let file = match syn::parse_file(text) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let referenced_in_body = collect_body_refs(&file);
    let mut out = Vec::new();

    for item in &file.items {
        let Item::Use(use_item) = item else { continue };

        let Some(single) = single_engage_use(&use_item.tree, manifest) else { continue };

        if referenced_in_body.contains(&single.local_name) {
            continue;
        }

        let span = use_item.span();
        let start_line = span.start().line.saturating_sub(1) as u32;
        let end_line = span.end().line.saturating_sub(1) as u32;

        out.push(DeadImportInFile {
            start_line,
            end_line,
            ident: single.local_name,
            path: single.full_path,
            feature: single.feature,
        });
    }

    out
}

struct SingleEngageUse {
    local_name: String,
    full_path: String,
    feature: String,
}

fn single_engage_use(tree: &UseTree, manifest: &Manifest) -> Option<SingleEngageUse> {
    let mut segments: Vec<String> = Vec::new();
    let mut local_name: Option<String> = None;

    walk_single(tree, &mut segments, &mut local_name)?;

    let path = segments.join("::");
    let feature = manifest.paths.get(&path)?.clone();

    Some(SingleEngageUse {
        local_name: local_name?,
        full_path: path,
        feature,
    })
}

fn walk_single(tree: &UseTree, segments: &mut Vec<String>, local_name: &mut Option<String>) -> Option<()> {
    match tree {
        UseTree::Path(p) => {
            segments.push(strip_raw(&p.ident.to_string()));
            walk_single(&p.tree, segments, local_name)
        },
        UseTree::Name(n) => {
            segments.push(strip_raw(&n.ident.to_string()));
            *local_name = Some(strip_raw(&n.ident.to_string()));
            Some(())
        },
        UseTree::Rename(r) => {
            segments.push(strip_raw(&r.ident.to_string()));
            *local_name = Some(strip_raw(&r.rename.to_string()));
            Some(())
        },
        UseTree::Group(_) | UseTree::Glob(_) => None,
    }
}

fn strip_raw(s: &str) -> String {
    s.strip_prefix("r#").unwrap_or(s).to_string()
}

fn collect_body_refs(file: &syn::File) -> BTreeSet<String> {
    let mut visitor = BodyRefVisitor { out: BTreeSet::new() };

    for item in &file.items {
        if matches!(item, Item::Use(_)) {
            continue;
        }

        visitor.visit_item(item);
    }

    visitor.out
}

struct BodyRefVisitor {
    out: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for BodyRefVisitor {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        if let Some(first) = path.segments.first() {
            self.out.insert(strip_raw(&first.ident.to_string()));
        }

        syn::visit::visit_path(self, path);
    }

    fn visit_ident(&mut self, ident: &'ast proc_macro2::Ident) {
        self.out.insert(strip_raw(&ident.to_string()));
    }
}

pub fn remove_lines(file: &Path, line_ranges: &[(u32, u32)]) -> Result<()> {
    let raw = std::fs::read_to_string(file).with_context(|| format!("reading {}", file.display()))?;
    let mut lines: Vec<&str> = raw.split('\n').collect();
    let trailing_newline = raw.ends_with('\n');

    let mut to_remove: BTreeSet<usize> = BTreeSet::new();

    for &(start, end) in line_ranges {
        for i in start..=end {
            to_remove.insert(i as usize);
        }
    }

    let kept: Vec<&str> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| if to_remove.contains(&i) { None } else { Some(*line) })
        .collect();
    lines.clear();

    let mut joined = kept.join("\n");

    if trailing_newline && !joined.ends_with('\n') {
        joined.push('\n');
    }

    std::fs::write(file, joined).with_context(|| format!("writing {}", file.display()))?;

    Ok(())
}

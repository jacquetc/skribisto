// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The fence around the generator: everything it writes has been seen in a real
//! project.
//!
//! `observed_shape.json` is a census of the local corpus of real Scrivener
//! projects (`scripts/scrivener_corpus/observed_shape.py`): per generation and
//! platform, every element path and attribute name of the manifests and of
//! Scrivener's own XML files, the shape of every file path, the lock file's keys.
//! Structure only, no content. This test builds every fixture and checks each
//! element, attribute and path it writes against that census, so a generator
//! that invents a format detail fails here instead of teaching the reader a
//! format nobody writes.
//!
//! A shape the corpus does not hold must be listed in [`UNVERIFIED`] with the
//! question it waits on, and only for inputs the importer must survive and
//! report, never for a shape a test asserts as mapped. A path the census holds
//! only masked (`<NAME>`: a name a writer or a sync service chose) counts as not
//! held: it matches any invented name of that extension.

use std::collections::BTreeSet;

use scrivener_fixtures::{Backup, Files, Generation, Platform, Spec, TemplateEncoding, template};

const OBSERVED: &str = include_str!("../observed_shape.json");

/// Shapes the generator writes that the corpus does not show, each with why it
/// is written anyway.
const UNVERIFIED: &[(&str, &str)] = &[(
    "file 2.0 Settings/Compile Formats/<NAME>.scrformat",
    "a compile format's file is named after the format, a name the writer chooses, so the \
     census masks every one; the importer only reports the styles such a format leaves out",
)];

struct Census {
    json: serde_json::Value,
}

impl Census {
    fn load() -> Self {
        match serde_json::from_str(OBSERVED) {
            Ok(json) => Census { json },
            Err(e) => panic!("observed_shape.json does not parse: {e}"),
        }
    }

    /// Every manifest path seen for generation `g`, on any platform.
    fn manifest_union(&self, g: &str) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        if let Some(map) = self.json["manifests"].as_object() {
            for (key, paths) in map {
                if key.split('/').next() == Some(g) {
                    out.extend(
                        paths
                            .as_array()
                            .into_iter()
                            .flatten()
                            .filter_map(|p| p.as_str().map(str::to_string)),
                    );
                }
            }
        }
        out
    }

    fn set(&self, pointer: &str) -> BTreeSet<String> {
        self.json
            .pointer(pointer)
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|s| s.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default()
    }
}

fn is_uuid(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 36
        && b.iter().enumerate().all(|(i, c)| match i {
            8 | 13 | 18 | 23 => *c == b'-',
            _ => c.is_ascii_hexdigit(),
        })
}

/// The census's masking of a name a writer or a machine chose.
fn masked(part: &str) -> String {
    let safe = |s: &str| {
        !s.is_empty()
            && s.len() <= 40
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || "._<>+~#[]-".contains(c))
    };
    if safe(part) {
        return part.to_string();
    }
    match part.rfind('.') {
        Some(dot) if dot > 0 && safe(&part[dot..]) => format!("<NAME>{}", &part[dot..]),
        _ => "<NAME>".to_string(),
    }
}

/// A path's shape, as the census writes it.
fn shape(rel: &str) -> String {
    let parts: Vec<&str> = rel.split('/').collect();
    let mut out = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        let top = parts[0];
        let last = i == parts.len() - 1;
        let mut p = if is_uuid(part) {
            "<UUID>".to_string()
        } else if part.len() > 36 && is_uuid(&part[..36]) {
            format!("<UUID>{}", &part[36..])
        } else {
            part.to_string()
        };
        if last && is_stamp(part) {
            p = "<STAMP>.rtf".to_string();
        } else if i == 0 && part.to_lowercase().ends_with(".scrivx") {
            p = "<STEM>.scrivx".to_string();
        } else if top == "Icons" && i > 0 {
            p = "<NAME>".to_string();
        } else if top == "Settings" && i == 1 && *part == "Compile Formats" {
            // Scrivener's own folder name, which the census keeps as written.
            p = part.to_string();
        } else if top == "Settings" && i == 2 && parts[1] == "Compile Formats" && last {
            // A compile format's file carries the format's name, the writer's choice.
            p = match part.rfind('.') {
                Some(dot) => format!("<NAME>{}", &part[dot..]),
                None => "<NAME>".to_string(),
            };
        } else {
            let digits = p.bytes().take_while(u8::is_ascii_digit).count();
            if digits > 0 {
                p = format!("<N>{}", &p[digits..]);
            }
            if let Some(rest) = p.strip_prefix("Note-")
                && rest.bytes().next().is_some_and(|b| b.is_ascii_digit())
            {
                let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
                p = format!("Note-<N>{}", &rest[digits..]);
            }
            for stem in ["content", "card-image", "<N>", "<N>_icImage"] {
                if let Some(ext) = p.strip_prefix(&format!("{stem}."))
                    && ext.chars().all(|c| c.is_ascii_alphanumeric())
                    && !["rtf", "comments", "styles", "links"]
                        .contains(&ext.to_lowercase().as_str())
                {
                    p = format!("{stem}.<EXT>");
                }
            }
            p = masked(&p);
        }
        out.push(p);
    }
    out.join("/")
}

fn is_stamp(name: &str) -> bool {
    let b = name.as_bytes();
    b.len() == 28
        && name.ends_with(".rtf")
        && b[..24].iter().enumerate().all(|(i, c)| match i {
            4 | 7 | 10 | 13 | 16 => *c == b'-',
            19 => *c == b'-' || *c == b'+',
            _ => c.is_ascii_digit(),
        })
}

/// Element and attribute paths of an XML document, nesting of binder items and
/// keywords collapsed as the census collapses it.
fn xml_paths(text: &str) -> BTreeSet<String> {
    let options = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..roxmltree::ParsingOptions::default()
    };
    let doc = match roxmltree::Document::parse_with_options(text, options) {
        Ok(doc) => doc,
        Err(e) => panic!("a fixture's XML does not parse: {e}"),
    };
    let mut found = BTreeSet::new();
    let mut stack: Vec<(roxmltree::Node<'_, '_>, Vec<String>)> =
        vec![(doc.root_element(), Vec::new())];
    while let Some((node, mut path)) = stack.pop() {
        path.push(node.tag_name().name().to_string());
        let key = collapse(&path);
        for attr in node.attributes() {
            found.insert(format!("{key}/@{}", attr.name()));
        }
        found.insert(key);
        for child in node.children().filter(|c| c.is_element()) {
            stack.push((child, path.clone()));
        }
    }
    found
}

fn collapse(stack: &[String]) -> String {
    let mut out: Vec<String> = stack.to_vec();
    for rec in ["BinderItem", "Keyword"] {
        let first = out.iter().position(|s| s == rec);
        let last = out.iter().rposition(|s| s == rec);
        if let (Some(first), Some(last)) = (first, last)
            && first != last
        {
            out = out[..first]
                .iter()
                .chain(out[last..].iter())
                .cloned()
                .collect();
        }
    }
    out.join("/")
}

fn generation_key(g: Generation) -> &'static str {
    match g {
        Generation::Format10 => "1.0",
        Generation::Format15 => "1.5",
        Generation::Format20 => "2.0",
        Generation::IosStub => "ios",
        Generation::ScrivenerOne => "scrivener1",
    }
}

fn platform_key(g: Generation, p: Platform) -> &'static str {
    match (g, p) {
        (Generation::IosStub, _) | (_, Platform::Ios) => "ios",
        (_, Platform::Mac) => "mac",
        (Generation::Format20, _) | (_, Platform::Windows) => "win",
        (_, Platform::Linux) => "lin",
    }
}

/// Which sidecar census a member belongs to, by its name.
fn sidecar_kind(rel: &str) -> Option<&'static str> {
    let name = rel.rsplit('/').next().unwrap_or_default();
    let number = |suffix: &str| {
        name.strip_suffix(suffix)
            .is_some_and(|stem| !stem.is_empty() && stem.bytes().all(|b| b.is_ascii_digit()))
    };
    Some(match name {
        "content.comments" => "content.comments",
        "search.indexes" => "search.indexes",
        "templateinfo.xml" => "templateinfo.xml",
        "compile.xml" => "compile.xml",
        "styles.xml" => "styles.xml",
        "writing.history" => "writing.history",
        "snapshot.indexes" => "snapshot.indexes",
        "index.xml" if rel.contains("ProjectNotes") => "ProjectNotes/index.xml",
        "index.xml" if rel.contains(".snapshots/") => "snapshots/index.xml",
        _ if number(".comments") => "N.comments",
        _ if number(".links") => "N.links",
        _ => return None,
    })
}

/// Every problem one built project has against the census.
fn problems(census: &Census, spec: &Spec, files: &Files) -> Vec<String> {
    let g = generation_key(spec.generation);
    let manifest_key = format!("{g}/{}", platform_key(spec.generation, spec.platform));
    let allowed_files = census.set(&format!("/files/{g}"));
    let unverified: BTreeSet<&str> = UNVERIFIED.iter().map(|(what, _)| *what).collect();
    let mut out = Vec::new();
    for (rel, bytes) in files.iter() {
        let s = shape(rel);
        // A masked name matches whatever some real project held under it (a sync
        // copy, a writer's own document), so it says nothing about what Scrivener
        // writes: a fixture file matching only that is listed as unverified or
        // refused.
        let known = allowed_files.contains(&s) && !s.contains("<NAME>");
        if !known && !unverified.contains(format!("file {g} {s}").as_str()) {
            out.push(format!("file {g} {s}"));
        }
        // Which census this XML file is held to. A manifest's elements are held
        // to everything its version holds on any platform (the corpus is thin for
        // some platforms of format 1.0); the attributes of its root, which say
        // which platform wrote it, to that platform alone.
        let (allowed, strict_root) = if rel.to_lowercase().ends_with(".scrivx")
            && !rel.contains('/')
        {
            if spec.generation == Generation::IosStub {
                (census.set("/manifests/ios~1ios"), None)
            } else {
                let exact = census.set(&format!("/manifests/{}", manifest_key.replace('/', "~1")));
                (census.manifest_union(g), Some(exact))
            }
        } else if rel == "Mobile/binder.mob" {
            (census.set("/manifests/binder.mob"), None)
        } else if let Some(kind) = sidecar_kind(rel) {
            (
                census.set(&format!("/sidecars/{}/{g}", kind.replace('/', "~1"))),
                None,
            )
        } else {
            continue;
        };
        let text = String::from_utf8_lossy(bytes);
        for path in xml_paths(&text) {
            let root_attribute = path.starts_with("ScrivenerProject/@");
            let known = match (&strict_root, root_attribute) {
                (Some(exact), true) => exact.contains(&path),
                _ => allowed.contains(&path),
            };
            let what = format!("xml {rel_kind} {path}", rel_kind = shape(rel));
            if !known && !unverified.contains(what.as_str()) {
                out.push(format!("{what} ({manifest_key})"));
            }
        }
    }
    if let Some(lock) = files.get("Files/user.lock") {
        let keys = census.set("/lock_keys");
        for line in String::from_utf8_lossy(lock).lines() {
            if let Some((key, _)) = line.split_once('=')
                && !keys.contains(key.trim())
            {
                out.push(format!("lock key {key}"));
            }
        }
    }
    out
}

fn every_spec() -> Vec<Spec> {
    let mut specs = Vec::new();
    for (g, platforms) in [
        (Generation::Format20, vec![Platform::Mac, Platform::Windows]),
        (
            Generation::Format15,
            vec![Platform::Mac, Platform::Windows, Platform::Linux],
        ),
        (
            Generation::Format10,
            vec![Platform::Mac, Platform::Windows, Platform::Linux],
        ),
        (Generation::IosStub, vec![Platform::Ios]),
        (Generation::ScrivenerOne, vec![Platform::Mac]),
    ] {
        for p in platforms {
            let mut spec = Spec::novel(g, p);
            specs.push(spec.clone());
            // The same project with every piece of metadata the importer maps.
            if g != Generation::ScrivenerOne {
                specs.push(spec.clone().with_metadata());
                // And every piece the second half of the mapping reads.
                specs.push(spec.clone().with_metadata().with_s2());
            }
            // The same project with a lock and a placeholder spare binder.
            spec.lock_host = Some("Test-Host".to_string());
            spec.backup = Backup::Placeholder;
            specs.push(spec);
        }
    }
    specs
}

#[test]
fn every_fixture_writes_only_shapes_real_projects_hold() {
    let census = Census::load();
    let mut all = Vec::new();
    for spec in every_spec() {
        let files = spec.build();
        for problem in problems(&census, &spec, &files) {
            all.push(format!(
                "{:?}/{:?}: {problem}",
                spec.generation, spec.platform
            ));
        }
    }
    all.sort();
    all.dedup();
    assert!(
        all.is_empty(),
        "shapes no real project holds:\n{}",
        all.join("\n")
    );
}

#[test]
fn a_template_wrapper_has_the_shape_of_a_real_one() {
    let census = Census::load();
    let allowed = census.set("/template");
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let zip = spec.build().zip(Some(&spec.folder())).unwrap_or_default();
    for encoding in [TemplateEncoding::Base64, TemplateEncoding::Hex] {
        let wrapper = template(&zip, encoding, "Starter");
        let paths = xml_paths(&String::from_utf8_lossy(&wrapper));
        let unknown: Vec<&String> = paths.iter().filter(|p| !allowed.contains(*p)).collect();
        assert!(unknown.is_empty(), "{encoding:?}: {unknown:?}");
    }
}

/// The census stays structure only: no title, no text, no value.
#[test]
fn the_census_holds_no_content() {
    let census = Census::load();
    for (_, files) in census.json["files"].as_object().into_iter().flatten() {
        for shape in files
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|s| s.as_str())
        {
            assert!(
                !shape.contains(' ') || shape.starts_with("Settings/Compile "),
                "{shape}"
            );
        }
    }
    assert!(UNVERIFIED.iter().all(|(_, why)| !why.is_empty()));
}

/// The census masks a name a writer or a sync service chose (`<NAME>`), so a
/// masked shape says only that some real project held some such file: sync
/// debris, a writer's own document. A fixture file matching nothing better is
/// invented, and the fence has to say so rather than wave it through.
#[test]
fn the_fence_catches_a_file_real_projects_hold_only_as_debris() {
    let census = Census::load();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let mut files = spec.build();
    files.put("Files/Invented Sidecar.xml", "<Invented/>");
    let found = problems(&census, &spec, &files);
    assert!(
        found.iter().any(|p| p.contains("Files/<NAME>.xml")),
        "{found:?}"
    );
}

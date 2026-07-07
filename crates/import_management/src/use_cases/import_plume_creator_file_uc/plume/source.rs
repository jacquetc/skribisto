//! Reading a `.plume` project's raw members, from either container Plume ever
//! used:
//!  - the modern **single-file zip** (`tree`/`info`/`attendance` + `text/`,
//!    `attend/`, `dicts/` members), and
//!  - the pre-0.3 **"old system"**: a bare directory of loose `*.plume` (the tree,
//!    root `<plume>`), `*.attend`, `*.prjinfo` files plus `text/`/`attend/`/`dicts/`
//!    subfolders, opened via the loose `.plume` file.
//!
//! Detection matches Plume's own `Utils::isProjectFromOldSystem = !ZipChecker::isZip`
//! (`utils.cpp` / `zipchecker.cpp`): if the path parses as a zip it is modern,
//! otherwise it is the old loose-file layout. There is no SQLite variant — Plume
//! never used one.
//!
//! A `.plume` is tiny (tens of KB), so every member is read into memory up front;
//! callers then look prose/synopsis/notes/attendance blobs up by their numeric id.

use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};

/// All of a `.plume` project's text members, read into memory.
pub struct PlumeSource {
    pub tree_xml: String,
    pub attendance_xml: Option<String>,
    pub info_xml: Option<String>,
    pub dict: Option<String>,
    /// Prose / synopsis / note / attendance HTML blobs keyed by their member
    /// path, e.g. `"text/T3.html"`, `"attend/A1.html"`.
    html: HashMap<String, String>,
}

const DICT_MEMBER: &str = "dicts/userDict.dict_plume";

impl PlumeSource {
    /// Open the project at `source_path` (a `.plume`/`.plume_backup` zip, or the
    /// loose `.plume` tree file of an old-system project).
    pub fn open(source_path: &str) -> Result<Self> {
        let bytes = fs::read(source_path)
            .with_context(|| format!("reading Plume project '{source_path}'"))?;

        // Modern zip? (Plume's own zip-vs-loose discriminator.)
        match zip::ZipArchive::new(std::io::Cursor::new(&bytes[..])) {
            Ok(archive) => Self::from_zip(archive),
            Err(_) => Self::from_old_system(Path::new(source_path), &bytes),
        }
    }

    fn from_zip(mut archive: zip::ZipArchive<std::io::Cursor<&[u8]>>) -> Result<Self> {
        let mut tree_xml = None;
        let mut attendance_xml = None;
        let mut info_xml = None;
        let mut dict = None;
        let mut html = HashMap::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            if !file.is_file() {
                continue;
            }
            let name = file.name().trim_start_matches('/').to_string();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            let text = String::from_utf8_lossy(&buf).into_owned();
            match name.as_str() {
                "tree" => tree_xml = Some(text),
                "attendance" => attendance_xml = Some(text),
                "info" => info_xml = Some(text),
                DICT_MEMBER => dict = Some(text),
                _ if name.starts_with("text/") || name.starts_with("attend/") => {
                    html.insert(name, text);
                }
                _ => {}
            }
        }

        let tree_xml = tree_xml.ok_or_else(|| {
            anyhow!("this file is a zip but has no `tree` member — not a Plume Creator project")
        })?;
        Ok(Self {
            tree_xml,
            attendance_xml,
            info_xml,
            dict,
            html,
        })
    }

    fn from_old_system(tree_file: &Path, tree_bytes: &[u8]) -> Result<Self> {
        let dir = tree_file
            .parent()
            .ok_or_else(|| anyhow!("Plume project path has no parent directory"))?;
        let tree_xml = String::from_utf8_lossy(tree_bytes).into_owned();

        // Locate the loose sibling metadata files by extension.
        let attendance_xml = first_file_with_ext(dir, "attend")
            .map(|p| read_lossy(&p))
            .transpose()?;
        let info_xml = first_file_with_ext(dir, "prjinfo")
            .map(|p| read_lossy(&p))
            .transpose()?;
        let dict = {
            let p = dir.join(DICT_MEMBER);
            if p.is_file() { Some(read_lossy(&p)?) } else { None }
        };

        // Load the text/ and attend/ subfolders into the html map.
        let mut html = HashMap::new();
        for sub in ["text", "attend"] {
            let subdir = dir.join(sub);
            if !subdir.is_dir() {
                continue;
            }
            for entry in fs::read_dir(&subdir)
                .with_context(|| format!("reading {}", subdir.display()))?
            {
                let path = entry?.path();
                if path.is_file()
                    && let Some(name) = path.file_name().and_then(|n| n.to_str())
                {
                    html.insert(format!("{sub}/{name}"), read_lossy(&path)?);
                }
            }
        }

        if tree_xml.trim().is_empty() {
            bail!("the Plume tree file is empty — not a Plume Creator project");
        }
        Ok(Self {
            tree_xml,
            attendance_xml,
            info_xml,
            dict,
            html,
        })
    }

    /// Manuscript prose for tree node `n` (`text/T{n}.html`).
    pub fn text(&self, n: u32) -> Option<&str> {
        self.html.get(&format!("text/T{n}.html")).map(String::as_str)
    }
    /// Synopsis for tree node `n` (`text/S{n}.html`).
    pub fn synopsis(&self, n: u32) -> Option<&str> {
        self.html.get(&format!("text/S{n}.html")).map(String::as_str)
    }
    /// Note for tree node `n` (`text/N{n}.html`).
    pub fn note(&self, n: u32) -> Option<&str> {
        self.html.get(&format!("text/N{n}.html")).map(String::as_str)
    }
    /// Detail sheet for attendance node `n` (`attend/A{n}.html`).
    pub fn attend_doc(&self, n: u32) -> Option<&str> {
        self.html.get(&format!("attend/A{n}.html")).map(String::as_str)
    }
}

fn read_lossy(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

/// The first file in `dir` whose extension is `ext` (case-insensitive), if any.
fn first_file_with_ext(dir: &Path, ext: &str) -> Option<PathBuf> {
    let mut matches: Vec<PathBuf> = fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.is_file()
                && p.extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|e| e.eq_ignore_ascii_case(ext))
        })
        .collect();
    matches.sort();
    matches.into_iter().next()
}

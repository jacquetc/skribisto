// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A project as a set of files, and the containers it travels in.
//!
//! A builder produces [`Files`]: every member of a project folder by its path
//! inside the folder. A test then changes it (drops a folder, adds a sync copy,
//! renames a manifest) and writes it where the importer will meet it: as a folder,
//! a zip archive with or without a wrapping folder, or a `.scrivtemplate`.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{Cursor, Write};
use std::path::Path;

use base64::Engine;

/// The members of a project folder, by `/`-separated path inside it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Files {
    files: BTreeMap<String, Vec<u8>>,
    /// Folders that exist though empty (Scrivener for Windows leaves item folders
    /// behind when a document is emptied).
    empty_dirs: BTreeSet<String>,
}

impl Files {
    pub fn new() -> Self {
        Files::default()
    }

    /// Add or replace the member at `path`.
    pub fn put(&mut self, path: impl Into<String>, bytes: impl Into<Vec<u8>>) {
        self.files.insert(path.into(), bytes.into());
    }

    /// Add an empty folder.
    pub fn put_dir(&mut self, path: impl Into<String>) {
        self.empty_dirs.insert(path.into());
    }

    pub fn get(&self, path: &str) -> Option<&[u8]> {
        self.files.get(path).map(Vec::as_slice)
    }

    pub fn contains(&self, path: &str) -> bool {
        self.files.contains_key(path)
    }

    /// Remove the member at `path`, returning it.
    pub fn remove(&mut self, path: &str) -> Option<Vec<u8>> {
        self.files.remove(path)
    }

    /// Remove every member at or below the folder `prefix`; how many went.
    pub fn remove_under(&mut self, prefix: &str) -> usize {
        let doomed: Vec<String> = self
            .files
            .keys()
            .filter(|k| k.as_str() == prefix || k.starts_with(&format!("{prefix}/")))
            .cloned()
            .collect();
        for key in &doomed {
            self.files.remove(key);
        }
        self.empty_dirs
            .retain(|d| !(d.as_str() == prefix || d.starts_with(&format!("{prefix}/"))));
        doomed.len()
    }

    /// Move the member at `from` to `to`.
    pub fn rename(&mut self, from: &str, to: &str) -> bool {
        match self.files.remove(from) {
            Some(bytes) => {
                self.files.insert(to.to_string(), bytes);
                true
            }
            None => false,
        }
    }

    /// Every member path, sorted.
    pub fn paths(&self) -> Vec<&str> {
        self.files.keys().map(String::as_str).collect()
    }

    /// Every member and its bytes, sorted by path.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &[u8])> {
        self.files.iter().map(|(k, v)| (k.as_str(), v.as_slice()))
    }

    /// Every member path below the folder `prefix`.
    pub fn paths_under(&self, prefix: &str) -> Vec<String> {
        self.files
            .keys()
            .filter(|k| k.starts_with(&format!("{prefix}/")))
            .cloned()
            .collect()
    }

    /// These members, each under `folder/`.
    pub fn nested(&self, folder: &str) -> Files {
        Files {
            files: self
                .files
                .iter()
                .map(|(k, v)| (format!("{folder}/{k}"), v.clone()))
                .collect(),
            empty_dirs: self
                .empty_dirs
                .iter()
                .map(|d| format!("{folder}/{d}"))
                .collect(),
        }
    }

    /// Add every member of `other`, replacing any of the same path.
    pub fn merge(&mut self, other: &Files) {
        for (k, v) in &other.files {
            self.files.insert(k.clone(), v.clone());
        }
        self.empty_dirs.extend(other.empty_dirs.iter().cloned());
    }

    /// Write the members into the folder `dir`, created as needed.
    pub fn write_to(&self, dir: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(dir)?;
        for folder in &self.empty_dirs {
            std::fs::create_dir_all(dir.join(folder))?;
        }
        for (path, bytes) in &self.files {
            let target = dir.join(path);
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(target, bytes)?;
        }
        Ok(())
    }

    /// The members as a deflated zip archive, each under `wrapper/` when given
    /// (Finder's "Compress" and Scrivener's backups wrap the project folder).
    pub fn zip(&self, wrapper: Option<&str>) -> std::io::Result<Vec<u8>> {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        let name = |path: &str| match wrapper {
            Some(w) => format!("{w}/{path}"),
            None => path.to_string(),
        };
        for folder in &self.empty_dirs {
            writer
                .add_directory(name(folder), options)
                .map_err(std::io::Error::other)?;
        }
        for (path, bytes) in &self.files {
            writer
                .start_file(name(path), options)
                .map_err(std::io::Error::other)?;
            writer.write_all(bytes)?;
        }
        let cursor = writer.finish().map_err(std::io::Error::other)?;
        Ok(cursor.into_inner())
    }
}

/// How a `.scrivtemplate` spells its project zip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateEncoding {
    /// Scrivener 3: `Type="Base64"`.
    Base64,
    /// Scrivener 2: no `Type`, hexadecimal.
    Hex,
}

/// A `.scrivtemplate` wrapping `zip`.
pub fn template(zip: &[u8], encoding: TemplateEncoding, title: &str) -> Vec<u8> {
    let (attr, data) = match encoding {
        TemplateEncoding::Base64 => (
            " Type=\"Base64\"",
            base64::engine::general_purpose::STANDARD.encode(zip),
        ),
        TemplateEncoding::Hex => ("", zip.iter().map(|b| format!("{b:02x}")).collect()),
    };
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ScrivenerTemplate Version=\"1.0\">\n    <Title>{}</Title>\n    <Category>Fiction</Category>\n    <ProjectZipData{attr}>{data}</ProjectZipData>\n</ScrivenerTemplate>\n",
        crate::xml_escape(title)
    )
    .into_bytes()
}

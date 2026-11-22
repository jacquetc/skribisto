// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The text a search matches against for one resource.
//!
//! One string, so `run_search` and `occurrences_for_result` count the same hits in
//! the same order: the title, the description, the file name and the address, one per
//! line, in that order. A field that is empty, or that repeats the one before it (a
//! stored file whose title is still its file name), is left out rather than counted
//! twice.

use common::entities::{Resource, ResourceKind};

/// See the module docs.
pub fn resource_text(resource: &Resource) -> String {
    let mut parts: Vec<&str> = Vec::with_capacity(4);
    let location = match resource.kind {
        // A stored file's `location` is empty; a linked file's is a path on this
        // machine, of which only the file name is the writer's word.
        ResourceKind::StoredFile | ResourceKind::LinkedFile => "",
        ResourceKind::WebLink => resource.location.as_str(),
    };
    for part in [
        resource.title.as_str(),
        resource.description.as_str(),
        resource.file_name.as_str(),
        location,
    ] {
        let part = part.trim();
        if !part.is_empty() && !parts.contains(&part) {
            parts.push(part);
        }
    }
    parts.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resource(kind: ResourceKind, title: &str, file: &str, location: &str) -> Resource {
        Resource {
            kind,
            title: title.into(),
            description: "Where the tannery stood".into(),
            file_name: file.into(),
            location: location.into(),
            ..Default::default()
        }
    }

    #[test]
    fn a_title_that_is_the_file_name_is_matched_once() {
        let r = resource(ResourceKind::StoredFile, "map.pdf", "map.pdf", "");
        assert_eq!(resource_text(&r), "map.pdf\nWhere the tannery stood");
    }

    #[test]
    fn a_web_link_is_found_by_its_address() {
        let r = resource(
            ResourceKind::WebLink,
            "Tanneries",
            "",
            "https://example.org/tanneries",
        );
        assert_eq!(
            resource_text(&r),
            "Tanneries\nWhere the tannery stood\nhttps://example.org/tanneries"
        );
    }

    #[test]
    fn a_linked_file_is_not_found_by_the_folders_of_its_path() {
        let r = resource(
            ResourceKind::LinkedFile,
            "Interview",
            "interview.mp3",
            "/home/someone/Research/interview.mp3",
        );
        assert!(!resource_text(&r).contains("Research"));
    }
}

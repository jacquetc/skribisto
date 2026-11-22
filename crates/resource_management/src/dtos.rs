// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StoreResourceFilesDto {
    pub dest_dir: String,
    pub source_paths: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StoredResourceFilesDto {
    pub source_paths: Vec<String>,
    pub uids: Vec<String>,
    pub file_names: Vec<String>,
    pub mime_types: Vec<String>,
    pub byte_sizes: Vec<u64>,
    pub content_hashes: Vec<String>,
    pub widths: Vec<u64>,
    pub heights: Vec<u64>,
    pub failed_paths: Vec<String>,
    pub failure_reasons: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AddResourcesDto {
    pub work_id: u64,
    pub item_ids: Vec<u64>,
    pub stored_uids: Vec<String>,
    pub stored_titles: Vec<String>,
    pub stored_file_names: Vec<String>,
    pub stored_mime_types: Vec<String>,
    pub stored_byte_sizes: Vec<u64>,
    pub stored_content_hashes: Vec<String>,
    pub stored_widths: Vec<u64>,
    pub stored_heights: Vec<u64>,
    pub linked_paths: Vec<String>,
    pub web_urls: Vec<String>,
    pub web_titles: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AddResourcesResultDto {
    pub created_ids: Vec<u64>,
}

// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `FolderResourcesViewModel`: the logic of a folder's **Resources** page, the files and
//! web links of the folder and of everything inside it.
//!
//! ## What the page lists
//!
//! The resources attached to the folder itself and to every item of its subtree, the
//! trashed ones left out (the same subtree the Overview tabulates,
//! [`crate::models::subtree_items`]). A resource on several of those items is listed
//! once, naming them all in binder order, the folder first. The list is sorted by title,
//! as the dock's is: a writer looks a map up by its name, not by where it was filed.
//!
//! ## What it does
//!
//! Everything the dock does, through the window's [`ResourcesViewModel`]: the row and its
//! menu are the dock's ([`crate::resources::row`]). What differs is where a new resource
//! goes: a file or a link added from this page is attached to the folder, whatever item
//! is in front of the writer.
//!
//! Plain Rust over the session's model; the widgets are in [`super::folder_pane`].

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;

use teksilo::prelude::*;

use frontend::AppContext;

use crate::models::ResourceRow;
use crate::resources::row::{AttachedItem, title_of};
use crate::resources::{AddTarget, ResourcesViewModel};

/// One resource on a folder's page, with the items it is on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FolderResource {
    pub row: ResourceRow,
    /// In binder order, the folder first when it carries the resource itself.
    pub items: Vec<AttachedItem>,
}

/// How the page lays its resources out: the remembered choice of
/// [`crate::RESOURCES_FOLDER_VIEW_KEY`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FolderView {
    /// The dock's rows, each with the items it is on.
    #[default]
    List,
    /// Tiles: a picture's thumbnail, else the type's icon and the title.
    Grid,
}

impl FolderView {
    pub const ALL: [Self; 2] = [Self::List, Self::Grid];

    /// The stored setting as a view; anything unknown is the list.
    pub fn from_setting(value: &str) -> Self {
        match value {
            "grid" => Self::Grid,
            _ => Self::List,
        }
    }

    /// The view as the setting stores it.
    pub fn as_setting(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Grid => "grid",
        }
    }

    /// Its place on the page's segmented control.
    pub fn index(self) -> usize {
        match self {
            Self::List => 0,
            Self::Grid => 1,
        }
    }

    pub fn from_index(index: usize) -> Self {
        if index == 1 { Self::Grid } else { Self::List }
    }
}

/// The resources of `items` (the folder first, then its subtree in binder order), each
/// once with the items it is on, sorted by title.
///
/// `attachments` maps an item to the resource ids it carries; `rows` are the resources
/// themselves. A row no item of `items` carries is left out, and so is an id with no row.
pub fn collect_in_subtree(
    items: &[AttachedItem],
    attachments: &HashMap<u64, Vec<u64>>,
    rows: Vec<ResourceRow>,
) -> Vec<FolderResource> {
    let mut on: HashMap<u64, Vec<AttachedItem>> = HashMap::new();
    for item in items {
        let Some(ids) = attachments.get(&item.id) else {
            continue;
        };
        let mut seen = HashSet::new();
        for id in ids {
            // An item listing a resource twice still names that item once.
            if seen.insert(*id) {
                on.entry(*id).or_default().push(item.clone());
            }
        }
    }
    let mut out: Vec<FolderResource> = rows
        .into_iter()
        .filter_map(|row| {
            let items = on.remove(&row.id)?;
            Some(FolderResource { row, items })
        })
        .collect();
    // `on.remove` above takes each resource's items once, so a row given twice is kept
    // once.
    out.sort_by_cached_key(|r| (super::resources_vm::title_key(&title_of(&r.row)), r.row.id));
    out
}

struct Inner {
    app_ctx: Rc<AppContext>,
    work_id: Signal<Option<u64>>,
    container: u64,
    container_title: Signal<String>,
    resources: ResourcesViewModel,
}

/// The Resources page of one folder tab. Cheap to clone; clones share their state.
#[derive(Clone)]
pub struct FolderResourcesViewModel {
    inner: Rc<Inner>,
}

impl std::fmt::Debug for FolderResourcesViewModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FolderResourcesViewModel")
            .field("container", &self.inner.container)
            .finish()
    }
}

impl FolderResourcesViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        work_id: Signal<Option<u64>>,
        container: u64,
        container_title: Signal<String>,
        resources: ResourcesViewModel,
    ) -> Self {
        Self {
            inner: Rc::new(Inner {
                app_ctx,
                work_id,
                container,
                container_title,
                resources,
            }),
        }
    }

    /// The window's Resources view-model, every action goes through.
    pub fn resources(&self) -> &ResourcesViewModel {
        &self.inner.resources
    }

    /// The folder whose page this is.
    pub fn container(&self) -> u64 {
        self.inner.container
    }

    /// Bumps whenever a resource or an attachment may have changed.
    pub fn version(&self) -> Signal<u64> {
        self.inner.resources.version()
    }

    pub fn work_id(&self) -> Signal<Option<u64>> {
        self.inner.work_id.clone()
    }

    /// The folder, then its subtree, by name.
    pub fn items(&self) -> Vec<AttachedItem> {
        let mut items = vec![AttachedItem {
            id: self.inner.container,
            title: self.inner.container_title.get(),
        }];
        items.extend(
            crate::models::subtree_items(
                &self.inner.app_ctx,
                &self.inner.work_id,
                self.inner.container,
            )
            .into_iter()
            .map(|(id, title)| AttachedItem { id, title }),
        );
        items
    }

    /// What the page lists now: see the module docs.
    pub fn entries(&self) -> Vec<FolderResource> {
        let items = self.items();
        let ids: Vec<u64> = items.iter().map(|i| i.id).collect();
        let model = self.inner.resources.session().model();
        let attachments = model.attachments(&ids).unwrap_or_else(|e| {
            eprintln!("resources: could not read what the items carry: {e}");
            HashMap::new()
        });
        let mut resource_ids: Vec<u64> = attachments.values().flatten().copied().collect();
        resource_ids.sort_unstable();
        resource_ids.dedup();
        let rows = model.by_ids(&resource_ids);
        collect_in_subtree(&items, &attachments, rows)
    }

    // ── Adding, to the folder ────────────────────────────────────────────────

    fn to_folder(&self) -> Vec<u64> {
        vec![self.inner.container]
    }

    /// Ask for files to attach to the folder.
    pub fn pick_files(&self, ctx: &mut EventContext) {
        self.inner.resources.pick_files_for(ctx, self.to_folder());
    }

    /// Attach `paths` to the folder, through the dock's questions and copy.
    pub fn add_paths(&self, ctx: &mut EventContext, paths: Vec<PathBuf>) {
        self.inner
            .resources
            .add_paths_to(ctx, self.to_folder(), paths, AddTarget::Dock);
    }

    /// Every web address in a dropped or pasted text, attached to the folder.
    pub fn add_dropped_text(&self, ctx: &mut EventContext, text: &str) -> bool {
        self.inner
            .resources
            .add_dropped_text_to(ctx, &self.to_folder(), text)
    }

    /// Ask for a web address and attach it to the folder.
    pub fn add_link(&self, ctx: &mut EventContext, url: &str) {
        super::edit_panel::present_add_link_to(&self.inner.resources, ctx, url, self.to_folder());
    }
}

#[cfg(test)]
mod tests;

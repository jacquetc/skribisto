//! Layer B — build-with-mocks (`mocks` feature).
//!
//! Fabricates a small binder tree so the UI runs with no real backend.
//! The Rust analogue of the C++ `SKR_BUILD_WITH_MOCKS` / `mock_imports`.

use bastyde::data::TreeModel;

use crate::models::TreeNode;

fn item(title: &str, label: &str, kind: &str) -> TreeNode {
    TreeNode { title: title.to_string(), label: label.to_string(), kind: kind.to_string() }
}

/// Populate the tree model with a hardcoded sample project.
pub fn populate(model: &TreeModel<TreeNode>) {
    let manuscript = model.insert_root(0, TreeNode::binder("Manuscript".to_string()));
    let chapter = model.insert_child(manuscript, 0, item("Chapter 1", "the setup", "folder"));
    model.insert_child(chapter, 0, item("Opening scene", "1st plot point", "item"));
    model.insert_child(chapter, 1, item("Inciting incident", "", "item"));
    let chapter2 = model.insert_child(manuscript, 1, item("Chapter 2", "rising action", "folder"));
    model.insert_child(chapter2, 0, item("The journey begins", "", "item"));

    let notes = model.insert_root(1, TreeNode::binder("Notes".to_string()));
    model.insert_child(notes, 0, item("Protagonist", "wants freedom", "item"));
    model.insert_child(notes, 1, item("Antagonist", "", "item"));
}

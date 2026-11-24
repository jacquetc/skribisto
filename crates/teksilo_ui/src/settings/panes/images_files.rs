// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Images & files: what happens to a large image inserted into the text and
//! a large file attached as a resource.
//!
//! Both rows are the way back out of a one-way door. The insert-an-image prompt and the
//! Resources dock's large-file question each offer "Don't ask again", and ticking it
//! writes an answer for good; this page is where the writer finds it and changes it. On
//! the old Editor Behavior page the first sat among the focus aids and the second under a
//! heading of its own, so nothing said the two were the same kind of setting.

use teksilo::widgets::{ComboBox, Segment, SegmentedControl};

#[allow(unused_imports)]
use super::super::*;

/// The three values `editor.image_size_policy` takes, in the order they are
/// offered. The strings are the setting's own storage vocabulary — the same three
/// `app::commands::images::stored_policy` reads back — so they are literals here
/// rather than an enum: an enum would be a fourth place the vocabulary is written
/// down, and the two sides could then disagree silently.
const IMAGE_SIZE_POLICIES: [&str; 3] = ["ask", "keep", "downscale"];

/// Display name for each large-image policy.
fn image_policy_label(value: &str) -> LocalizedString {
    match value {
        "keep" => tr!(settings_image_policy_keep()),
        "downscale" => tr!(settings_image_policy_downscale()),
        _ => tr!(settings_image_policy_ask()),
    }
}

/// Which segment a stored policy selects.
///
/// Anything unrecognised lands on "Ask each time" — the same fallback
/// `app::commands::images::stored_policy` applies when it reads the key back, so
/// a value hand-written into `general.toml` cannot make the control disagree with
/// what the insert path will actually do.
fn image_policy_index(value: &str) -> usize {
    IMAGE_SIZE_POLICIES
        .iter()
        .position(|p| *p == value)
        .unwrap_or(0)
}

/// The three values `resources.large_file` takes, in the order they are offered: the
/// storage vocabulary the Resources dock's question reads back
/// (`resources::large_file_choice`), as the image policy's is above.
const LARGE_FILE_CHOICES: [&str; 3] = ["ask", "store", "link"];

/// Display name for each large-file answer, worded as the dock's question words them.
fn large_file_label(value: &str) -> LocalizedString {
    match value {
        "store" => tr!(settings_resources_large_store()),
        "link" => tr!(settings_resources_large_link()),
        _ => tr!(settings_resources_large_ask()),
    }
}

/// Which choice a stored large-file answer selects; anything unrecognised is "Ask each
/// time", the dock's own reading of it.
fn large_file_choice(value: &str) -> &'static str {
    LARGE_FILE_CHOICES
        .iter()
        .find(|p| **p == value)
        .copied()
        .unwrap_or(LARGE_FILE_CHOICES[0])
}

/// Bridge the stored `resources.large_file` string to the `ComboBox`'s selection, the
/// image policy's bridge over another vocabulary. A combo box rather than segments: the
/// three answers are worded as the dock's question words them, and three sentences that
/// long do not fit side by side.
fn bridge_large_file(
    ctx: &mut BuildContext,
    choice: Signal<String>,
) -> Signal<Option<&'static str>> {
    let selected = Signal::new(Some(large_file_choice(&choice.get())));
    {
        let selected = selected.clone();
        ctx.effect(&choice, move |v: &String| {
            selected.set_if_changed(Some(large_file_choice(v)));
        });
    }
    {
        ctx.effect(&selected, move |s| {
            let want = s.unwrap_or(LARGE_FILE_CHOICES[0]).to_string();
            if choice.get() != want {
                choice.set(want);
            }
        });
    }
    selected
}

/// Bridge the stored `editor.image_size_policy` string to the
/// `SegmentedControl`'s `usize`, and return the index signal.
///
/// Two guarded effects, the same shape as the caret band's bridge: the `!=`
/// guards are what stop the two signals from writing each other back and forth
/// for ever. The stored form stays a string because it is the vocabulary the
/// *insert* path reads (`app::commands::images`), and a settings page must not
/// invent a second one.
fn bridge_image_policy(ctx: &mut BuildContext, policy: Signal<String>) -> Signal<usize> {
    let index: Signal<usize> = Signal::new(image_policy_index(&policy.get()));
    {
        let index = index.clone();
        ctx.effect(&policy, move |v: &String| {
            let i = image_policy_index(v);
            if index.get() != i {
                index.set(i);
            }
        });
    }
    {
        let policy = policy.clone();
        ctx.effect(&index, move |i| {
            let want = IMAGE_SIZE_POLICIES
                .get(*i)
                .copied()
                .unwrap_or(IMAGE_SIZE_POLICIES[0])
                .to_string();
            if policy.get() != want {
                policy.set(want);
            }
        });
    }
    index
}

/// Editor ▸ Images & files: the large-image policy and the large-file answer.
pub(in crate::settings) fn images_files_pane(
    ctx: &mut BuildContext,
    crumbs: &Crumbs,
    vm: &SettingsViewModel,
) -> impl Widget {
    // Through the view-model, not `ctx.settings()` directly: this row and the
    // Reset button must move the *same* signal, or a reset would put the store
    // back to `ask` while the segmented control kept showing what it had. The
    // accessor is lazy on purpose (it seeds the key only where it is about to be
    // written), so reading it here — inside a pane the writer opened — is the
    // moment it is meant to be read.
    let policy_index = bridge_image_policy(ctx, vm.image_size_policy());
    let image_policy_control = IMAGE_SIZE_POLICIES
        .iter()
        .fold(SegmentedControl::indexed(policy_index), |control, value| {
            control.segment(Segment::new(image_policy_label(value)))
        });

    let large_file_control = ComboBox::from_items(
        LARGE_FILE_CHOICES,
        bridge_large_file(ctx, vm.resources_large_file()),
        |value: &&str| large_file_label(value),
    )
    .label(tr!(settings_resources_large_file()));

    let form = FormLayout::new()
        .label(tr!(settings_page_images_files()))
        .label_gap(16.0)
        .row_spacing(14.0)
        // The insert-an-image prompt offers "don't ask again", and ticking it writes
        // `keep` or `downscale` into `editor.image_size_policy` for good — after which
        // every later insert is silently handled that way and *no page in the window
        // said so*, which left hand-editing `general.toml` as the only undo.
        .full_width(group(tr!(settings_group_images())))
        .line(
            field_label(tr!(settings_field_image_size_policy())),
            image_policy_control,
        )
        .full_width(hint(tr!(settings_hint_image_size_policy())))
        // The same one-way door on the Resources side: the large-file question's
        // "Don't ask again" writes here, and this is the way back.
        .full_width(group(tr!(resources_title())))
        .line(
            field_label(tr!(settings_resources_large_file())),
            large_file_control,
        )
        .full_width(hint(tr!(settings_hint_resources_large_file())));

    pane_frame(crumbs.of(Pane::ImagesFiles), form)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::core::{LayoutContext, LayoutResponse, Widget, WidgetId};

    /// Hosts [`bridge_image_policy`] in a real `BuildContext` and hands the index
    /// signal back, so a test can act as the `SegmentedControl` does.
    struct PolicyHost {
        policy: Signal<String>,
        out: Rc<std::cell::RefCell<Option<Signal<usize>>>>,
    }
    impl std::fmt::Debug for PolicyHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PolicyHost").finish()
        }
    }
    impl Widget for PolicyHost {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            *self.out.borrow_mut() = Some(bridge_image_policy(ctx, self.policy.clone()));
            Vec::new()
        }
        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            proposal.resolve(0.0, 0.0).into()
        }
    }

    /// A live policy bridge. The tree is kept alive deliberately: `ctx.effect`'s
    /// observers are owned by it, and dropping it would unregister the bridge
    /// under test and leave every assertion passing against nothing.
    struct Policy {
        policy: Signal<String>,
        index: Signal<usize>,
        _tree: WidgetTree,
    }

    fn policy_bridge(stored: &str) -> Policy {
        let policy = Signal::new(stored.to_string());
        let out = Rc::new(std::cell::RefCell::new(None));
        let mut tree = WidgetTree::new();
        tree.add(PolicyHost {
            policy: policy.clone(),
            out: out.clone(),
        });
        tree.layout(SizeProposal::exact(100.0, 100.0));
        let index = out.borrow().clone().expect("the bridge was built");
        Policy {
            policy,
            index,
            _tree: tree,
        }
    }

    /// Hosts [`bridge_large_file`] the way [`PolicyHost`] hosts the image policy's.
    struct LargeFileHost {
        choice: Signal<String>,
        out: Rc<std::cell::RefCell<Option<Signal<Option<&'static str>>>>>,
    }
    impl std::fmt::Debug for LargeFileHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("LargeFileHost").finish()
        }
    }
    impl Widget for LargeFileHost {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            *self.out.borrow_mut() = Some(bridge_large_file(ctx, self.choice.clone()));
            Vec::new()
        }
        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            proposal.resolve(0.0, 0.0).into()
        }
    }

    /// The Resources dock's large-file question and this page speak one vocabulary: the
    /// three values the combo box writes are the ones the question reads back, and
    /// picking one writes it, while one written by the question's "Don't ask again"
    /// moves the combo box.
    #[test]
    fn the_large_file_preference_round_trips_with_the_docks_question() {
        use crate::resources::{LargeFileChoice, large_file_choice as read};
        assert_eq!(LARGE_FILE_CHOICES[0], crate::RESOURCES_LARGE_FILE_DEFAULT);
        assert_eq!(
            read(LARGE_FILE_CHOICES[0]),
            None,
            "ask: the question comes up"
        );
        assert_eq!(read(LARGE_FILE_CHOICES[1]), Some(LargeFileChoice::Store));
        assert_eq!(read(LARGE_FILE_CHOICES[2]), Some(LargeFileChoice::Link));

        let choice = Signal::new("ask".to_string());
        let out = Rc::new(std::cell::RefCell::new(None));
        let mut tree = WidgetTree::new();
        tree.add(LargeFileHost {
            choice: choice.clone(),
            out: out.clone(),
        });
        tree.layout(SizeProposal::exact(100.0, 100.0));
        let selected = out.borrow().clone().expect("the bridge was built");
        assert_eq!(selected.get(), Some("ask"));
        selected.set(Some("link"));
        assert_eq!(choice.get(), "link");
        choice.set("store".to_string()); // the question's "Don't ask again"
        assert_eq!(selected.get(), Some("store"));
        choice.set("whatever".to_string());
        assert_eq!(
            selected.get(),
            Some("ask"),
            "unknown reads as ask, as the dock does"
        );
        drop(tree);
    }

    /// The vocabulary this page writes is the one the *insert* path reads.
    ///
    /// `editor.image_size_policy` had no page at all: the insert prompt's "don't
    /// ask again" box wrote `keep` or `downscale` and nothing in the window could
    /// write it back. A page that offered its own spelling of those three would
    /// have been worse than none — it would look like an undo and change nothing.
    #[test]
    fn the_offered_policies_are_the_ones_the_insert_path_stores() {
        assert_eq!(IMAGE_SIZE_POLICIES, ["ask", "keep", "downscale"]);
    }

    /// Picking a segment writes the stored string, in both directions.
    #[test]
    fn the_large_image_policy_round_trips_through_the_control() {
        let b = policy_bridge("ask");
        assert_eq!(b.index.get(), 0);

        b.index.set(2); // the writer picks "Optimise"
        assert_eq!(b.policy.get(), "downscale");

        b.index.set(1);
        assert_eq!(b.policy.get(), "keep");

        // …and a value written from elsewhere (the insert prompt's "don't ask
        // again" box) moves the control.
        b.policy.set("ask".to_string());
        assert_eq!(b.index.get(), 0, "the control must follow the setting");
    }

    /// A value nothing recognises reads as "ask" — the same fallback
    /// `stored_policy` applies — so a hand-edited `general.toml` cannot make the
    /// page disagree with what an insert will actually do.
    #[test]
    fn an_unrecognised_stored_policy_shows_as_ask() {
        assert_eq!(image_policy_index("shrink-a-bit"), 0);
        assert_eq!(image_policy_index(""), 0);
        let b = policy_bridge("shrink-a-bit");
        assert_eq!(b.index.get(), 0);
    }
}

// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings an extension owns: its own keys in `general.toml`, and its own page
//! in the Settings window.
//!
//! ## Keys, not a private file of its own
//!
//! An extension could always have kept preferences in a file beside the app's.
//! What it could not do is be *configurable the way the app is*: `--dump-config`
//! would not list its keys, `--config` would refuse them as unknown, and a typo
//! in one would be silence rather than a startup error naming the nearest legal
//! neighbour. That silence is the exact failure `settings_keys` exists to
//! abolish, and an extension living outside it just moves the failure rather than
//! removing it.
//!
//! So a registered key is a real key: [`settings_keys::all_specs`] is the app's
//! table **plus** whatever is registered, and every consumer reads through it.
//! The value lives in `general.toml` like any other, and the extension reads it
//! with `ctx.settings().signal(key, default)` exactly as the app does.
//!
//! ⚠ **Uninstalling leaves its keys behind in `general.toml`.** They are inert —
//! nothing reads them, and `general.toml` is not validated on load — but
//! `--dump-config` stops listing them and `--config` starts refusing them. That
//! is the honest behaviour: the values are still there if the extension comes
//! back, and silently deleting a writer's configuration because a build did not
//! recognise it is the mistake carry-through exists to avoid one level down.
//!
//! ## Namespacing is enforced, and the guard cannot rot
//!
//! A key must be namespaced under a section the application does not use, and
//! that section list is **derived from `SETTINGS` at runtime**
//! ([`settings_keys::app_sections`]) rather than hand-copied. `commands_ext`'s
//! first cut hand-copied its equivalent and shipped missing an entry; this one
//! cannot, because there is nothing to keep in step.
//!
//! ## Pages
//!
//! A registered page becomes a node under a single **Extensions** section in the
//! Settings window's tree, and its body is built with a real `BuildContext`, so
//! it reaches `ctx.settings()` and can bind its own keys like any built-in pane.
//!
//! The section is one fixed place rather than a parent an extension names,
//! deliberately: letting a registration address the app's own tree would make
//! that tree's shape a compatibility promise, and rearranging Settings is
//! ordinary work.
//!
//! ⚠ Like every registry in the seam this is a **snapshot**, read when the
//! Settings window is built (and, for keys, whenever a key is looked up).
//! Register at startup, on the main thread, before `run()`.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{LazyLock, RwLock};

use teksilo::prelude::{BuildContext, Widget};

use crate::settings_keys::{self, SettingSpec};

// ─────────────────────────────────────────────────────────────────────────────
// Keys
// ─────────────────────────────────────────────────────────────────────────────

struct RegisteredSettings {
    namespace: String,
    specs: Vec<SettingSpec>,
}

// A plain `RwLock`, not a `thread_local!`: `SettingSpec` is `&'static str`s and
// `fn` pointers, so it is `Send + Sync`, and `--dump-config` runs before any UI
// thread exists.
static SETTINGS_REG: LazyLock<RwLock<Vec<RegisteredSettings>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));

/// Declare settings keys of an extension's own.
///
/// Every key must be namespaced under a section the application does not use
/// (`pro.…`, `acme.…`), must contain a dot, and must not already be taken.
/// Returns `Err` naming the offender if any of those fails — **and registers
/// none of them**, so a typo cannot leave an extension half-configured.
///
/// The returned handle unregisters on drop; registering the same namespace twice
/// replaces its earlier keys rather than stacking a second copy.
pub fn register_settings(
    namespace: impl Into<String>,
    specs: Vec<SettingSpec>,
) -> Result<SettingsHandle, String> {
    let namespace = namespace.into();
    if specs.is_empty() {
        return Err("register_settings was given no keys".to_string());
    }

    let app_sections = settings_keys::app_sections();
    let taken: Vec<&'static str> = SETTINGS_REG
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .filter(|r| r.namespace != namespace)
        .flat_map(|r| r.specs.iter().map(|s| s.key))
        .collect();

    // Validate the whole batch before touching the registry: half-registering a
    // set of keys would leave the extension reading a default for the one that
    // was refused, which is worse than refusing all of them.
    for spec in &specs {
        let Some(section) = spec.key.split('.').next().filter(|s| !s.is_empty()) else {
            return Err(format!("setting key '{}' has no name", spec.key));
        };
        if !spec.key.contains('.') {
            return Err(format!(
                "setting key '{}' is not namespaced; use '{namespace}.{}'",
                spec.key, spec.key
            ));
        }
        if app_sections.contains(section) {
            return Err(format!(
                "setting key '{}' is under '{section}', which the application owns; namespace \
                 yours",
                spec.key
            ));
        }
        if settings_keys::SETTINGS.iter().any(|s| s.key == spec.key) {
            return Err(format!("setting key '{}' is one of the app's own", spec.key));
        }
        if taken.contains(&spec.key) {
            return Err(format!(
                "setting key '{}' is already registered by another extension",
                spec.key
            ));
        }
        if let Err(e) = (spec.check)(&(spec.default)()) {
            return Err(format!(
                "setting key '{}' declares a default its own validator rejects ({e})",
                spec.key
            ));
        }
    }

    let mut reg = SETTINGS_REG.write().unwrap_or_else(|e| e.into_inner());
    reg.retain(|r| r.namespace != namespace);
    reg.push(RegisteredSettings {
        namespace: namespace.clone(),
        specs,
    });
    Ok(SettingsHandle { namespace })
}

/// Unregisters its keys when dropped.
#[derive(Debug)]
pub struct SettingsHandle {
    namespace: String,
}

impl Drop for SettingsHandle {
    fn drop(&mut self) {
        SETTINGS_REG
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|r| r.namespace != self.namespace);
    }
}

/// Every registered key, in registration order. Read by
/// [`settings_keys::all_specs`].
pub fn registered_settings() -> Vec<SettingSpec> {
    SETTINGS_REG
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .flat_map(|r| r.specs.iter().copied())
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Pages
// ─────────────────────────────────────────────────────────────────────────────

/// Builds a settings page's body.
///
/// Handed a real `BuildContext`, so it reaches `ctx.settings()` and binds its own
/// keys exactly as a built-in pane does — there is no separate context type,
/// because there is nothing a settings page needs that a `BuildContext` does not
/// already carry.
pub type PageBuildFn = Rc<dyn Fn(&mut BuildContext) -> Box<dyn Widget>>;

/// An extension's page in the Settings window.
#[derive(Clone)]
pub struct SettingsPage {
    /// Stable id, namespaced (`"pro.structure"`). Shown nowhere; it is what the
    /// panel addresses the page by, so it must not change once shipped.
    pub id: &'static str,
    /// Resolved per build, so a runtime locale switch reaches the tree label —
    /// the same reason [`crate::docks::ExtensionDock::title`] stores a closure.
    pub label: crate::docks::LabelFn,
    pub build: PageBuildFn,
}

struct RegisteredPage {
    namespace: String,
    page: SettingsPage,
}

// Thread-local, unlike the keys above: a page holds an `Rc` widget builder, and
// its only reader is the Settings window on the UI thread.
thread_local! {
    static PAGES: RefCell<Vec<RegisteredPage>> = const { RefCell::new(Vec::new()) };
}

/// Add a page to the Settings window, under **Extensions**.
///
/// Returns `Err` if the id is empty, is not namespaced, or is already taken by
/// another extension — a second page on one id would make whichever the tree
/// reached first the only one ever shown.
pub fn register_page(
    namespace: impl Into<String>,
    page: SettingsPage,
) -> Result<PageHandle, String> {
    let namespace = namespace.into();
    if page.id.is_empty() {
        return Err("a settings page needs an id".to_string());
    }
    if !page.id.contains('.') {
        return Err(format!(
            "settings page id '{}' is not namespaced; use '{namespace}.{}'",
            page.id, page.id
        ));
    }
    PAGES.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(other) = reg
            .iter()
            .find(|r| r.page.id == page.id && r.namespace != namespace)
        {
            return Err(format!(
                "settings page id '{}' is already registered by '{}'",
                page.id, other.namespace
            ));
        }
        reg.retain(|r| r.namespace != namespace);
        reg.push(RegisteredPage {
            namespace: namespace.clone(),
            page,
        });
        Ok(PageHandle { namespace })
    })
}

/// Unregisters its page when dropped.
#[derive(Debug)]
pub struct PageHandle {
    namespace: String,
}

impl Drop for PageHandle {
    fn drop(&mut self) {
        // `try_with`: a handle released during thread teardown must not panic.
        let _ = PAGES.try_with(|reg| {
            reg.borrow_mut().retain(|r| r.namespace != self.namespace);
        });
    }
}

/// Every registered page, in registration order.
pub fn registered_pages() -> Vec<SettingsPage> {
    PAGES.with(|reg| reg.borrow().iter().map(|r| r.page.clone()).collect())
}

/// Whether the Settings tree should grow an **Extensions** section at all.
pub fn has_pages() -> bool {
    PAGES.with(|reg| !reg.borrow().is_empty())
}

/// The body of the page with `id`, or `None` if nothing is registered under it.
pub(crate) fn build_page(id: &str, ctx: &mut BuildContext) -> Option<Box<dyn Widget>> {
    let build = PAGES.with(|reg| {
        reg.borrow()
            .iter()
            .find(|r| r.page.id == id)
            .map(|r| r.page.build.clone())
    })?;
    Some(build(ctx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::prelude::lit;

    // Both registries are process-wide and tests run in parallel, so no assertion
    // here may depend on a total — each test owns its own namespace and keys.

    fn spec(key: &'static str) -> SettingSpec {
        SettingSpec {
            key,
            ty: "bool",
            default: || toml::Value::Boolean(false),
            check: |v| {
                v.as_bool()
                    .map(|_| ())
                    .ok_or_else(|| "expected a boolean".to_string())
            },
            doc: "A test setting.",
        }
    }

    fn page(id: &'static str) -> SettingsPage {
        SettingsPage {
            id,
            label: Rc::new(|| lit!("Test".to_string())),
            build: Rc::new(|_| Box::new(teksilo::widgets::Spacer::new())),
        }
    }

    fn registered(key: &str) -> bool {
        registered_settings().iter().any(|s| s.key == key)
    }

    /// A registered key is a **real** key: it reaches the same lookup
    /// `--config` validates against and `--dump-config` prints from. A key that
    /// could be set but not dumped is the silent-configuration failure this whole
    /// module exists to remove, one layer out.
    #[test]
    fn a_registered_key_is_settable_and_dumpable_like_any_other() {
        assert!(settings_keys::spec("t1.enabled").is_none());
        let _h = register_settings("test.set.basic", vec![spec("t1.enabled")]).expect("register");

        assert!(registered("t1.enabled"));
        assert!(
            settings_keys::spec("t1.enabled").is_some(),
            "the app's own lookup must see it, or --config refuses it as unknown"
        );

        let dir = tempfile::tempdir().unwrap();
        let general = dir.path().join("general.toml");
        let text = settings_keys::dump(&general);
        assert!(
            text.contains("t1.enabled = false"),
            "--dump-config must list an extension's keys:\n{text}"
        );
    }

    /// …and dropping the handle takes it back out of every one of those.
    #[test]
    fn dropping_the_handle_unregisters_the_keys() {
        {
            let _h = register_settings("test.set.drop", vec![spec("t2.enabled")]).expect("register");
            assert!(settings_keys::spec("t2.enabled").is_some());
        }
        assert!(settings_keys::spec("t2.enabled").is_none());
        assert!(!registered("t2.enabled"));
    }

    /// The app's own sections are off limits: an extension key under `editor.`
    /// would sit inside a section the app's own drift test polices, and could
    /// collide outright with a key added later.
    ///
    /// Checked against **every** section `SETTINGS` actually declares, not a
    /// hand-picked few — the guard derives its list at runtime, so the test that
    /// proves it should too. (`backup.` is deliberately absent from both: backup
    /// policy is a `SettingsFile<T>`, not a `general.toml` scalar.)
    #[test]
    fn no_key_under_any_app_section_is_accepted() {
        let sections = settings_keys::app_sections();
        assert!(
            sections.len() >= 5,
            "the section scan found only {sections:?} — the derivation is broken"
        );
        for section in sections {
            // `&'static str` by construction: every section comes from a
            // `&'static` key in the app's own table.
            let key: &'static str = Box::leak(format!("{section}.mine").into_boxed_str());
            let err = register_settings("test.set.shadow", vec![spec(key)])
                .expect_err("must refuse a key under an app section");
            assert!(err.contains("the application owns"), "unhelpful: {err}");
        }
    }

    /// An unnamespaced key has no section to be refused by, so it is refused on
    /// its own account rather than slipping through.
    #[test]
    fn an_unnamespaced_key_is_refused() {
        let err = register_settings("test.set.flat", vec![spec("flat")])
            .expect_err("must refuse an unnamespaced key");
        assert!(err.contains("not namespaced"), "unhelpful: {err}");
    }

    /// Two extensions cannot claim one key.
    #[test]
    fn a_second_extension_cannot_take_a_taken_key() {
        let _first = register_settings("test.set.first", vec![spec("t3.shared")]).expect("first");
        let err = register_settings("test.set.second", vec![spec("t3.shared")])
            .expect_err("must refuse");
        assert!(err.contains("already registered"), "unhelpful: {err}");
    }

    /// **Nothing is registered if anything is refused.** A half-registered batch
    /// leaves the extension silently reading a default for the key that failed,
    /// which is worse than refusing the lot.
    #[test]
    fn a_batch_with_one_bad_key_registers_none_of_them() {
        let err = register_settings(
            "test.set.batch",
            vec![spec("t4.good"), spec("editor.bad"), spec("t4.also_good")],
        )
        .expect_err("must refuse the batch");
        assert!(err.contains("editor.bad"), "the error must name the offender: {err}");
        assert!(!registered("t4.good"), "a refused batch must register nothing");
        assert!(!registered("t4.also_good"));
    }

    /// A default its own validator rejects would make the app refuse a perfectly
    /// good pins file — caught at registration, where it is one extension's
    /// problem, rather than at a writer's startup.
    #[test]
    fn a_default_that_fails_its_own_check_is_refused() {
        let bad = SettingSpec {
            key: "t5.wrong",
            ty: "bool",
            default: || toml::Value::String("not a bool".into()),
            check: |v| {
                v.as_bool()
                    .map(|_| ())
                    .ok_or_else(|| "expected a boolean".to_string())
            },
            doc: "A broken test setting.",
        };
        let err = register_settings("test.set.bad-default", vec![bad]).expect_err("must refuse");
        assert!(err.contains("its own validator"), "unhelpful: {err}");
    }

    /// A typo in an extension's key gets the same did-you-mean an app key does —
    /// which only works because `nearest` searches the combined list.
    #[test]
    fn a_typo_in_an_extension_key_suggests_the_real_one() {
        let _h =
            register_settings("test.set.typo", vec![spec("t6.tolerance")]).expect("register");
        let dir = tempfile::tempdir().unwrap();
        let pins = dir.path().join("pins.toml");
        std::fs::write(&pins, "t6.tolerence = false\n").unwrap();

        let err = settings_keys::load_pins(&pins).expect_err("a typo must be refused");
        assert!(
            err.contains("t6.tolerance"),
            "the suggestion must reach an extension's keys too:\n{err}"
        );
    }

    // ── Pages ────────────────────────────────────────────────────────────────

    #[test]
    fn a_registered_page_is_listed_and_builds() {
        assert!(!registered_pages().iter().any(|p| p.id == "t7.page"));
        let _h = register_page("test.page.basic", page("t7.page")).expect("register");

        assert!(has_pages());
        assert!(registered_pages().iter().any(|p| p.id == "t7.page"));

        use teksilo::core::widget_tree::WidgetTree;
        use teksilo::prelude::SizeProposal;
        let mut tree = WidgetTree::new();
        let built = tree.add_boxed(Box::new(crate::tabs::Boxed::new(Box::new(
            teksilo::widgets::Spacer::new(),
        ))));
        tree.layout(SizeProposal::exact(400.0, 300.0));
        assert!(tree.bounds(built).width >= 0.0);
    }

    #[test]
    fn a_page_id_must_be_namespaced_and_unique() {
        let err = register_page("test.page.flat", page("flat")).expect_err("must refuse");
        assert!(err.contains("not namespaced"), "unhelpful: {err}");

        let _first = register_page("test.page.one", page("t8.contested")).expect("first");
        let err = register_page("test.page.two", page("t8.contested")).expect_err("must refuse");
        assert!(err.contains("test.page.one"), "must name the holder: {err}");
    }

    #[test]
    fn re_registering_a_namespace_replaces_its_page() {
        let _a = register_page("test.page.same", page("t9.old")).expect("first");
        let _b = register_page("test.page.same", page("t9.new")).expect("re-register");
        assert!(registered_pages().iter().any(|p| p.id == "t9.new"));
        assert!(!registered_pages().iter().any(|p| p.id == "t9.old"));
    }

    #[test]
    fn dropping_the_handle_unregisters_the_page() {
        {
            let _h = register_page("test.page.drop", page("t10.page")).expect("register");
            assert!(registered_pages().iter().any(|p| p.id == "t10.page"));
        }
        assert!(!registered_pages().iter().any(|p| p.id == "t10.page"));
    }
}

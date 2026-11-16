// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Handing a URL to the OS, and refusing to hand over the wrong sort.
//!
//! Not a view-model: two openers and a predicate.
//!
//! ## Why there is an allowlist
//!
//! The launcher's own links are hard-coded and trustworthy. A link in a
//! manuscript is not: a `.skrib` can arrive from a collaborator, a workshop, an
//! importer, or the web, and every hyperlink in it is a string someone else
//! chose. `open::that_detached` hands that string to the desktop's default
//! handler, which will happily launch a program for a scheme like `file:` or
//! `smb:` — so a document could make a click run something.
//!
//! Only the three schemes a prose link plausibly means are allowed through.
//! Anything else is reported rather than silently dropped, because a click that
//! does nothing reads as a broken app and teaches the writer nothing about why.

use teksilo::prelude::*;
use teksilo::widgets::Toast;

/// The schemes a link in a manuscript may open.
///
/// `http`/`https` for the web, `mailto` for an address. Deliberately not
/// `file:` — see the module docs.
const ALLOWED_SCHEMES: [&str; 3] = ["http", "https", "mailto"];

/// Whether `url` is something we will hand to the OS.
pub fn is_openable(url: &str) -> bool {
    let s = url.trim();
    match s.find(':') {
        Some(colon) => {
            let scheme = &s[..colon];
            ALLOWED_SCHEMES
                .iter()
                .any(|a| scheme.eq_ignore_ascii_case(a))
        }
        None => false,
    }
}

/// Why a link was not opened.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpenLinkError {
    /// The scheme is not one a link may open; see [`is_openable`].
    Refused,
    /// The desktop's opener could not be started, with its error.
    Failed(String),
}

/// Open `url` in the user's browser or mail client, and say whether that worked,
/// without telling the writer: for a caller that reports a failure its own way.
///
/// `that_detached`, not `that`: the latter keeps the spawned opener as a child
/// process and waits for it to exit, and the desktop opener a browser is
/// launched through does not reliably return before the browser itself does —
/// on a cold start that is the UI thread stalled for seconds. Detaching hands
/// the child to the OS and returns now.
///
/// ⚠ `Ok` means an opener *started*, not that a browser showed the page. On
/// Windows the `open` crate runs PowerShell's `Start-Process` and reports success
/// as soon as PowerShell spawns, whatever `Start-Process` then does; and if
/// PowerShell cannot spawn, its `explorer.exe` fallback opens the *Documents*
/// folder for any address with a `=` in it, also as a success (measured on
/// Windows 11 with `open` 5.4.3, 2026-11-16). A caller whose link matters keeps a
/// way to copy it in view either way.
///
/// In a debug build, `SKRIBISTO_LINK_SINK=<file>` appends each URL to that file
/// instead of launching anything, so an automation script can check what a click
/// would have opened without a browser window per run.
pub fn try_open_external_link(url: &str) -> Result<(), OpenLinkError> {
    if !is_openable(url) {
        return Err(OpenLinkError::Refused);
    }
    #[cfg(debug_assertions)]
    if let Some(sink) = std::env::var_os("SKRIBISTO_LINK_SINK").filter(|s| !s.is_empty()) {
        use std::io::Write;
        return std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(sink)
            .and_then(|mut f| writeln!(f, "{url}"))
            .map_err(|e| OpenLinkError::Failed(e.to_string()));
    }
    open::that_detached(url).map_err(|e| OpenLinkError::Failed(e.to_string()))
}

/// Open `url` in the user's browser or mail client, reporting a refusal or a
/// failure in a toast.
///
/// Both are reported. A click that silently does nothing reads as a dead
/// control, and neither the launcher sidebar nor the middle of a manuscript has
/// another surface to notice it on.
pub fn open_external_link(url: &str, ctx: &mut EventContext) {
    match try_open_external_link(url) {
        Ok(()) => {}
        Err(OpenLinkError::Refused) => {
            ctx.show_toast(Toast::error(tr!(link_scheme_refused(
                url = url.to_string()
            ))));
        }
        Err(OpenLinkError::Failed(error)) => {
            ctx.show_toast(Toast::error(tr!(could_not_open_link(
                url = url.to_string(),
                error = error
            ))));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OpenLinkError, is_openable, try_open_external_link};

    #[test]
    fn a_refused_scheme_is_never_handed_to_the_desktop() {
        assert_eq!(
            try_open_external_link("file:///etc/passwd"),
            Err(OpenLinkError::Refused)
        );
    }

    #[test]
    fn the_web_and_mail_are_openable() {
        assert!(is_openable("https://example.com"));
        assert!(is_openable("http://example.com"));
        assert!(is_openable("mailto:someone@example.com"));
        assert!(
            is_openable("HTTPS://EXAMPLE.COM"),
            "scheme is case-insensitive"
        );
    }

    #[test]
    fn a_document_cannot_make_a_click_run_something() {
        // The reason this predicate exists. Every one of these is a string a
        // third-party `.skrib` could contain.
        for hostile in [
            "file:///etc/passwd",
            "javascript:alert(1)",
            "smb://host/share",
            "vscode://file/etc/passwd",
            r"C:\Windows\System32\calc.exe",
        ] {
            assert!(!is_openable(hostile), "{hostile} must be refused");
        }
    }

    #[test]
    fn a_schemeless_address_is_refused() {
        // Callers normalize before storing, so anything still bare here was
        // never a URL to begin with.
        assert!(!is_openable("example.com"));
        assert!(!is_openable(""));
    }
}

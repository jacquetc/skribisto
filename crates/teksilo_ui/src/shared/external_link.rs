// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Handing a URL to the OS, and refusing to hand over the wrong sort.
//!
//! Not a view-model — one function and a predicate.
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

/// Open `url` in the user's browser or mail client.
///
/// `that_detached`, not `that`: the latter keeps the spawned opener as a child
/// process and waits for it to exit, and the desktop opener a browser is
/// launched through does not reliably return before the browser itself does —
/// on a cold start that is the UI thread stalled for seconds. Detaching hands
/// the child to the OS and returns now.
///
/// Both refusals and failures are reported. A click that silently does nothing
/// reads as a dead control, and neither the launcher sidebar nor the middle of
/// a manuscript has another surface to notice it on.
pub fn open_external_link(url: &str, ctx: &mut EventContext) {
    if !is_openable(url) {
        ctx.show_toast(Toast::error(tr!(link_scheme_refused(
            url = url.to_string()
        ))));
        return;
    }
    if let Err(e) = open::that_detached(url) {
        ctx.show_toast(Toast::error(tr!(could_not_open_link(
            url = url.to_string(),
            error = e.to_string()
        ))));
    }
}

#[cfg(test)]
mod tests {
    use super::is_openable;

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

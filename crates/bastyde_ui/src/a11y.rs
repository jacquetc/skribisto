// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Crate-wide accessibility invariants that no single call site can enforce.
//!
//! Everything here is a *source-level* guard rather than a behavioural test. That is a
//! deliberate trade: the invariant below concerns a rule about how a widget is composed,
//! and the failure mode is a call site that simply forgot it. A behavioural test can prove
//! one popover behaves; it cannot notice the thirteenth popover that nobody wrapped.

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Every `.rs` under `src/`, deepest-first order irrelevant.
    fn sources() -> Vec<PathBuf> {
        fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
            for entry in fs::read_dir(dir).expect("readable source dir").flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    out.push(path);
                }
            }
        }
        let mut out = Vec::new();
        walk(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src").as_path(),
            &mut out,
        );
        out
    }

    /// The file's code with line comments and all whitespace removed, so a match cannot be
    /// broken up by however rustfmt chose to wrap the expression, and cannot be satisfied by
    /// a comment that merely *mentions* the rule.
    ///
    /// Cutting at `//` also truncates a string literal containing `//` (a URL, say). That is
    /// harmless here: it can only remove text, and none of the patterns below live in a
    /// string.
    fn code_of(path: &Path) -> String {
        let text = fs::read_to_string(path).expect("readable source file");
        // A `#[cfg(test)]` module may legitimately construct a bare popover to test the
        // widget itself, so stop at the first one.
        let text = match text.find("#[cfg(test)]") {
            Some(i) => &text[..i],
            None => &text[..],
        };
        text.lines()
            .map(|l| match l.find("//") {
                Some(i) => &l[..i],
                None => l,
            })
            .collect::<String>()
            .split_whitespace()
            .collect()
    }

    /// The three ways this crate opens an anchored overlay.
    const POPOVERS: [&str; 3] = [
        "Popover::new",
        "PopoverButton::new",
        "PopoverIconButton::new",
    ];

    const TRAP: &str = "FocusScope::new(TraversalScopePolicy::Cycle)";

    /// A popover that does not trap Tab is reachable but not operable by keyboard.
    ///
    /// Opening an anchored overlay leaves the traversal order rooted in the window behind
    /// it, so Tab walks straight out of the open popover and into the toolbar: a
    /// keyboard-only writer can open the thing and then do nothing with it (WCAG 2.1.1).
    /// `FocusScope` with [`TraversalScopePolicy::Cycle`] is the framework's answer.
    ///
    /// **What it actually checks, and what it does not.** The guarantee is per *file*:
    /// a file that opens a popover must also mention the trap somewhere in it. It does
    /// not try to pair each constructor with its own content expression — the content is
    /// often a `let`-bound widget assembled several lines earlier, and matching those
    /// lexically would be guesswork dressed up as rigour — so this would miss a file that
    /// wraps one popover and forgets a second. Pairing them properly needs the type
    /// system, not a regex: the real fix is for `Popover` to scope its own content in
    /// bastyde, which is not this crate's call.
    #[test]
    fn every_file_with_a_popover_also_traps_tab() {
        let mut checked = 0usize;
        let mut untrapped = Vec::new();

        for path in sources() {
            let code = code_of(&path);
            let count = POPOVERS
                .iter()
                .map(|p| code.matches(p).count())
                .sum::<usize>();
            if count == 0 {
                continue;
            }
            checked += count;
            if !code.contains(TRAP) {
                untrapped.push(format!(
                    "{} ({count} popover{})",
                    path.strip_prefix(env!("CARGO_MANIFEST_DIR"))
                        .unwrap_or(&path)
                        .display(),
                    if count == 1 { "" } else { "s" }
                ));
            }
        }

        assert!(
            untrapped.is_empty(),
            "these files open a popover but never trap Tab inside it, so a keyboard-only \
             writer can open the overlay and not reach its contents (WCAG 2.1.1). Wrap the \
             content in `{TRAP}`, as `ProjectSwitcherButton` does:\n  {}",
            untrapped.join("\n  ")
        );

        // Without this the test passes trivially the day someone renames the constructors
        // or moves them behind a helper — which is precisely when it would be needed.
        assert!(
            checked >= 12,
            "expected to find at least the 12 known popovers, found {checked}; the \
             constructor names in POPOVERS have probably drifted and this test is now \
             checking nothing"
        );
    }
}

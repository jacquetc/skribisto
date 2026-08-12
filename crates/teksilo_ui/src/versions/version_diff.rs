// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What changed between two versions of one row's prose.
//!
//! A support module, not a view-model: no signals, no widgets, no I/O. It takes
//! two Djot strings and answers "what moved, and where" — so it can be tested
//! headlessly, which is the only way the risky parts here get proven at all.
//!
//! ## Two levels, because a rewrite reorders paragraphs
//!
//! A single word-level pass over a whole scene interleaves fragments of unrelated
//! sentences the moment paragraphs move: Myers finds the longest common
//! subsequence, and after a reorder that subsequence is a scatter of "the", "and"
//! and full stops. So the first pass is over **blocks** (Patience, which pins
//! unique lines and is the algorithm built for exactly this), and only inside a
//! pair of blocks that Patience already decided correspond does a **word** pass
//! run.
//!
//! ## Plain text, never Djot source
//!
//! Both sides are reduced to their extracted plain text before anything is
//! compared. Diffing markup would report `*lamp*` → `_lamp_` as a changed word and
//! would put hunk boundaries in the middle of a `[link](target)`, in a coordinate
//! space no consumer shares. The cost is stated rather than hidden: **a change
//! that is purely formatting produces no textual difference**, so it is detected
//! separately and reported as such (`DiffSummary::formatting_only`) rather than
//! shown as an empty diff under a row that claims to have changed.
//!
//! ## Cleaning up coincidental matches
//!
//! `similar` ships a `semantic_cleanup` pass, and it is reachable only from
//! `iter_inline_changes`, whose output shape is the unified-diff line pair
//! (*old line with its removals emphasised*, then *new line with its additions*).
//! This pane renders the shape a novelist already knows instead — one paragraph,
//! removals struck through and additions underlined, the way track changes reads —
//! so that pass cannot be borrowed and `absorb_islands` does the job it exists
//! to do: a one- or two-word island of "unchanged" text stranded between two
//! changes is not a match, it is a coincidence, and rendering it as agreement
//! shreds both sentences into confetti.
//!
//! ## Rendering
//!
//! Output is ordinary Djot with `{+…+}` and `{-…-}` — which the shipped parser
//! already maps to underline and strikeout (`content_parser.rs:2372-2375`), so the
//! pane is a plain `RichTextEditor::read_only`
//! over a normal document: no new widget, no character-format plumbing. Shape
//! rather than colour also satisfies WCAG G182/G183 without further work.
//!
//! Every piece of prose is **escaped before it is spliced in**. Novels are full of
//! asterisks, brackets, braces and backslashes, and splicing them raw into a
//! synthesised document reparses them as markup — corrupting precisely the text
//! the pane exists to show. `render_block` round-trips through the real parser
//! in its own tests rather than trusting the escape table by eye.

use similar::{Algorithm, ChangeTag, DiffOp, TextDiff, capture_diff_slices};

use teksilo::text_document::{DjotImportOptions, djot_to_plain_text};

/// Below this word-level similarity, two blocks Patience paired are reported as a
/// replacement rather than as an edit.
///
/// The judgement being made is "would a reader recognise the second paragraph as a
/// revision of the first". Under about a third of the words in common the answer
/// is no, and an interleaved word diff of two unrelated sentences is strictly
/// worse than saying *this went, that came*.
const MIN_PAIR_RATIO: f32 = 0.35;

/// An "unchanged" island of at most this many words, stranded between two changes,
/// is absorbed into both sides instead of splitting them.
const MAX_EQUAL_ISLAND_WORDS: usize = 2;

/// A block has to be at least this many words before an exact match elsewhere is
/// read as a move rather than as a coincidence. Two scenes both containing
/// `"* * *"` have not moved anything.
const MIN_MOVE_WORDS: usize = 3;

/// How long one paragraph's word pass may take before `similar` settles for a
/// coarser answer.
const WORD_PASS_BUDGET: std::time::Duration = std::time::Duration::from_millis(200);

// ── the shape of an answer ─────────────────────────────────────────────────────

/// What one run of text did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunOp {
    Equal,
    Insert,
    Delete,
}

/// A stretch of text that shares one verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    pub op: RunOp,
    pub text: String,
}

/// What one block did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    /// Present, identical, in the same place.
    Equal,
    /// Wholly new.
    Insert,
    /// Wholly gone.
    Delete,
    /// Recognisably the same paragraph, edited — see [`DiffBlock::runs`].
    Changed,
    /// Identical text that appears somewhere else in the other version. Rendered
    /// unmarked (it did not change) and counted in the summary (it did move).
    Moved,
}

/// One block of the rendered comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffBlock {
    pub kind: BlockKind,
    /// For [`BlockKind::Changed`], the interleaved word runs. For every other kind
    /// a single run carrying the whole block.
    pub runs: Vec<Run>,
}

impl DiffBlock {
    /// The text this block contributes to the rendered document — additions and
    /// unchanged text, never deletions, are what the *newer* version reads as.
    fn plain(&self) -> String {
        self.runs.iter().map(|r| r.text.as_str()).collect()
    }
}

/// The one-line answer, for a reader who will not study the pane.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DiffSummary {
    pub words_added: usize,
    pub words_removed: usize,
    pub blocks_moved: usize,
    pub words_before: usize,
    pub words_after: usize,
    /// A few words of unchanged text next to the first change, so the summary can
    /// say *where* — "near the garden gate" — for a reader who is scanning.
    pub anchor: Option<String>,
    /// The two versions carry different markup but exactly the same text.
    ///
    /// Recorded rather than swallowed: the timeline lists a version because its
    /// stored bytes changed, so a version that italicised one word would otherwise
    /// open onto an empty diff under a row insisting something happened.
    pub formatting_only: bool,
}

/// Two versions of one row's prose, compared.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VersionDiff {
    pub blocks: Vec<DiffBlock>,
    pub summary: DiffSummary,
}

impl VersionDiff {
    /// Nothing textual moved. May still be a `DiffSummary::formatting_only`
    /// change.
    pub fn is_empty(&self) -> bool {
        !self.blocks.iter().any(|b| b.kind != BlockKind::Equal)
    }

    /// How much of the row changed, as a fraction in `0.0..=1.0`.
    ///
    /// Answers "is this worth opening" for a list row and nothing more.
    /// Deliberately **not** built on `analysis::repetition`'s shingle-Jaccard,
    /// which answers a different question (are two *scenes* near-duplicates), says
    /// in its own docs that plain Jaccard is the wrong measure there, and returns
    /// nothing at all below thirty distinct shingles — silently blanking exactly
    /// the short notes and early drafts a novelist edits hardest.
    pub fn magnitude(&self) -> f32 {
        let moved = self.summary.words_added + self.summary.words_removed;
        if moved == 0 {
            return 0.0;
        }
        let scale = self
            .summary
            .words_before
            .max(self.summary.words_after)
            .max(1);
        (moved as f32 / scale as f32).clamp(0.0, 1.0)
    }
}

// ── comparing ──────────────────────────────────────────────────────────────────

/// Compare two Djot documents.
///
/// `before` may be empty, which is how a row's first recorded state reads: every
/// block is an insertion.
pub fn diff_djot(before: &str, after: &str) -> VersionDiff {
    let old = blocks(before);
    let new = blocks(after);

    let mut pending = pair_blocks(&old, &new);
    let moved = mark_moves(&mut pending);

    let blocks: Vec<DiffBlock> = pending.into_iter().map(Pending::into_block).collect();

    let mut summary = DiffSummary {
        blocks_moved: moved,
        words_before: old.iter().map(|b| word_count(b)).sum(),
        words_after: new.iter().map(|b| word_count(b)).sum(),
        ..Default::default()
    };
    for block in &blocks {
        for run in &block.runs {
            match run.op {
                RunOp::Insert => summary.words_added += word_count(&run.text),
                RunOp::Delete => summary.words_removed += word_count(&run.text),
                RunOp::Equal => {}
            }
        }
    }
    summary.anchor = anchor_for(&blocks);
    // Same text, different bytes: the only remaining explanation is markup.
    summary.formatting_only =
        summary.words_added == 0 && summary.words_removed == 0 && moved == 0 && before != after;

    VersionDiff { blocks, summary }
}

/// One block's plain text, in document order.
///
/// Empty blocks are dropped. A block with no characters (an empty code fence) has
/// no prose to compare, and keeping it would make the rendered document's block
/// count disagree with its own text — two adjacent blank lines in Djot are one
/// separator, so every offset after such a block would be wrong.
fn blocks(djot: &str) -> Vec<String> {
    if djot.trim().is_empty() {
        return Vec::new();
    }
    djot_to_plain_text(djot, &DjotImportOptions::default())
        .split('\n')
        .filter(|b| !b.is_empty())
        .map(str::to_string)
        .collect()
}

/// A block awaiting the move pass, which needs to see the whole document.
#[derive(Debug, Clone)]
enum Pending {
    Equal(String),
    Insert(String),
    Delete(String),
    Moved(String),
    Changed(Vec<Run>),
}

impl Pending {
    fn into_block(self) -> DiffBlock {
        let one = |op, text| DiffBlock {
            kind: match op {
                RunOp::Equal => BlockKind::Equal,
                RunOp::Insert => BlockKind::Insert,
                RunOp::Delete => BlockKind::Delete,
            },
            runs: vec![Run { op, text }],
        };
        match self {
            Pending::Equal(t) => one(RunOp::Equal, t),
            Pending::Insert(t) => one(RunOp::Insert, t),
            Pending::Delete(t) => one(RunOp::Delete, t),
            Pending::Moved(t) => DiffBlock {
                kind: BlockKind::Moved,
                runs: vec![Run {
                    op: RunOp::Equal,
                    text: t,
                }],
            },
            Pending::Changed(runs) => DiffBlock {
                kind: BlockKind::Changed,
                runs,
            },
        }
    }
}

/// The block-level pass: Patience over whole blocks, then a word pass inside each
/// pair it believes corresponds.
fn pair_blocks(old: &[String], new: &[String]) -> Vec<Pending> {
    let mut out = Vec::new();
    for op in capture_diff_slices(Algorithm::Patience, old, new) {
        match op {
            DiffOp::Equal { new_index, len, .. } => {
                for b in &new[new_index..new_index + len] {
                    out.push(Pending::Equal(b.clone()));
                }
            }
            DiffOp::Delete {
                old_index, old_len, ..
            } => {
                for b in &old[old_index..old_index + old_len] {
                    out.push(Pending::Delete(b.clone()));
                }
            }
            DiffOp::Insert {
                new_index, new_len, ..
            } => {
                for b in &new[new_index..new_index + new_len] {
                    out.push(Pending::Insert(b.clone()));
                }
            }
            DiffOp::Replace {
                old_index,
                old_len,
                new_index,
                new_len,
            } => {
                // Paired by position within the replaced region, which is what a
                // paragraph edited in place looks like. Anything the pair gate
                // rejects falls through to *this went, that came* — and the move
                // pass below still gets a chance at it.
                let paired = old_len.min(new_len);
                for i in 0..paired {
                    let (o, n) = (&old[old_index + i], &new[new_index + i]);
                    match changed_runs(o, n) {
                        Some(runs) => out.push(Pending::Changed(runs)),
                        None => {
                            out.push(Pending::Delete(o.clone()));
                            out.push(Pending::Insert(n.clone()));
                        }
                    }
                }
                for b in &old[old_index + paired..old_index + old_len] {
                    out.push(Pending::Delete(b.clone()));
                }
                for b in &new[new_index + paired..new_index + new_len] {
                    out.push(Pending::Insert(b.clone()));
                }
            }
        }
    }
    out
}

/// The word-level pass, or `None` when the two blocks are too unalike to be read
/// as one paragraph edited.
fn changed_runs(before: &str, after: &str) -> Option<Vec<Run>> {
    let mut config = TextDiff::configure();
    // Whitespace-delimited words rather than `unicode` word bounds: the latter
    // splits `l'aube` into three tokens, so a French elision would highlight a
    // fragment of a word. Coarser is more legible here, and every token still
    // concatenates back to the source exactly.
    config.algorithm(Algorithm::Patience);
    // A ceiling, not a target. Patience over a long scene is milliseconds, but the
    // input here is whatever a novelist wrote, and a pathological paragraph must
    // degrade to a coarser answer rather than stall the pass that computes every
    // row's magnitude.
    config.timeout(WORD_PASS_BUDGET);
    let diff = config.diff_words(before, after);

    let mut runs: Vec<Run> = Vec::new();
    let (mut same, mut total) = (0usize, 0usize);
    for change in diff.iter_all_changes() {
        let text = change.value();
        let words = word_count(text);
        let op = match change.tag() {
            ChangeTag::Equal => {
                same += words;
                total += 2 * words;
                RunOp::Equal
            }
            ChangeTag::Delete => {
                total += words;
                RunOp::Delete
            }
            ChangeTag::Insert => {
                total += words;
                RunOp::Insert
            }
        };
        push_run(&mut runs, op, text);
    }

    // Over *words*, not over tokens: whitespace tokens always match, so a token
    // ratio would score two unrelated paragraphs of similar length at ~0.5 and
    // wave every replacement through the gate.
    let ratio = if total == 0 {
        1.0
    } else {
        2.0 * same as f32 / total as f32
    };
    if ratio < MIN_PAIR_RATIO {
        return None;
    }
    absorb_islands(&mut runs);
    Some(runs)
}

/// Append to the last run when it agrees, otherwise start a new one.
fn push_run(runs: &mut Vec<Run>, op: RunOp, text: &str) {
    if text.is_empty() {
        return;
    }
    match runs.last_mut() {
        Some(last) if last.op == op => last.text.push_str(text),
        _ => runs.push(Run {
            op,
            text: text.to_string(),
        }),
    }
}

/// Collapse each divergence into one deletion followed by one insertion, absorbing
/// short "unchanged" islands between two changes.
///
/// The islands are the whole point. A word diff of *she opened the gate* against
/// *he shut a door* matches all three spaces and nothing else, and rendering that
/// faithfully produces four struck-out fragments interleaved with four underlined
/// ones — technically correct and unreadable. Anything up to
/// [`MAX_EQUAL_ISLAND_WORDS`] with a change on both sides of it is treated as the
/// coincidence it is.
fn absorb_islands(runs: &mut Vec<Run>) {
    let mut out: Vec<Run> = Vec::new();
    let (mut deleted, mut inserted) = (String::new(), String::new());

    let flush = |out: &mut Vec<Run>, deleted: &mut String, inserted: &mut String| {
        if !deleted.is_empty() {
            push_run(out, RunOp::Delete, deleted);
            deleted.clear();
        }
        if !inserted.is_empty() {
            push_run(out, RunOp::Insert, inserted);
            inserted.clear();
        }
    };

    for i in 0..runs.len() {
        match runs[i].op {
            RunOp::Delete => deleted.push_str(&runs[i].text),
            RunOp::Insert => inserted.push_str(&runs[i].text),
            RunOp::Equal => {
                let stranded = !deleted.is_empty()
                    && !inserted.is_empty()
                    && word_count(&runs[i].text) <= MAX_EQUAL_ISLAND_WORDS
                    && runs[i + 1..].iter().any(|r| r.op != RunOp::Equal);
                if stranded {
                    // It belongs to both readings, so it is shown in both.
                    deleted.push_str(&runs[i].text);
                    inserted.push_str(&runs[i].text);
                } else {
                    flush(&mut out, &mut deleted, &mut inserted);
                    push_run(&mut out, RunOp::Equal, &runs[i].text);
                }
            }
        }
    }
    flush(&mut out, &mut deleted, &mut inserted);
    *runs = out;
}

/// Re-read whole-block deletions and insertions that carry identical text as one
/// block that moved. Returns how many.
///
/// Patience keeps the longest consistent ordering, so a paragraph dragged
/// elsewhere arrives here as a delete in one place and an insert in another. Left
/// alone it reads as *this paragraph was destroyed and an identical one written* —
/// which is not what happened, and is alarming in a feature about not losing work.
fn mark_moves(pending: &mut [Pending]) -> usize {
    let mut moved = 0;
    loop {
        let insert = pending.iter().position(|p| match p {
            Pending::Insert(t) => word_count(t) >= MIN_MOVE_WORDS,
            _ => false,
        });
        let Some(insert) = insert else { break };
        let Pending::Insert(text) = pending[insert].clone() else {
            break;
        };
        let delete = pending
            .iter()
            .position(|p| matches!(p, Pending::Delete(d) if *d == text));
        let Some(delete) = delete else {
            // Not a move. Settle it as a block so the next pass does not find it
            // again — `Moved` and `Insert` render identically apart from the mark.
            pending[insert] = Pending::Changed(vec![Run {
                op: RunOp::Insert,
                text,
            }]);
            continue;
        };
        pending[insert] = Pending::Moved(text);
        pending[delete] = Pending::Equal(String::new());
        moved += 1;
    }
    // Restore the insertions that were parked as single-run `Changed` blocks, and
    // drop the emptied deletions.
    for slot in pending.iter_mut() {
        if let Pending::Changed(runs) = slot
            && runs.len() == 1
            && runs[0].op == RunOp::Insert
        {
            *slot = Pending::Insert(runs[0].text.clone());
        }
    }
    moved
}

/// A few words of unchanged text beside the first change, for the summary line.
fn anchor_for(blocks: &[DiffBlock]) -> Option<String> {
    let block = blocks.iter().find(|b| b.kind == BlockKind::Changed)?;
    let first_change = block.runs.iter().position(|r| r.op != RunOp::Equal)?;
    let before = block.runs[..first_change]
        .iter()
        .rev()
        .find(|r| word_count(&r.text) > 0);
    let text = match before {
        Some(run) => tail_words(&run.text, 4),
        // The change opens the paragraph, so the words after it are the landmark.
        None => head_words(&block.runs[first_change].text, 4),
    };
    (!text.is_empty()).then_some(text)
}

fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn tail_words(text: &str, n: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    words[words.len().saturating_sub(n)..].join(" ")
}

fn head_words(text: &str, n: usize) -> String {
    text.split_whitespace()
        .take(n)
        .collect::<Vec<_>>()
        .join(" ")
}

// ── rendering ──────────────────────────────────────────────────────────────────

/// How much unchanged text to keep around each change.
///
/// A scene is mostly unchanged by definition, and a pane that opens on three
/// thousand untouched words has answered the wrong question. The named complaint
/// about the equivalent feature elsewhere is that a change is "difficult to spot".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollapseRule {
    /// Unchanged blocks kept on each side of a change.
    pub context: usize,
    /// Shortest run of unchanged blocks worth hiding at all.
    pub min_run: usize,
}

impl Default for CollapseRule {
    fn default() -> Self {
        Self {
            context: 1,
            min_run: 3,
        }
    }
}

/// A rendered comparison, plus where its changes are.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Rendered {
    /// Ordinary Djot, ready for `TextDocument::set_djot_sync`.
    pub djot: String,
    /// Character offset, in the rendered document's own addressable text, of the
    /// first character of each changed block — what jump-to-next-change moves the
    /// cursor to. In the same order the blocks appear.
    pub change_offsets: Vec<usize>,
}

/// Render a comparison as Djot.
///
/// `elide` formats the placeholder that stands in for hidden unchanged blocks; it
/// is passed the number hidden. Taking it as a callback keeps this module free of
/// the localisation layer, so its tests can assert on the structure.
pub fn render(
    diff: &VersionDiff,
    collapse: Option<CollapseRule>,
    elide: &dyn Fn(usize) -> String,
) -> Rendered {
    let keep = match collapse {
        Some(rule) => visible_blocks(diff, rule),
        None => diff.blocks.iter().map(|_| Visibility::Show).collect(),
    };

    let mut out = Rendered::default();
    let mut offset = 0usize;
    let mut hidden = 0usize;

    let emit = |djot: &str, plain: &str, out: &mut Rendered, offset: &mut usize| {
        if !out.djot.is_empty() {
            out.djot.push_str("\n\n");
            *offset += 1; // the document joins blocks with a single newline
        }
        out.djot.push_str(djot);
        *offset += plain.chars().count();
    };

    for (block, visibility) in diff.blocks.iter().zip(keep) {
        match visibility {
            Visibility::Hide => {
                hidden += 1;
                continue;
            }
            Visibility::Show => {}
        }
        if hidden > 0 {
            let label = elide(hidden);
            emit(&render_block_text(&label), &label, &mut out, &mut offset);
            hidden = 0;
        }
        let plain = block.plain();
        if block.kind != BlockKind::Equal {
            // Recorded before the separator is added for the *next* block, so it
            // points at this block's first character.
            let start = if out.djot.is_empty() {
                offset
            } else {
                offset + 1
            };
            out.change_offsets.push(start);
        }
        emit(&render_block(block), &plain, &mut out, &mut offset);
    }
    if hidden > 0 {
        let label = elide(hidden);
        emit(&render_block_text(&label), &label, &mut out, &mut offset);
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Visibility {
    Show,
    Hide,
}

/// Which blocks survive the collapse rule.
fn visible_blocks(diff: &VersionDiff, rule: CollapseRule) -> Vec<Visibility> {
    let mut keep = vec![Visibility::Show; diff.blocks.len()];
    let mut run_start: Option<usize> = None;
    for i in 0..=diff.blocks.len() {
        let unchanged = diff
            .blocks
            .get(i)
            .is_some_and(|b| b.kind == BlockKind::Equal);
        match (unchanged, run_start) {
            (true, None) => run_start = Some(i),
            (false, Some(start)) => {
                hide_middle(&mut keep, start, i, rule, start > 0, i < diff.blocks.len());
                run_start = None;
            }
            _ => {}
        }
    }
    keep
}

/// Hide the middle of the unchanged run `start..end`, keeping context on whichever
/// sides actually border a change.
fn hide_middle(
    keep: &mut [Visibility],
    start: usize,
    end: usize,
    rule: CollapseRule,
    change_before: bool,
    change_after: bool,
) {
    let lead = if change_before { rule.context } else { 0 };
    let trail = if change_after { rule.context } else { 0 };
    let len = end - start;
    if len <= lead + trail || len - lead - trail < rule.min_run {
        return;
    }
    for slot in keep.iter_mut().take(end - trail).skip(start + lead) {
        *slot = Visibility::Hide;
    }
}

/// One block as Djot: every run escaped, additions in `{+…+}`, deletions in
/// `{-…-}`.
fn render_block(block: &DiffBlock) -> String {
    let mut line = String::new();
    for run in &block.runs {
        let escaped = escape_inline(&run.text);
        match run.op {
            RunOp::Equal => line.push_str(&escaped),
            RunOp::Insert => {
                line.push_str("{+");
                line.push_str(&escaped);
                line.push_str("+}");
            }
            RunOp::Delete => {
                line.push_str("{-");
                line.push_str(&escaped);
                line.push_str("-}");
            }
        }
    }
    guard_block_start(line)
}

/// A block of plain text with no marks — the elision placeholder.
fn render_block_text(text: &str) -> String {
    guard_block_start(escape_inline(text))
}

/// Backslash-escape everything that could start Djot *inline* markup, so arbitrary
/// prose survives a reparse verbatim.
///
/// The first group is the inline-markup set the shipped Djot exporter escapes
/// (`document_io::export_djot_uc::escape_djot`). The second — `.`, `-`, `'`, `"` —
/// is this renderer's own, and is needed because the exporter has a weaker
/// obligation than this does: it emits from a model whose smart punctuation was
/// already resolved, whereas this splices back text that has *already been through*
/// the parser once. Without it a stretch reading `...` would re-enter as `…` and
/// the pane would show a change nobody made.
///
/// Over-escaping is always safe: a backslash before any ASCII punctuation is a
/// Djot escape yielding that character literally.
fn escape_inline(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' | '*' | '_' | '`' | '~' | '^' | '[' | ']' | '(' | ')' | '{' | '}' | '|' | '<'
            | '.' | '-' | '\'' | '"' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

/// Neutralise a line's leading characters so they are not read as a block marker.
///
/// [`escape_inline`] already covers the inline set; this covers the block-only
/// markers (`#`, `>`, `+`, `:`) and the ordered-list forms `<digits>.` and
/// `<digits>)`. It runs *after* the `{+`/`{-` marks are placed, so it also sees a
/// line that opens with one of them.
fn guard_block_start(line: String) -> String {
    let Some(first) = line.chars().next() else {
        return line;
    };
    if matches!(first, '#' | '>' | '+' | ':') {
        return format!("\\{line}");
    }
    if first.is_ascii_digit() {
        let rest = line.trim_start_matches(|c: char| c.is_ascii_digit());
        if rest.starts_with('.') || rest.starts_with(')') {
            let digits = line.len() - rest.len();
            // Escape the delimiter, not the digit: a backslash before a digit is
            // literal, so `\1.` would render as `\1.`.
            return format!("{}\\{}", &line[..digits], &line[digits..]);
        }
    }
    line
}

#[cfg(test)]
mod tests;

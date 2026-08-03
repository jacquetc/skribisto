# Analysis tab — design note

Brainstorm only. No implementation. Branch `feat/analysis-tab`.

Ground truth gathered against the real tree (worktree of `dev` @ `ceddd81c`), plus a
literature/stack review. Everything below cites what it rests on.

---

## 0. Calibration facts

From the bundled reference manuscript `resources/examples/Starforgers.skrib`:

| | |
|---|---|
| Words | 85,210 |
| Prose items | 36 scenes (37 binder items) |
| Mean scene length | 2,367 words |
| Synopses | **0** |
| Tags | **0** |
| `aliases` populated | **0** |
| `paces.ron` | `[]` (empty) |

Two consequences that matter more than they look:

1. **Every "is O(n²) acceptable?" question is moot.** 36 scenes → 630 pairs. Even a
   pathological 500-scene project is 125k pairs, sub-second. Correctness of Djot
   stripping matters ~100× more than algorithmic complexity here.
2. **The flagship example would render half the proposed analyses empty.** Anything
   keyed on synopsis, tag or alias shows nothing in the one project a new user opens.
   Fix the fixture alongside the feature, or the feature demos as broken.

Prose is stored as **raw Djot** in `Content.data` (`crates/common/src/entities.rs:405`) —
`_“Senator Constantine…”_`, `## Chapter 6`, `- ` lists. Analysis must run
`text_document::djot_to_plain_text` **once per scene** and reuse the result across all
metrics, mirroring `skribisto_model::counting`'s content-addressed cache. Counting words on
raw Djot counts syntax; counting sentences on it counts headings.

---

## 1. Three prerequisites the proposal assumes and the codebase does not have

### 1.1 POV does not exist. At all.

Verified across the whole model:

- `BinderItem` (`crates/common/src/entities.rs:336-359`) — `id, created_at, updated_at,
  uid, title, sub_title, role, sub_role, label, activated, is_favorite, is_exportable,
  indent, word_count_goal, char_count_goal, dict_language, aliases, contents, references,
  tags`. No POV.
- `Content` (`entities.rs:405-421`) — no POV.
- `BinderTag` (`entities.rs:386-403`) — `name, color, details, discoverable`. Free-form,
  **untyped, unnamespaced**, duplicate-name-tolerant by design
  (`crates/tag_management/src/use_cases/import_tags_uc.rs:12-16`).
- No `Character` entity anywhere.

So **features #5 and #6 are not "hard", they are unbuildable as written.** They rest on a
concept that has to be designed first.

The honest options, cheapest first:

| Option | Cost | Verdict |
|---|---|---|
| Reserved `POV:` tag-name prefix, enforced only in the analysis layer | Zero backend | Works, but it is a convention masquerading as a schema — nothing stops 0 or 2 POV tags per scene |
| **Promote one `BinderItem.references` entry to POV** | Manifest edit + regen + Inspector UI | **Recommended.** Cast/Présence already ships (`crates/bastyde_ui/src/docks/inspector.rs:300`) and already answers "who is in this scene". POV is "which of them is the camera" |
| New `Character` entity | Large | Overkill |

Either way: **POV is a feature in its own right, and a good one independent of analysis**
(a POV column in Overview, a POV filter on the corkboard, POV in the compile stream). It
should not be smuggled in as an analysis prerequisite. Design it separately, then the
analyses become easy.

Note the mention scanner's own code already knows presence ≠ POV
(`crates/mention_management/src/use_cases/scan_mentions_uc.rs:250-252`: *"a scene told in
deep POV may name nobody"*) — which is exactly why hand-pinned references get unioned in.

### 1.2 The sentence splitter is private — but the fix is small

> **Corrected in round 2.** I first assessed this as potentially open-ended. It isn't.

`text-document` already has a **fully pure** splitter:
`pub(crate) fn sentence_bounds(text: &str, char_offset: usize, locale: Option<&str>)
-> Option<(usize, usize)>` at `crates/public_api/src/sentence.rs:411`. It has **zero**
`TextDocument` coupling — both callers (`TextDocument::sentence_at`,
`TextCursor::find_sentence_boundaries`) do all the document work themselves and hand it a
bare `&str`. It's `pub(crate)` only because nothing outside needed it yet.

It also carries a private **~40-language `PROFILES` tailoring table** and does **not** use
ICU — so the "ICU sentence data is Greek-only" note does not apply here. The locale
parameter is genuinely load-bearing.

It was authored in commit `0dc3d66`, alongside the caret-highlight feature — i.e. very
recently. So this is a visibility/plumbing exercise, not a redesign.

This blocks sentence-length variance (#4) and the sentence-length strand of #8. It does
*not* block paragraph length or punctuation density, which need no splitter.

### 1.3 There is no dialogue detector — but the hard half already exists

Zero hits for `dialogue` in `crates/` outside the typing-time smart-punctuation toggle. But
`crates/bastyde_ui/src/text_replacement/typography.rs:225` already carries a `RULESETS`
table — 15 rows, 13 base languages (en, fr, de/de-CH, es, ca, it, pt/pt-BR, nl, pl, ru, sv,
tr, ar) — with `primary_quotes` and `dialogue_dash` per locale. **Query that table; do not
invent a second glyph set**, or the two features will silently disagree about what a quote is.

That also fixes the honest language boundary: dialogue ratio is defensible for exactly
those 13 languages and must show an explicit unsupported state elsewhere, not a
silently-wrong number.

### 1.4 There is no batched "this Book's scenes with text" read

No single function does it. It assembles from `skribisto_compiler::item_metas()` +
`skrib_format::gather` + `djot_to_plain_text` + `skribisto_model::scene_break::strip_markers_djot`.

Two traps:
- `skribisto_model::compile::resolve_scope(…, ScopeKind::Book)` **silently drops
  non-exportable items** via `push_swept` (`compile.rs:254`). An "analyze everything I
  wrote" feature does not want the export filter.
- Do **not** route through `StreamViewModel` — it opens a real `TextDocument` per row and is
  the documented cause of a freeze. Copy `OverviewRowsModel`'s batched `get_content_multi`
  pattern instead (`crates/bastyde_ui/src/models/overview_rows_model.rs:780-870`).

Also: the `NoteBook` template (`crates/work_management/src/use_cases/new_work_uc/templates.rs:211-226`)
creates **no `Folder/Book` row at all**, and there is no persisted template-kind field. So
"per Book scope" needs a real detect-and-degrade path, not an assumption.

---

## 2. Feature-by-feature

| # | Feature | Verdict | Reason |
|---|---|---|---|
| 3 | Orphaned note finder | **Ship first — reframed** | `ScanMentions` already does folded, whole-word, alias-aware matching and already unions hand-pinned refs. This is a *view*, not an algorithm. But "orphaned" pathologises legitimate worldbuilding notes — reframe as **mention coverage / dropped threads** |
| 7 | Chapter/Part balance | **Ship first** | Reuse `OverviewRowsModel`'s cached counts + `bastyde-charts` `BarChart`. Near-free |
| 8a | Word count along the stream | **Ship first** | Same counts, positional layout |
| 2 | Near-duplicate scenes | **Ship — measure changed** | Sound, but **Jaccard is the wrong metric**. A 500-word scene fully inside a 3,000-word one scores Jaccard ≈ 0.17 (looks unrelated) and containment ≈ 1.0. Containment is the "I expanded this scene" detector; keep Jaccard as a secondary column |
| 4a | Paragraph length, punctuation density | **Ship** | No splitter needed |
| 1 | Exact repeated blocks (suffix array) | **Ship third, reshaped** | See §3 — right technique, wrong priority, and the stated algorithm returns the wrong object |
| 5 | Vocabulary richness per POV | **Reshape** | POV blocked (§1.1). And **raw TTR is invalid as specified** — see §4 |
| 4b/8b | Sentence-length metrics | **Defer** | Blocked on §1.2 |
| 8c | Dialogue ratio | **Defer** | Blocked on §1.3 (small once unblocked) |
| 6 | Voice-drift (Burrows's Delta) | **Cut** | See §4. Triple-blocked and not repairable by threshold tuning |
| 9 | Paraphrase duplicate detector (embedding) | **Cut for now** | See §5 |
| 10 | Foreshadowing matcher (embedding) | **Cut** | No benchmark, no eval set, unfalsifiable as specified |

---

## 3. The repetition feature is aimed at the rarer problem

This is my own editorial call, and I think it's the most useful correction in this note.

A suffix array finds **long exact verbatim blocks**. That catches a real bug — pasting a
paragraph twice — but it's a *rare accident*. The common craft flaw is the **echo**: the
same distinctive word landing twice within a few hundred words. *"He glanced at the door.
She glanced back."* ProWritingAid's Repeats/Echoes is its most-praised report for exactly
this reason; its Sticky Sentences report is its most-criticised, because it flags
deliberately dense prose as a defect.

Note the design pass's own mockup drifted to `"glanced" appears 14 times in Chapter 6` —
that's **word frequency in a window**, not longest-repeated-substring. The mockup was right
and the algorithm spec was aiming elsewhere.

So: one **Repetition** module, one shared tokenizer, three views, in this build order:

1. **Echoes** — distinctive word repeated within an N-word window. Rank by
   `−log p(word)` over the manuscript's own frequency table so "the" never surfaces.
   Cheapest to build, most often useful.
2. **Similar scenes** — shingle containment (§2).
3. **Repeated passages** — the suffix array. Most expensive, least often triggered.

And when you do build #3, the spec needs four corrections:

- **Wrong object.** Classic LRS is `argmax(LCP)` — one match. You want **supermaximal
  repeats** (Gusfield §7.12; Abouelhoda/Kurtz/Ohlebusch 2004), deduped, plus an orthogonal
  greedy non-overlap pass on occurrence positions (`"the the the"` self-overlaps).
- **Wrong alphabet.** Byte-level SA matches mid-codepoint and mid-word (`"into the door"`
  spanning `doorway`). Tokenise to case-folded word IDs (`u32`) first; keep original spans
  for display.
- **No precision mechanism at all.** As stated it drowns in idiom. Floor: ≥4 tokens, ≥2
  content words, ≥2 non-overlapping occurrences, not confined to one scene. Rank by
  surprisal, not length.
- **Refrains are intentional.** Needs a dismiss affordance, keyed by content, not position.

---

## 4. The stylometry is outside its regime

### Vocabulary richness — repairable, but not as specified

Raw TTR is not "somewhat" length-dependent. Heaps'/Herdan's law makes it a near-deterministic
decreasing function of N: `V ≈ K·N^β` with β≈0.4–0.6 for prose, so `TTR ≈ K·N^(β−1)`.
Comparing a 40,000-word POV to a 12,000-word POV is reading two points on a curve whose
slope's sign you know before looking at a single word — **the longer one is guaranteed to
score lower even if the vocabularies are statistically identical.** Hapax rate is *worse*:
it decays faster still, since every repeat converts a hapax into a non-hapax.

Fix (small): **MATTR** (moving-average TTR, ~50-token window, O(N), no dependency) as the
primary figure, optionally **MTLD** as a secondary — McCarthy & Jarvis (2010,
*Behavior Research Methods* 42:381-392) recommend reporting more than one index. Drop
standalone hapax rate. Floor at ~500–1,000 words with a "not enough text yet" state.

Additional honesty problem: for morphologically rich European languages (Polish, Czech,
Finnish, German compounds) surface-form TTR without lemmatisation measures inflection, not
vocabulary. `rust-stemmers` blunts but does not solve this. Scope the copy to
within-book, within-language, and present it descriptively — never a cross-POV leaderboard.

### Voice-drift (Delta) — cut it

Three independent blockers:

1. **POV doesn't exist** (§1.1).
2. **No function-word lists exist.** Zero hits for `stopword|function_word` across `crates/`.
   Curated, licensed lists across 13 languages is a linguistic-resource project, not a
   coding task.
3. **The sample sizes are wrong by a factor of 2–6.** Eder (2015, *DSH* 30(2):167-182,
   *"Does size matter?"*) puts the reliability floor at ~2,500 (Latin prose) to ~5,000 words
   for English/German/Polish/Hungarian novels, and found it largely method-independent.
   Starforgers averages **2,367 words per scene** — under half the floor.

And even unblocked, the mechanism is wrong for the question. Evert et al. show Delta's
discrimination is dominated by z-scored MFW vectors under L1 distance, standardised against
a corpus of *different authors*. A "reference corpus" of a few hundred same-author,
same-book scenes is degenerate — there is no cross-author variance for the mechanism to
lock onto. Worse, Biber (1988) identifies function-word frequency as the *primary carrier*
of dialogue-vs-narration register variation. So it will preferentially flag **ordinary
scene-type alternation** — action vs. reflection — and call it voice drift.

**The intent is legitimate; the instrument is not.** Better answers to "did I keep this
voice consistent", in ascending cost:

- **Tense/person consistency** — rule-based, zero statistical uncertainty, catches a real
  craft bug (a first-person POV slipping into "he", a past-tense scene drifting present).
  This is the honest replacement and it is *more* actionable than any Delta score.
- **Dialogue-vs-narration ratio per scene** — directly useful, and it is the confound
  control any future voice metric needs anyway, so it pays for itself twice.
- **Craig's Zeta** rather than Delta, if you ever want "how distinct do my POV voices
  sound" — Zeta was built for the two-corpus contrastive question; Delta was not.

---

## 5. Tier 1: the download is not earned yet

For **#9 (paraphrase duplicates)** the premise is backwards. Near-duplicate detection in the
high-similarity regime is the case where lexical methods (MinHash/shingling) remain the
practical standard; dense embeddings earn their keep when wording *diverges substantially*.
Putting #9 behind a 120 MB download assumes the opposite of what the field reports.

For **#10 (foreshadowing/Chekhov's gun)** there is no benchmark, no dataset, and no
published task. It is the single most speculative item on the list — and it's the one that
would justify the download. Planted story details are *named entities*, which is precisely
the anchoring where BM25 tends to match or beat dense retrieval, and Skribisto's
tag/reference graph is an app-specific signal generic literature can't account for.

**Do not build Tier 1 until Phase 2's lexical near-duplicate results have been manually
evaluated against real near-duplicate pairs and shown to miss real cases.** No such evidence
exists today. If you ever build #10, hand-label a small planted-detail → payoff eval set
*first* — otherwise you cannot tell a working feature from a plausible-looking one.

If it does eventually ship, two corrections to the stated plan:

- **The model choice is stale.** `multilingual-e5-small` scores **50.9** on MTEB multilingual
  retrieval. IBM's `granite-embedding-97m-multilingual-r2` (Apache-2.0, published May 2026,
  98 MB int8, 32K context, ships ONNX + OpenVINO natively) scores **60.3** — #1 under 100M
  params. Smaller *and* substantially better. Caveat: it is not in fastembed-rs's model
  table, so you'd drive `ort` + `tokenizers` directly rather than use the one-liner API.
- **The packaging hazard is real and doubled.** `ort` has been at `2.0.0-rc.N` for 2+ years
  (rc.13 as of Jul 2026) — an API-stability risk for a long-lived desktop app. Its
  `download-binaries` default feature hits the network at **`cargo build` time**, which
  breaks a stock offline `flatpak-builder` run unless pinned via `ORT_LIB_LOCATION` /
  `ORT_DYLIB_PATH`. Model weights are a **separate** runtime download to
  `.fastembed_cache` — solving one does not solve the other. Both must be pinned as manifest
  sources. Feature-gate it exactly like the existing opt-in `pdf`/Typst precedent.
- fastembed **never** auto-adds e5's `"query: "` / `"passage: "` prefixes. Omitting them
  doesn't error — it silently degrades retrieval. (Moot if you go Granite.)
- `jina-embeddings-v3` is **CC BY-NC** — ruled out for a redistributed app, and easy to miss
  since v2 was Apache-2.0.

---

## 6. The two best ideas are not on the list

Both exploit the thing no competitor has: **every writing row owns a synopsis *and* its
prose as first-class sibling fields.** Scrivener, ProWritingAid, AutoCrit, Fictionary,
bibisco and Manuskript all either treat the manuscript as flat text or make the writer
re-enter structure by hand (Fictionary asks for 38 elements per scene — criticised by its
own users for exactly that tedium).

1. **Synopsis-vs-prose drift.** Lexical overlap between what a scene *says it does* and what
   it *actually does*. Surfaces *"this scene has drifted from your outline"*. No surveyed
   tool can compute this at all. Essentially free once §1.4's batched read exists.
2. **Synopsis-length vs prose-length ratio outliers.** Near-equal lengths signal
   summary-not-scene (an info-dump). A one-line synopsis against a 4,000-word scene signals
   scope creep. Again free, again impossible for tools where the synopsis is a detached
   planning card.

A third, cheap: **run the Tier-0 pacing/balance metrics over the concatenated *synopsis*
stream** as well as the prose stream. "Full Synopsis" is already a shipped segment. This
lets a writer sanity-check plot shape at the compressed outline level — structurally
impossible where synopses live in a separate object graph.

These should be in Phase 1, not treated as stretch goals.

---

## 7. Architecture

**Where it lives.** A new **Book-only segment** in `folder_segmented`, beside Pace — same
category as Pace/Corkboard/Overview, all of which are segments. Not a dock (nothing scopes
dock content per open container tab today; `ContentTab` already scopes segments per tab).
Not a new `(role, sub_role)` tab module — that would violate the `tabs/` ↔ `COMBINATIONS`
1:1 invariant and its enforcing test, since Analysis is not a writing-model combination.

Two concrete costs:
- `folder_segmented`'s `extra: Option<(…)>` currently supports **one** Book-only extra
  segment. It needs widening.
- **The positional trap fires.** Segment indices are hardcoded in tests at
  [`tabs.rs:1172`](crates/bastyde_ui/src/tabs.rs#L1172) and
  [`:1229`](crates/bastyde_ui/src/tabs.rs#L1229). Inserting Analysis shifts Corkboard and
  Overview. A segment added without its child compiles fine and silently shows the previous
  view under the new label.

**Inside the segment**, a master/detail list rail — *not* a second `SegmentedControl`
(8 analyses is well past its documented 2–5 ceiling) and not one long scrolling report
(3 charts + 3 sortable lists is 2000px of unscannable page, and each finder needs its own
scroll/sort chrome inside the page's `ScrollArea` — the exact greedy-child-in-flowing-page
trap `search_preview.rs`'s tests exist to catch). The search dock → search-preview dock
split is already this pattern, shipping.

**Backend.** A new Qleany feature crate `analysis_management`, scaffolded like
`progress_management` / `mention_management`, whose use case implements `TreeReader` and
calls the shared `skrib_format::gather` — exactly like `count_words_uc` and
`scan_mentions_uc`. This satisfies "no use case calls a use case".

**Do not depend on `search_management`'s `corpus_cache`/`matching`.** They're `pub` but have
zero cross-crate dependents, they don't strip scene-break markers, and they're
fold-oriented rather than count-oriented. Depending on them would be the first
feature-crate-on-feature-crate dependency in the workspace.

Pure text primitives (sentence/paragraph stats, dialogue ratio, shingles, frequency) belong
in **`skribisto_model` as a new module** alongside `counting.rs` / `mentions.rs` — reads
nothing, writes nothing, knows only strings and ids. That's also what makes them headless-testable.

**Long op.** One dedicated OS thread per op (plain `std::thread::spawn`, no pool).
`QueryUnitOfWork` + `begin_frozen_read_transaction` gives an O(1) atomic snapshot, so
analysis reads a consistent manuscript while the user keeps typing. Cancellation is a
cooperative `Arc<AtomicBool>` — thread it into the per-item loop or Cancel will be ignored
until the next checkpoint. `rayon` 1.12 is already resolved in `Cargo.lock` but only linked
under the `pdf` feature; making it a direct dep is a real (if modest) new cost. At 85k words
you almost certainly don't need it.

**Cache.** The in-memory content-addressed pattern (`RwLock<Option<Store>>`, heap-budgeted,
wholesale-cleared on overflow and project close) is the blessed idiom but is **not persisted
across restarts**. `SettingsFile<T>` is TOML-only and cannot hold vectors. If persistence is
wanted, it's a genuinely new mechanism: a bespoke serde file under `AppPaths::data_dir()`
keyed by `Work.unique_id`, guarded by the existing `uid_is_usable()`.

**Dismissals** key by `BinderItem.uid` plus a content-derived key — never a store `EntityId`
(re-minted every `load_work`) and never a byte offset (invalidated by the next edit). This
codebase has already been burned once by ordinal-keyed persistence (`workspace.toml` v1).

---

## 8. Naming collision

**"Pace" already means writing-schedule planner** in Skribisto — goal, deadline, weekday
schedule, streak, snapshots, charts. Shipping a "pacing heat-strip" next to it in the same
segmented bar would be actively confusing. Call the new thing **Rhythm**, or **Shape**, or
fold it in as an unnamed strip inside Analysis.

---

## 9. Tone

These metrics can read as *"your writing is bad"*. Non-negotiables:

- **Report measurements, not verdicts.** Never "too many", "weak", "should", "problem".
- **Compare the manuscript only to itself** — its own median chapter, its own other scenes.
  Never imply a genre norm or a correct sentence length. (Genre-corpus benchmarking, which
  is AutoCrit's flagship, needs a licensed corpus of comparable published fiction — a
  non-starter without real data-licensing work.)
- **Never lead with a bare readability score.** Flesch-Kincaid on fiction is a documented
  validity complaint — dialogue skews it.
- Sentence case everywhere. Both locales, always. Data stays `lit!`.

Example strings:

> EN — "“glanced” appears 14 times in Chapter 6 — 3 more than its average across the book."
> FR — « glanced » apparaît 14 fois dans le chapitre 6 — 3 de plus que sa moyenne dans le reste du livre.

> EN — "This scene and “The Cellar, Again” share about 78% of their wording."
> FR — Cette scène et « The Cellar, Again » partagent environ 78 % de leur texte.

---

## 10. Sequencing

**Phase 1 — free wins on existing plumbing** *(~1–2 weeks; mostly UI + one shared read primitive)*
Mention coverage (over `ScanMentions`), chapter/part balance, word count along the stream,
per-Book vocabulary richness (MATTR, POV split dropped), **synopsis-vs-prose drift**,
**synopsis/prose length-ratio outliers**. Builds §1.4's batched read, which every later
phase needs.
*Proves:* the structural home, the one shared data primitive, and graceful degradation for
Book-less projects — three things every later phase would otherwise each get wrong.

**Phase 2 — repetition, pure Rust** *(~1–2 weeks; built as a cancellable long op from day one)*
Echoes → similar scenes (containment) → repeated passages (supermaximal repeats over word
tokens). Paragraph length + punctuation density.
*Proves:* whether near-duplicate detection is good enough without embeddings. **This
evaluation is the gate on Tier 1.**

**Phase 3 — the blocked metrics** *(~1–2 weeks in-repo; open-ended if it means a `text-document` change)*
Sentence splitter (needs your sign-off if it touches the other repo), then sentence-length
variance and the dialogue detector off `typography.rs`'s existing table — gated to its 13
languages with an explicit unsupported state elsewhere.
*Proves:* the multilingual honesty boundary is enforced in the UI, not just documented.

**Phase POV — independent, whenever you want it**
Not an analysis phase. Ship POV as a first-class writing-model feature; the POV-split
analyses then become trivial re-runs of Phase 1/3 metrics with a grouping key. Design in a
mandatory "unassigned" bucket — nothing stops a scene having zero or two POV markers.

**Phase 4 — Tier 1, conditional and opt-in** *(~3–4 weeks, dominated by packaging and consent UX, not ML)*
Only #9, only if Phase 2's evaluation shows a real gap. Granite-r2 over e5-small.
Cargo-feature-gated like `pdf`. Flatpak manifest pinning both the runtime binary and the
weights. `#10` stays cut until an eval set exists.

---

## 11. Decisions — resolved

1. **POV** → weak many-to-many `point_of_view` on `BinderItem`, via Qleany. See §12.
2. **Sentence splitter** → edit `text-document`. Authorised. See §13.
3. **Voice-drift** → cut, substituted by tense/person consistency.
4. **Fixture** → metadata-only enrichment. See §15.
5. **Rhythm** → its own view, peer of Pace. See §14.

---

# Round 2

## 12. POV as a weak M2M

**The manifest edit.** `references` today (`qleany.yaml:488-491`):

```yaml
- name: references
  type: entity
  entity: BinderItem
  relationship: many_to_many
```

`point_of_view` is its structural twin, inserted after `references`, before `tags`:

```yaml
- name: point_of_view
  type: entity
  entity: BinderItem
  relationship: many_to_many
```

Weak and unordered **by construction** — Qleany's `many_to_many` is always weak, and there
is no `ordered_many_to_many` in the manifest schema at all.

**Regeneration.** Commit first. Then `qleany diff <path>` each candidate individually and
generate only what actually changes. **Never `generate entity binder_item`** — it pulls in
~21 files including the hand-dep `Cargo.toml` and every hand-maintained uow scaffold.

| File | Action |
|---|---|
| `common/src/entities.rs` | regenerate, re-add SPDX |
| `common/src/direct_access/binder_item/binder_item_repository.rs` | regenerate, re-add SPDX |
| `common/src/direct_access/binder_item/binder_item_table.rs` | regenerate, re-add SPDX |
| `common/src/database/hashmap_store.rs` | regenerate, re-add SPDX |
| `direct_access/src/binder_item/dtos.rs` | regenerate, re-add SPDX |
| `direct_access/src/binder_item/binder_item_controller.rs` | **regenerate-then-repair** — regen drops `with_identity()` (`:20-37`) and `#[allow(clippy::too_many_arguments)]` (`:281`), silently, while still compiling |

Then ~9 hand-written files: `skrib_format`'s `bundle.rs` / `mapping.rs` / `loaded.rs` /
`tree_read.rs` (the hydration line lives at `tree_read.rs:239-242`, shared by save / export /
backup / scan_mentions), `load_work_uc.rs` (both the modern `materialize` at `:396-405` and
the legacy-SQLite path at `:924-941`), `duplicate_uc.rs`, `single_binder_item.rs`,
`inspector.rs`.

**No new use case is needed.** `binder_item_controller.rs::set_relationship` (`:265-279`)
already routes any `BinderItemRelationshipField` through the generic
`UndoableSetRelationshipUseCase`. And **no `uow_action` macro-list changes** — the
`GetRelationshipRO`/`SetRelationship` actions are generic over the whole enum, not per field.

**Persistence: serde default, no `FORMAT_VERSION` bump.** Precedent splits two ways: `uid`
(v2→v3, `d350c67ea`) bumped and needed a real `heal_uid` step, because nil uid was never
valid. `aliases`/`discoverable`/`kind`/`chapter_flat` (`abb100978`) are additive
serde-default fields that did **not** bump. Empty `point_of_view` is always a legal state,
so it matches the latter. Stay at 4.

**The compile-error tripwire is a feature.** Every non-spread `BinderItem { … }` literal
breaks until it chooses explicitly — `duplicate_uc.rs:150`, `load_work_uc.rs:331,864`,
`mapping.rs:457`, `language.rs:270,282`, plus two test files. Sites using
`..Default::default()` compile silently with an empty POV; check each is actually correct.
`duplicate_uc.rs` explicitly carries `references` and `tags` forward (`:187-210`);
`split_scene_uc.rs` carries neither (`:168-193`) and probably shouldn't.

**What M2M means downstream — the part worth deciding now.** Zero and two-plus POV scenes
are both legal. So:

- **Never read `Vec[0]`.** It's unordered; "the first POV" is not a thing.
- **Every POV-keyed view needs an explicit "unassigned" bucket.**
- **Vocabulary richness:** do *not* fractionally split a co-POV scene's words. Attribute
  each character only across scenes where they **solely** hold POV, and report multi-POV
  scenes as their own bucket.
- **A two-POV scene is a signal, not an error** — that's head-hopping detection, for free,
  as a by-product of the schema. Surface it; don't block it.

**What the model cannot enforce** (and shouldn't try — `references` doesn't either):
exactly-one-POV, POV-implies-cast, POV-restricted-to-`discoverable`. All three stay UI-layer.
If picking a POV should auto-add that character to the cast, do it as a **one-directional UI
invariant** wrapped in `begin_composite`/`end_composite` (the `single_milestone.rs` pattern)
— otherwise Ctrl+Z reverts only half and strands the character in the cast.

Reuse `cast_add_button` / `CastAddPopover` / `candidates_from_table` for the picker's add
mechanics, but **not** `MentionList` for the roster — its row type carries scan-evidence
fields that are irrelevant here. A small chip row is right.

Also: apply the same unresolved-target filter `scan_mentions_uc.rs` uses for `references`, or
a POV id pointing at a trashed item will mis-render.

## 13. The `text-document` change

Smaller than first assessed (§1.2). Recommended shape:

- **Relocate** the pure logic from `crates/public_api/src/sentence.rs` to
  `crates/common/src/parser_tools/sentence.rs` — matching the `word_count.rs` /
  `content_parser.rs` precedent for "pure `&str`, no document, no store" primitives.
- Keep `sentence_bounds`'s signature **byte-for-byte unchanged**; just make it `pub`.
- Add one sibling, factored out of the boundary computation it already does internally:

```rust
pub struct Sentence<'a> { pub text: &'a str, pub char_range: std::ops::Range<usize> }
pub fn sentences<'a>(text: &'a str, locale: Option<&str>) -> Vec<Sentence<'a>>
```

Returning the sliced `&str` alongside the range matters: a stats caller wants substrings to
word-count, and would otherwise redo an O(n) char→byte reconversion per sentence.

- Char ranges, not byte ranges — the crate's uniform convention (cf. `Match::char_start`).
- **Keep the locale parameter.** ~40 languages are genuinely tailored here.
- **Strictly additive — do not fix accuracy in the same change.** Two verified gaps: a
  literal ellipsis `U+2026` never splits, and French em-dash dialogue turns mis-split. Pin
  both with named "documented limitation" tests and document them in the doc comment.
  Improving them touches already-shipped caret-navigation behaviour and is separate work.
- A proptest asserting `sentence_bounds(text, at, locale)` is always exactly one of
  `sentences(text, locale)`'s ranges locks the two against future divergence.
- Two internal call sites move: `document.rs:744`, `cursor.rs:3090`.

**Accuracy is adequate for the use case.** Occasional mis-splits bias a mean or variance
over a 2,400-word scene negligibly — unlike caret navigation, where a wrong boundary is
visible immediately. But that's also the new hazard: a batch-statistics caller has no caret
to sanity-check against.

Commit separately in that repo, staging only our own files; it develops on `main`.

## 14. Segments, Rhythm, and where drift lives

**The 8-segment problem, solved by moving Pace.** Adding Rhythm *and* Analysis to the Book
bar gives eight segments — past `SegmentedControl`'s documented 2–5 ceiling. Recommendation:

- Keep a **uniform 5-segment bar everywhere**: own page / manuscript / Full Synopsis /
  Corkboard / Overview.
- Add a **Book-only trailing "Insights" ComboBox**: Pace / Rhythm / Analysis.

This keeps Rhythm a genuine one-click peer of Pace (as asked), respects bastyde's own
past-five guidance, and — the real win — **removes the existing Book-only index shift
entirely**, since Pace is today's `extra` and is exactly what makes Book's indices differ
from every other container.

**Kill the positional fragility while you're in there.** Replace the raw `Signal<usize>`
with a `ContainerSegment` enum. The hardcoded indices at
[`tabs.rs:1172`](crates/bastyde_ui/src/tabs.rs#L1172) and
[`:1229`](crates/bastyde_ui/src/tabs.rs#L1229) are the standing trap; a segment added
without its child compiles fine and silently shows the previous view under the new label.

**Trade to accept:** Pace becomes one click deeper than today.

**Where synopsis-vs-prose lives — two places, deliberately.** They are a *sibling pair*, not
one blended health score:

| | Home | Why |
|---|---|---|
| **Length ratio** (synopsis words ÷ prose words) | a sortable **Overview column** | Cheap, self-explanatory, every row has one. Sorting *is* the finder. The synopsis text is already fetched — zero new backend cost |
| **Lexical drift** (does the prose do what the synopsis says?) | a **finder in Analysis** | Needs an explanation to act on — "your synopsis names Constantine and the Senate; the prose mentions neither" is a row with content, not a cell |

Use Overview's existing `Option<usize>` / em-dash convention for the ratio column, as
`own_words` does — never a misleading `0%` where the answer is "no synopsis".

**Build drift on the mention layer, not bag-of-words.** Raw lexical overlap between a
2-sentence synopsis and a 2,400-word scene is near-zero *even when the synopsis is perfectly
accurate* — the same trap as raw TTR. Two corrections:

1. Measure **containment of the synopsis's terms in the prose**, IDF-weighted — not
   symmetric similarity — and threshold against the distribution across this book's own
   scenes.
2. Proper nouns carry the signal, and `ScanMentions` already resolves them with folding,
   word boundaries and aliases. Fall back to IDF-weighted content words when there's no
   story bible, so it degrades gracefully.

**Rhythm's charts.** One `BarChart` (word count along the stream, self-referentially
coloured like Pace's) plus two **hand-rolled `RectWidget` heat-strips** (dialogue ratio,
sentence length) reusing the `WeekdayChips` pattern from `tabs/pace/` — don't fight
`BarChart`'s axis/legend/gridline machinery for a widget that wants none of it. Sequential
ramp via `Color::mix` between two **semantic roles**, so light and dark both work and no raw
hex appears.

**Rhythm needs no long op.** It's strictly cheaper than what `OverviewRowsModel` already
does synchronously today; worst case ~100 ms, an order of magnitude under the app's own
200 ms spinner threshold. Compute it inline. (Analysis's finders still need the long op.)

**Degenerate states.** A Notebook project has no Book row, so Rhythm and Analysis simply
never render — structurally, exactly as Pace already doesn't. Analysis's drift finder needs
a "no synopses yet" message *distinct* from "0 findings".

## 15. The fixture — go on metadata, stop at the prose

**`Starforgers` is a real, in-print commercial novel.** *Starforgers* (Star Saga Book 1) by
**Ken McConnell**, GB Press, 2011/2012, with at least four sequels actively sold today. The
only permission in the repo is informal and lives in two places: the `blurb: "by Ken
McConnell"` string in `examples_list_model.rs`, and the fixture's own copyright page —
*"This novel is used as an example in Skribisto with the full agreement of the author Ken
McConnell… Copyright 2011, 2012 by Ken McConnell. All Rights Reserved."*

Skribisto is GPL-3.0-only; that governs the app, not bundled third-party content. The
project already documents bundled third-party licences for spell-check dictionaries
(`crates/bastyde_ui/assets/dictionaries/licenses/`) — **Starforgers has no equivalent
`NOTICE`.**

So:

- ✅ **GO — metadata only.** 36 short synopses in our own words, tags, character/story-bible
  `Note` items, aliases, POV assignments, one deliberately-mismatched synopsis and one
  deliberately-unreferenced note. All of this is metadata *about* the work, stays inside
  "used as an example", and never touches his prose. The novel is third-person with a large
  recurring cast (Devon Ardel, Hap Anders, Capt. Morgan Blud, Trimble, Kantor, Constantine…)
  across multiple POV threads — ideal for demoing POV, cast and mention coverage.
- ❌ **NO-GO — planted prose flaws.** A duplicated paragraph or a near-identical scene pair
  means altering a living author's copyrighted expression, in a product shipped to every
  user, for QA convenience. The informal grant does not cover it.
- ➕ **Build a third, small, fully project-owned synthetic fixture** for the prose-level
  planted flaws *and* for threshold calibration. `resources/test/skribisto_test_project.skrib`
  already proves the pattern (39 KB, placeholder prose, consumed by 9 test files) but must
  **not** be reused for this — its exact word/tag/search-hit counts are asserted by those 9
  unrelated consumers, so content edits become cross-cutting breakage.
- 📄 **Write the `NOTICE`** formalising the Starforgers permission before shipping enriched
  metadata.

**One caveat on the synopses.** They double as calibration ground truth for the drift
detector. A subtly wrong synopsis across a 31-chapter multi-POV plot doesn't just demo badly
— it teaches the wrong threshold. They need reading, not skimming.

**Housekeeping spotted in passing:** `Starforgers-20260722-144410.skrib` and
`-230450.skrib` are committed in `resources/examples/` and look like accidental backup-file
commits from manual testing (commits `8db8f9b80`, `b10742a48`), not intended content.

## 16. Still open

1. Should `duplicate` carry `point_of_view` forward, or is a POV pick scene-specific enough
   not to clone?
2. POV candidates — the full `discoverable` table (which makes auto-add-to-cast meaningful),
   or only the already-confirmed cast?
3. Accept the vocabulary-attribution rule (solely-held-POV scenes only, multi-POV as its own
   bucket)?
4. Pace moving into an "Insights" combo — acceptable, or should Pace stay in the bar and
   Rhythm/Analysis go elsewhere?
5. **Scope selector** — This Book / whole Work / a Part?
6. Inspector copy for the POV section, both locales — needs its own pass.

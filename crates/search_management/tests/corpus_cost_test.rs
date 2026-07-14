//! Which half of a search is expensive — the parse, or the fold?
//!
//! The plan assumed the **parse** (`djot_to_plain_text`, once per scene) dominated, and that
//! caching the extracted prose would be enough. The measurement says otherwise, and the
//! cache is designed against the measurement.
//!
//! Run it:
//!
//! ```text
//! cargo test --release -p skribisto-search-management --test corpus_cost_test \
//!     -- --ignored --nocapture
//! ```

use std::time::Instant;

use text_document::matching::{MatchOptions, find_all};
use text_document::{DjotImportOptions, djot_to_plain_text};

const SCENES: usize = 4000;

fn scene_prose(i: usize) -> String {
    let hit = if i % 10 == 0 { "Aurélien" } else { "Elena" };
    format!(
        "{hit} traversa la forêt où l'ombre s'étirait entre les *hêtres*, et le vent \
         portait l'odeur du sel. Elle songeait à la _promesse_ faite au bord de l'eau, \
         celle qu'elle n'avait jamais su tenir, et le [souvenir](https://exemple.test/{i}) \
         revenait comme une marée lente et patiente qui ne demandait rien.\n\n\
         Le café refroidissait doucement sur la table de chêne pendant qu'elle relisait la \
         lettre, encore une fois, sans y trouver ce qu'elle cherchait vraiment."
    )
}

#[test]
#[ignore = "measurement, not an assertion — run with --release --ignored --nocapture"]
fn which_half_of_a_search_is_expensive() {
    let djot: Vec<String> = (0..SCENES).map(scene_prose).collect();

    // 1. The parse. Once per scene, per search — unless cached.
    let t = Instant::now();
    let prose: Vec<String> = djot
        .iter()
        .map(|d| djot_to_plain_text(d, &DjotImportOptions::default()))
        .collect();
    let parse = t.elapsed();

    // 2. Fold + scan, which is what `find_all` does today: it folds the haystack from
    //    scratch on every call.
    let folded_opts = MatchOptions::default();
    let t = Instant::now();
    let mut hits = 0;
    for p in &prose {
        hits += find_all(p, "aurelien", &folded_opts).len();
    }
    let fold_and_scan = t.elapsed();

    // 3. The scan alone, with no fold to do (both toggles strict = the fold is the
    //    identity). This is the floor: what a search would cost if the fold were free.
    let strict = MatchOptions {
        case_sensitive: true,
        diacritic_sensitive: true,
        ..MatchOptions::default()
    };
    let t = Instant::now();
    let mut hits2 = 0;
    for p in &prose {
        hits2 += find_all(p, "Elena", &strict).len();
    }
    let scan_only = t.elapsed();

    let chars: usize = prose.iter().map(|p| p.chars().count()).sum();
    eprintln!("\n  {SCENES} scenes, {} chars of prose", chars);
    eprintln!("    parse   (djot_to_plain_text)   {parse:>12?}");
    eprintln!("    fold + scan (find_all)         {fold_and_scan:>12?}   ({hits} hits)");
    eprintln!("    scan alone (identity fold)     {scan_only:>12?}   ({hits2} hits)");
    eprintln!(
        "\n    => the FOLD costs {:?} ({:.0}x the scan, {:.1}x the parse)\n",
        fold_and_scan.saturating_sub(scan_only),
        fold_and_scan.as_secs_f64() / scan_only.as_secs_f64().max(1e-9),
        fold_and_scan.as_secs_f64() / parse.as_secs_f64().max(1e-9),
    );
}

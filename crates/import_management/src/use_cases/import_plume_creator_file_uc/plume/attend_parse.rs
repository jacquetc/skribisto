//! Parse the `attendance` member into a version-neutral [`PlumeAttendance`].
//!
//! Normalizes every attendance version (0.1–0.6), mirroring
//! `FileUpdater::updateAttendanceFile`:
//!  - root tag `plume-attendance` (0.3+) **or** legacy `attendance` (≤0.2);
//!  - children are `<group>`/`<obj>` (0.4+) **or**, in older files, direct
//!    `<char>`/`<item>`/`<place>` elements — which we fold into synthesized
//!    Characters / Items / Places groups (a `<char>`'s name defaults to
//!    `firstName + " " + lastName`, as Plume's 0.3→0.4 step did);
//!  - the root box catalogs come from `box_1/2/3` (0.5+) **or** the legacy
//!    `levelsNames`/`rolesNames` (→ box_1/box_2); each obj's classification comes
//!    from `box_1/2/3` **or** the legacy `level`/`role`, resolved to a label here.

use anyhow::Result;

use super::model::{PlumeAttendance, PlumeGroup, PlumeObj};
use super::version::{check_root, parse_xml, version_newer_than};

/// The newest attendance schema this importer understands.
const ATTEND_TERMINAL: f64 = 0.6;

pub fn parse(xml: &str) -> Result<PlumeAttendance> {
    let doc = parse_xml(xml)?;
    let root = doc.root_element();
    check_root(&root, &["plume-attendance", "attendance"], "attendance")?;
    if version_newer_than(root.attribute("version"), ATTEND_TERMINAL) {
        anyhow::bail!(
            "this Plume project's attendance is version {} — newer than this importer supports ({ATTEND_TERMINAL})",
            root.attribute("version").unwrap_or("?")
        );
    }

    // Root classification catalogs: box_1/2/3 (0.5+) or the legacy names.
    let catalogs = [
        split_catalog(
            root.attribute("box_1")
                .or_else(|| root.attribute("levelsNames")),
        ),
        split_catalog(
            root.attribute("box_2")
                .or_else(|| root.attribute("rolesNames")),
        ),
        split_catalog(root.attribute("box_3")),
    ];
    let spinbox_label = root
        .attribute("spinBox_1_label")
        .unwrap_or_default()
        .to_string();

    let groups: Vec<roxmltree::Node> = root
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "group")
        .collect();

    let parsed_groups = if !groups.is_empty() {
        groups
            .iter()
            .map(|g| PlumeGroup {
                number: parse_u32(g.attribute("number")),
                name: g.attribute("name").unwrap_or_default().to_string(),
                objs: g
                    .children()
                    .filter(|n| n.is_element() && n.tag_name().name() == "obj")
                    .map(|o| parse_obj(&o, &catalogs))
                    .collect(),
            })
            .collect()
    } else {
        // Legacy (≤0.3): flat char/item/place elements → 3 synthesized groups.
        legacy_groups(&root, &catalogs)
    };

    Ok(PlumeAttendance {
        spinbox_label,
        groups: parsed_groups,
    })
}

fn legacy_groups(root: &roxmltree::Node, catalogs: &[Vec<String>; 3]) -> Vec<PlumeGroup> {
    let collect = |tag: &str, group_name: &str| -> Option<PlumeGroup> {
        let objs: Vec<PlumeObj> = root
            .descendants()
            .filter(|n| n.is_element() && n.tag_name().name() == tag)
            .map(|o| parse_obj(&o, catalogs))
            .collect();
        if objs.is_empty() {
            None
        } else {
            Some(PlumeGroup {
                number: None,
                name: group_name.to_string(),
                objs,
            })
        }
    };
    [
        collect("char", "Characters"),
        collect("item", "Items"),
        collect("place", "Places"),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn parse_obj(o: &roxmltree::Node, catalogs: &[Vec<String>; 3]) -> PlumeObj {
    // Name: explicit `name`, else legacy first+last (the 0.3→0.4 synthesis).
    let name = match o.attribute("name") {
        Some(n) if !n.trim().is_empty() => n.trim().to_string(),
        _ => {
            let first = o.attribute("firstName").unwrap_or_default();
            let last = o.attribute("lastName").unwrap_or_default();
            format!("{first} {last}").trim().to_string()
        }
    };

    let box_labels = [
        resolve_label(
            &catalogs[0],
            o.attribute("box_1").or_else(|| o.attribute("level")),
        ),
        resolve_label(
            &catalogs[1],
            o.attribute("box_2").or_else(|| o.attribute("role")),
        ),
        resolve_label(&catalogs[2], o.attribute("box_3")),
    ];

    let spinbox = match o.attribute("spinBox_1") {
        Some(s) if !s.trim().is_empty() && s.trim() != "0" => s.trim().to_string(),
        _ => String::new(),
    };

    PlumeObj {
        number: parse_u32(o.attribute("number")),
        name,
        aliases: o
            .attribute("aliases")
            .unwrap_or_default()
            .trim()
            .to_string(),
        quick_details: o.attribute("quickDetails").unwrap_or_default().to_string(),
        box_labels,
        spinbox,
    }
}

/// Split a `--`-separated label catalog (`"Main--Secondary--None"`).
fn split_catalog(raw: Option<&str>) -> Vec<String> {
    match raw {
        Some(s) if !s.is_empty() => s.split("--").map(|p| p.trim().to_string()).collect(),
        _ => Vec::new(),
    }
}

/// Resolve a box index (into its catalog) to the label string, or `""`.
fn resolve_label(catalog: &[String], idx_raw: Option<&str>) -> String {
    let idx: usize = idx_raw.and_then(|s| s.trim().parse().ok()).unwrap_or(0);
    catalog.get(idx).cloned().unwrap_or_default()
}

fn parse_u32(raw: Option<&str>) -> Option<u32> {
    raw.and_then(|s| s.trim().parse::<u32>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modern_groups_and_boxes() {
        let xml = r#"<plume-attendance version="0.6"
            box_1="Main--Secondary--None" box_2="None--Protagonist--Antagonist" spinBox_1_label="Age :">
            <group number="2" name="Characters">
              <obj number="1" name="Elise" aliases="Kiri" quickDetails="Heroine"
                   box_1="0" box_2="1" spinBox_1="20"/>
            </group>
            <group number="3" name="Places"/>
        </plume-attendance>"#;
        let a = parse(xml).unwrap();
        assert_eq!(a.spinbox_label, "Age :");
        assert_eq!(a.groups.len(), 2);
        let obj = &a.groups[0].objs[0];
        assert_eq!(obj.name, "Elise");
        assert_eq!(obj.box_labels[0], "Main"); // box_1=0 → catalog[0]
        assert_eq!(obj.box_labels[1], "Protagonist"); // box_2=1 → catalog[1]
        assert_eq!(obj.spinbox, "20");
    }

    #[test]
    fn legacy_char_item_place_synthesizes_groups() {
        let xml = r#"<!DOCTYPE plume><attendance version="0.3"
            levelsNames="Main--Secondary" rolesNames="None--Protagonist">
            <char number="1" firstName="Elise" lastName="Laroche" level="0" role="1"/>
            <place number="2" name="Paris"/>
        </attendance>"#;
        let a = parse(xml).unwrap();
        // Characters (from <char>) + Places (from <place>); Items empty → omitted.
        assert_eq!(a.groups.len(), 2);
        assert_eq!(a.groups[0].name, "Characters");
        assert_eq!(a.groups[0].objs[0].name, "Elise Laroche"); // synthesized
        assert_eq!(a.groups[0].objs[0].box_labels[0], "Main"); // level=0 → box_1
        assert_eq!(a.groups[0].objs[0].box_labels[1], "Protagonist"); // role=1 → box_2
        assert_eq!(a.groups[1].name, "Places");
        assert_eq!(a.groups[1].objs[0].name, "Paris");
    }
}

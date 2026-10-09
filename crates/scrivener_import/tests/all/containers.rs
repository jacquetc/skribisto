// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Every way a writer can hand over a project, and every pick that means one:
//! the folder, the folder around it, a folder or file inside it, its manifest, a
//! zip with or without a wrapping folder, a template in either encoding. The same
//! project reads into the same model through each.

use scrivener_fixtures::{Generation, Platform, Spec, TemplateEncoding, template};
use scrivener_import::model::{
    BinderRow, Container, ManifestChoice, TemplateEncoding as ReadEncoding,
};
use scrivener_import::outcome::BackupSign;
use scrivener_import::{Answers, Notice, Question, Refusal, looks_like_a_scrivener_project};

use crate::support::{
    has, question, read, refusal, run, run_with, temp, write, write_file, write_files,
};

/// What must not differ between two containers of one project: each row's
/// place, kind, title, key, and where its files are.
fn shape(rows: &[BinderRow]) -> Vec<String> {
    rows.iter()
        .map(|r| {
            format!(
                "{} {:?} {:?} {:?} {:?} {:?} {:?}",
                r.depth,
                r.kind,
                r.title,
                r.key,
                r.payload.text.as_ref().map(|m| (&m.path, m.size)),
                r.payload.synopsis.as_ref().map(|m| &m.path),
                r.payload.file.as_ref().map(|m| &m.path)
            )
        })
        .collect()
}

#[test]
fn the_same_project_reads_the_same_from_a_folder_a_zip_and_a_template() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let files = spec.build();
    let folder = write(&spec, dir.path());
    let from_folder = read(run(&folder));
    assert_eq!(from_folder.project.source.container, Container::Folder);

    // A zip wrapping the folder, as Finder's "Compress" and Scrivener's backups do.
    let zipped = write_file(
        &files.zip(Some(&spec.folder())).unwrap_or_default(),
        dir.path(),
        "Lighthouse.scriv.zip",
    );
    let from_zip = read(run(&zipped));
    assert_eq!(from_zip.project.source.container, Container::Zip);
    assert!(has(&from_zip, "scrivener-notice-project-found-below-pick"));
    assert_eq!(
        shape(&from_zip.project.binder),
        shape(&from_folder.project.binder)
    );
    assert_eq!(from_zip.project.source.folder_name, spec.folder());

    // A zip of the folder's contents, with no wrapping folder.
    let bare = write_file(&files.zip(None).unwrap_or_default(), dir.path(), "bare.zip");
    let from_bare = read(run(&bare));
    assert!(!has(
        &from_bare,
        "scrivener-notice-project-found-below-pick"
    ));
    assert_eq!(
        shape(&from_bare.project.binder),
        shape(&from_folder.project.binder)
    );

    // A Scrivener 3 template: the project zip in base64 inside XML.
    let zip = files.zip(Some(&spec.folder())).unwrap_or_default();
    let tpl = write_file(
        &template(&zip, TemplateEncoding::Base64, "Lighthouse starter"),
        dir.path(),
        "Starter.scrivtemplate",
    );
    let from_template = read(run(&tpl));
    assert_eq!(
        from_template.project.source.container,
        Container::Template(ReadEncoding::Base64)
    );
    assert_eq!(
        from_template.project.source.template_title.as_deref(),
        Some("Lighthouse starter")
    );
    assert!(from_template.notices.contains(&Notice::SourceIsTemplate {
        title: Some("Lighthouse starter".to_string())
    }));
    assert_eq!(
        shape(&from_template.project.binder),
        shape(&from_folder.project.binder)
    );
}

/// Scrivener 2 wrote templates in hexadecimal with no `Type`; read as base64
/// they are garbage.
#[test]
fn a_scrivener_2_hex_template_reads() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format10, Platform::Mac);
    let zip = spec.build().zip(Some(&spec.folder())).unwrap_or_default();
    let tpl = write_file(
        &template(&zip, TemplateEncoding::Hex, "RPG"),
        dir.path(),
        "RPG.scrivtemplate",
    );
    let read = read(run(&tpl));
    assert_eq!(
        read.project.source.container,
        Container::Template(ReadEncoding::Hex)
    );
    assert_eq!(read.project.binder.len(), spec.items.len());
}

/// Windows users pick the `.scrivx`: its folder is the project, and exactly that
/// manifest is read.
#[test]
fn a_picked_manifest_pins_its_folder_and_itself() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format15, Platform::Windows);
    let folder = write(&spec, dir.path());
    let read = read(run(&folder.join(spec.manifest_name())));
    assert_eq!(read.project.source.manifest.chosen, ManifestChoice::Picked);
    assert_eq!(read.project.binder.len(), spec.items.len());
}

/// A manifest named unlike its folder (Windows 1.x called them all
/// `project.scrivx`) is still the project's.
#[test]
fn a_manifest_need_not_carry_the_folder_s_name() {
    let dir = temp();
    let mut spec = Spec::novel(Generation::Format10, Platform::Windows);
    spec.manifest_stem = Some("project".to_string());
    let folder = write(&spec, dir.path());
    let read = read(run(&folder));
    assert_eq!(read.project.source.manifest.name, "project.scrivx");
    assert_eq!(read.project.source.folder_name, "Lighthouse.scriv");
}

/// The folder around a project finds it two levels down; the folder around two
/// asks which, and the answer reads it.
#[test]
fn a_folder_around_projects_finds_one_or_asks_which() {
    let dir = temp();
    let writing = dir.path().join("Writing");
    let novels = writing.join("Novels");
    let first = write(&Spec::novel(Generation::Format20, Platform::Mac), &novels);
    let read_one = read(run(&writing));
    assert!(read_one.notices.contains(&Notice::ProjectFoundBelowPick {
        path: "Novels/Lighthouse.scriv".to_string()
    }));

    let mut other = Spec::novel(Generation::Format15, Platform::Mac);
    other.name = "Harbour".to_string();
    write(&other, &novels);
    let asked = question(run(&writing));
    let Question::SeveralProjects { candidates } = &asked else {
        panic!("expected several projects, got {asked:?}");
    };
    let paths: Vec<&str> = candidates.iter().map(|c| c.path.as_str()).collect();
    assert_eq!(
        paths,
        vec!["Novels/Harbour.scriv", "Novels/Lighthouse.scriv"]
    );
    let answered = read(run_with(
        &writing,
        &Answers {
            project: Some("Novels/Harbour.scriv".to_string()),
            ..Answers::default()
        },
    ));
    assert_eq!(answered.project.source.folder_name, "Harbour.scriv");
    assert!(first.exists());
}

/// A project is never searched inside: the stale manifest a template leaves in
/// `Files/` is not a second project.
#[test]
fn a_project_is_never_searched_inside() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let mut files = spec.build();
    files.put(
        "Files/Workflow.scrivx",
        files.get("Lighthouse.scrivx").unwrap_or_default().to_vec(),
    );
    write_files(&files, dir.path(), "Lighthouse.scriv");
    let read = read(run(dir.path()));
    assert!(
        read.notices
            .iter()
            .all(|n| !matches!(n, Notice::UnrecognisedFiles { .. }))
    );
}

/// Picking a folder or a file inside a project offers the project; accepting
/// reads it.
#[test]
fn a_pick_inside_a_project_offers_the_project() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let folder = write(&spec, dir.path());
    let data = folder.join("Files").join("Data");
    assert_eq!(
        question(run(&data)),
        Question::EnclosingProject {
            project: "Lighthouse.scriv".to_string(),
            levels_up: 2,
        }
    );
    let accepted = read(run_with(
        &data,
        &Answers {
            use_enclosing_project: true,
            ..Answers::default()
        },
    ));
    assert_eq!(accepted.project.binder.len(), spec.items.len());
    assert!(matches!(
        question(run(&folder.join("Files").join("binder.autosave"))),
        Question::EnclosingProject { .. }
    ));
    // The stale manifest inside Files/ means the project around it, too.
    let mut files = spec.build();
    files.put(
        "Files/Workflow.scrivx",
        files.get("Lighthouse.scrivx").unwrap_or_default().to_vec(),
    );
    let other = write_files(&files, &dir.path().join("second"), "Lighthouse.scriv");
    assert!(matches!(
        question(run(&other.join("Files").join("Workflow.scrivx"))),
        Question::EnclosingProject { levels_up: 2, .. }
    ));
}

/// The analysis says where the project it read is, whatever the writer picked: a
/// folder inside it, the folder around it, the project chosen among several, the zip
/// it came in, a link to it. What is beside the project, and what is inside it, is
/// decided from that, never from the pick.
#[test]
fn the_project_read_is_located_whatever_was_picked() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let folder = write(&spec, dir.path());
    let enclosing = Answers {
        use_enclosing_project: true,
        ..Answers::default()
    };
    assert_eq!(read(run(&folder)).location, folder);
    assert_eq!(
        read(run_with(&folder.join("Files").join("Data"), &enclosing)).location,
        folder,
        "a folder inside the project"
    );
    assert_eq!(
        read(run_with(
            &folder.join("Files").join("binder.autosave"),
            &enclosing
        ))
        .location,
        folder,
        "a file inside the project"
    );
    assert_eq!(
        read(run(&folder.join("Lighthouse.scrivx"))).location,
        folder,
        "its manifest"
    );
    assert_eq!(
        read(run(dir.path())).location,
        folder,
        "the folder around it"
    );

    let mut other = Spec::novel(Generation::Format15, Platform::Mac);
    other.name = "Harbour".to_string();
    let harbour = write(&other, dir.path());
    let chosen = Answers {
        project: Some("Harbour.scriv".to_string()),
        ..Answers::default()
    };
    assert_eq!(
        read(run_with(dir.path(), &chosen)).location,
        harbour,
        "the project chosen among several"
    );

    let zipped = write_file(
        &spec.build().zip(Some(&spec.folder())).unwrap_or_default(),
        dir.path(),
        "Lighthouse.scriv.zip",
    );
    assert_eq!(read(run(&zipped)).location, zipped, "the zip it came in");

    #[cfg(unix)]
    {
        let link = dir.path().join("Shortcut.scriv");
        if let Err(e) = std::os::unix::fs::symlink(&folder, &link) {
            panic!("a link: {e}");
        }
        let real = std::fs::canonicalize(&folder).unwrap_or(folder.clone());
        let located = read(run(&link)).location;
        assert_eq!(
            std::fs::canonicalize(&located).unwrap_or(located),
            real,
            "the project a link leads to"
        );
    }
}

/// A folder holding only manifests (published ones, or a copy that lost its
/// files) is refused by name, and a folder of nothing Scrivener made is not a
/// project.
#[test]
fn manifests_without_their_files_are_refused() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let mut files = spec.build();
    files.remove_under("Files");
    files.remove_under("Settings");
    files.remove_under("Snapshots");
    files.remove_under("QuickLook");
    let lone = write_files(&files, dir.path(), "Published");
    assert_eq!(
        refusal(run(&lone)),
        Refusal::ManifestWithoutFiles {
            manifest: "Lighthouse.scrivx".to_string()
        }
    );
    let empty = dir.path().join("Holiday photos");
    let _ = std::fs::create_dir_all(&empty);
    write_file(b"not a project", &empty, "beach.txt");
    assert_eq!(refusal(run(&empty)), Refusal::NotAScrivenerProject);
    assert_eq!(
        refusal(run(&empty.join("beach.txt"))),
        Refusal::NotAScrivenerProject
    );
    assert_eq!(
        refusal(run(&dir.path().join("nowhere"))),
        Refusal::Missing {
            file: "nowhere".to_string()
        }
    );
}

/// Scrivener's automatic backups say so: their text may be older than the
/// writer's working project.
#[test]
fn an_automatic_backup_is_named_as_one() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let zip = spec.build().zip(Some(&spec.folder())).unwrap_or_default();
    let bak = write_file(&zip, dir.path(), "Lighthouse.bak3.zip");
    assert!(read(run(&bak)).notices.contains(&Notice::SourceIsBackup {
        sign: BackupSign::BakArchive
    }));
    let stamped = write(&spec, &dir.path().join("Lighthouse [2025_06_01_05_14_50]"));
    assert!(
        read(run(&stamped))
            .notices
            .contains(&Notice::SourceIsBackup {
                sign: BackupSign::DateStamped
            })
    );
    let backups = write(&spec, &dir.path().join("Backups"));
    assert!(
        read(run(&backups))
            .notices
            .contains(&Notice::SourceIsBackup {
                sign: BackupSign::BackupsFolder
            })
    );
}

/// A zip made by Finder carries `__MACOSX` and AppleDouble halves; they are
/// dropped without a word.
#[test]
fn finder_litter_in_an_archive_is_dropped_silently() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let mut files = spec.build().nested(&spec.folder());
    files.put(
        "__MACOSX/Lighthouse.scriv/._Lighthouse.scrivx",
        b"\0\x05\x16\x07".to_vec(),
    );
    files.put(
        "Lighthouse.scriv/._Lighthouse.scrivx",
        b"\0\x05\x16\x07".to_vec(),
    );
    files.put("Lighthouse.scriv/.DS_Store", b"\0\0\0\x01Bud1".to_vec());
    let zip = write_file(
        &files.zip(None).unwrap_or_default(),
        dir.path(),
        "Lighthouse.zip",
    );
    let read = read(run(&zip));
    let keys: Vec<&str> = read
        .notices
        .iter()
        .map(|n| n.key())
        .filter(|k| !crate::support::about_any_project(k))
        .collect();
    assert_eq!(keys, vec!["scrivener-notice-project-found-below-pick"]);
}

/// A caller about to open a file can tell a Scrivener project from anything
/// else without analysing it.
#[test]
fn a_scrivener_project_is_recognised_cheaply() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let folder = write(&spec, dir.path());
    assert!(looks_like_a_scrivener_project(&folder));
    assert!(looks_like_a_scrivener_project(
        &folder.join("Lighthouse.scrivx")
    ));
    assert!(!looks_like_a_scrivener_project(dir.path()));
    assert!(!looks_like_a_scrivener_project(
        &folder.join("Files").join("version.txt")
    ));
}

/// A refusal names what was picked by its own name, never by a path through the
/// writer's folders, even when the path ends in `..` and so has no name of its
/// own to give.
#[test]
fn a_pick_is_named_without_the_folders_above_it() {
    let dir = temp();
    let gone = dir.path().join("Writing").join("Old drafts").join("..");
    let Refusal::Missing { file } = refusal(run(&gone)) else {
        panic!("expected the pick to be missing");
    };
    assert_eq!(file, "Writing");
}

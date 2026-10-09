// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Archives and folders built to hurt: names that climb out, members under one
//! name twice, links, encryption, methods nobody reads, sizes that lie, a folder
//! too wide to search, files the reader may not open. Each is refused with a
//! typed reason or read around, and none takes the process down.

use scrivener_fixtures::hostile::{RawEntry, zip as raw_zip};
use scrivener_fixtures::{Files, Generation, Platform, Spec};
use scrivener_import::outcome::{ManifestDamage, TemplateDamage, UnsafeName};
use scrivener_import::{Question, Refusal};

use crate::support::{question, read, refusal, run, temp, write, write_file, write_files};

/// The novel's files as raw archive entries under `Lighthouse.scriv/`.
fn novel_entries() -> Vec<RawEntry> {
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    spec.build()
        .iter()
        .map(|(path, bytes)| RawEntry::file(&format!("Lighthouse.scriv/{path}"), bytes))
        .collect()
}

fn refused_archive(entries: &[RawEntry]) -> Refusal {
    let dir = temp();
    let path = write_file(&raw_zip(entries), dir.path(), "hostile.zip");
    refusal(run(&path))
}

#[test]
fn the_hand_built_archive_is_itself_readable() {
    // The control: the same writer, nothing hostile, reads.
    let dir = temp();
    let path = write_file(&raw_zip(&novel_entries()), dir.path(), "honest.zip");
    let read = read(run(&path));
    assert_eq!(
        read.project.binder.len(),
        Spec::novel(Generation::Format20, Platform::Mac).items.len()
    );
}

#[test]
fn names_that_leave_the_archive_are_refused() {
    for (name, why) in [
        ("../outside.rtf", UnsafeName::ParentFolder),
        (
            "Lighthouse.scriv/../../outside.rtf",
            UnsafeName::ParentFolder,
        ),
        ("/etc/passwd", UnsafeName::Absolute),
        ("C:/Windows/win.ini", UnsafeName::Absolute),
        (
            "Lighthouse.scriv\\..\\..\\outside.rtf",
            UnsafeName::ParentFolder,
        ),
        (
            "Lighthouse.scriv/bell\u{7}.rtf",
            UnsafeName::ControlCharacter,
        ),
    ] {
        let mut entries = novel_entries();
        entries.push(RawEntry::file(name, b"x"));
        match refused_archive(&entries) {
            Refusal::ArchiveUnsafeName { why: got, .. } => assert_eq!(got, why, "{name}"),
            other => panic!("{name}: {other:?}"),
        }
    }
}

/// Two members under one name: which one a reader sees is an accident.
#[test]
fn members_under_one_name_twice_are_refused() {
    let mut entries = novel_entries();
    entries.push(RawEntry::file(
        "Lighthouse.scriv/Lighthouse.scrivx",
        b"<ScrivenerProject/>",
    ));
    assert!(matches!(
        refused_archive(&entries),
        Refusal::DuplicateArchiveMembers { .. }
    ));
    // The same name spelled with the other separator is the same member.
    let mut entries = novel_entries();
    entries.push(RawEntry::file(
        "Lighthouse.scriv\\Files\\version.txt",
        b"23",
    ));
    assert!(matches!(
        refused_archive(&entries),
        Refusal::DuplicateArchiveMembers {
            member: Some(_),
            ..
        }
    ));
}

#[test]
fn an_encrypted_archive_is_refused() {
    let mut entries = novel_entries();
    entries.push(RawEntry {
        encrypted: true,
        ..RawEntry::file("Lighthouse.scriv/Files/secret.rtf", b"x")
    });
    assert!(matches!(
        refused_archive(&entries),
        Refusal::ArchiveEncrypted { .. }
    ));
}

/// A member the import needs, compressed with a method this importer does not
/// read (7-Zip's Deflate64 here), is refused with its own reason.
#[test]
fn a_needed_member_in_an_unread_compression_is_refused() {
    let mut entries = novel_entries();
    for entry in &mut entries {
        if entry.name.ends_with("Lighthouse.scrivx") {
            entry.method = 9;
        }
    }
    assert!(matches!(
        refused_archive(&entries),
        Refusal::ArchiveUnsupportedCompression { method, .. } if method == "Deflate64"
    ));
}

/// A link inside an archive is recorded and never followed: where the import
/// needs a document, the link refuses it; elsewhere it is reported.
#[test]
fn links_in_an_archive_are_never_followed() {
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let arrival = spec
        .items
        .iter()
        .position(|i| i.title.as_deref() == Some("Arrival"))
        .unwrap_or(0);
    let text = format!("Lighthouse.scriv/{}", spec.text_path(arrival));
    let mut entries: Vec<RawEntry> = novel_entries()
        .into_iter()
        .filter(|e| e.name != text)
        .collect();
    entries.push(RawEntry::link(&text, "/etc/passwd"));
    assert!(matches!(
        refused_archive(&entries),
        Refusal::NeededMemberIsLink { member } if member == spec.text_path(arrival)
    ));

    let mut entries = novel_entries();
    entries.push(RawEntry::link("Lighthouse.scriv/Research link", "/home"));
    let dir = temp();
    let path = write_file(&raw_zip(&entries), dir.path(), "links.zip");
    let read = read(run(&path));
    assert!(read.notices.iter().any(|n| matches!(
        n,
        scrivener_import::Notice::SymlinkNotFollowed { count: 1, .. }
    )));
}

/// A header can claim any size: a document claiming four gigabytes is refused
/// before a byte of it is read.
#[test]
fn a_member_claiming_an_absurd_size_is_refused() {
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let arrival = spec
        .items
        .iter()
        .position(|i| i.title.as_deref() == Some("Arrival"))
        .unwrap_or(0);
    let text = format!("Lighthouse.scriv/{}", spec.text_path(arrival));
    let mut entries = novel_entries();
    for entry in &mut entries {
        if entry.name == text {
            entry.declared_size = Some(u32::MAX);
        }
    }
    assert!(matches!(
        refused_archive(&entries),
        Refusal::ArchiveMemberTooLarge { .. }
    ));
}

/// A manifest that inflates past any real one is refused as too large.
#[test]
fn a_bomb_for_a_manifest_is_refused() {
    let bomb =
        skrib_format::zip_guard::fixtures::compressible_zip("Bomb.scriv/Bomb.scrivx", 80 << 20);
    let dir = temp();
    let path = write_file(&bomb, dir.path(), "bomb.zip");
    // The bomb has no Files folder: it is a lone manifest, refused as one, and
    // never inflated.
    assert!(matches!(
        refusal(run(&path)),
        Refusal::ManifestWithoutFiles { .. } | Refusal::ManifestTooLarge { .. }
    ));
    let mut entries = vec![RawEntry::file("Bomb.scriv/Files/version.txt", b"23")];
    let over =
        skrib_format::zip_guard::fixtures::over_declared_zip("Bomb.scriv/Bomb.scrivx", 1 << 40);
    let path = write_file(&over, dir.path(), "over.zip");
    assert!(matches!(
        refusal(run(&path)),
        Refusal::ManifestWithoutFiles { .. }
            | Refusal::ManifestTooLarge { .. }
            | Refusal::ArchiveMemberTooLarge { .. }
    ));
    entries.push(RawEntry {
        declared_size: Some(u32::MAX),
        ..RawEntry::file(
            "Bomb.scriv/Bomb.scrivx",
            b"<ScrivenerProject Version=\"2.0\"/>",
        )
    });
    assert!(matches!(
        refused_archive(&entries),
        Refusal::ManifestTooLarge { .. }
    ));
}

/// More members than a project may hold, refused from the directory alone.
#[test]
fn an_archive_of_too_many_members_is_refused() {
    let mut entries = vec![RawEntry::file("Many.scriv/Many.scrivx", b"<x/>")];
    for i in 0..100_001u32 {
        entries.push(RawEntry::file(&format!("Many.scriv/Files/{i}"), b""));
    }
    assert!(matches!(
        refused_archive(&entries),
        Refusal::ArchiveTooManyMembers { .. }
    ));
}

#[test]
fn a_damaged_archive_is_refused_as_unreadable() {
    let dir = temp();
    let path = write_file(
        b"PK\x03\x04 and then nothing a zip reader wants",
        dir.path(),
        "broken.zip",
    );
    assert!(matches!(
        refusal(run(&path)),
        Refusal::ArchiveUnreadable { .. }
    ));
}

#[test]
fn a_template_that_does_not_decode_is_refused_naming_why() {
    let dir = temp();
    let truncated = b"<?xml version=\"1.0\"?>\n<ScrivenerTemplate Version=\"1.0\"><Title>T</Title><ProjectZipData Type=\"Base64\">UEsDBBQAAAAIA</ProjectZipData></ScrivenerTemplate>";
    let path = write_file(truncated, dir.path(), "Truncated.scrivtemplate");
    assert!(matches!(
        refusal(run(&path)),
        Refusal::TemplateUndecodable {
            damage: TemplateDamage::Undecodable | TemplateDamage::NotAZip,
            ..
        }
    ));
    let empty = b"<?xml version=\"1.0\"?>\n<ScrivenerTemplate Version=\"1.0\"><Title>T</Title></ScrivenerTemplate>";
    let path = write_file(empty, dir.path(), "Empty.scrivtemplate");
    assert_eq!(
        refusal(run(&path)),
        Refusal::TemplateUndecodable {
            file: "Empty.scrivtemplate".to_string(),
            damage: TemplateDamage::NoProjectData,
        }
    );
}

/// A folder holding a hundred thousand photographs beside a novel costs one
/// listing: the novel is still found.
#[test]
fn a_huge_folder_beside_a_project_does_not_hide_it() {
    let dir = temp();
    let writing = dir.path().join("Writing");
    write(&Spec::novel(Generation::Format20, Platform::Mac), &writing);
    let archive = writing.join("Archive");
    let _ = std::fs::create_dir_all(&archive);
    for i in 0..12_000 {
        let _ = std::fs::write(archive.join(format!("{i}.jpg")), b"");
    }
    let read = read(run(&writing));
    assert!(!read.project.binder.is_empty());
}

/// A pick whose second level holds more folders than the search lists (a home
/// folder) is refused: pick the project itself.
#[test]
fn a_pick_too_wide_to_search_is_refused() {
    let dir = temp();
    let home = dir.path().join("home");
    for i in 0..45 {
        for j in 0..45 {
            let _ = std::fs::create_dir_all(home.join(format!("d{i}")).join(format!("e{j}")));
        }
    }
    assert!(matches!(refusal(run(&home)), Refusal::SearchTooWide { .. }));
}

/// A manifest that declares an entity and names it four thousand times: 78 KB
/// that the XML parser would expand into a 256 MiB title, and a manifest within
/// the 64 MiB ceiling into more memory than any computer has. Scrivener never
/// declares an entity, so the manifest is refused before anything is expanded.
#[test]
fn a_manifest_whose_entities_would_expand_is_refused_unexpanded() {
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let mut files = spec.build();
    let manifest =
        String::from_utf8_lossy(files.get("Lighthouse.scrivx").unwrap_or_default()).into_owned();
    let value = "a".repeat(64 << 10);
    let bomb = manifest
        .replacen(
            "<ScrivenerProject",
            &format!("<!DOCTYPE ScrivenerProject [<!ENTITY e \"{value}\">]>\n<ScrivenerProject"),
            1,
        )
        .replacen(
            "<Title>Arrival</Title>",
            &format!("<Title>{}</Title>", "&e;".repeat(4096)),
            1,
        );
    assert!(bomb.len() < 100 << 10);
    files.put("Lighthouse.scrivx", bomb);
    files.remove("Files/binder.autosave");
    let dir = temp();
    let folder = write_files(&files, dir.path(), "Lighthouse.scriv");
    let refused = refusal(run(&folder));
    assert!(
        matches!(
            refused,
            Refusal::DamagedManifest {
                damage: ManifestDamage::DeclaresEntities,
                ..
            }
        ),
        "{refused:?}"
    );
}

/// A manifest of a hundred thousand and one bare items: 2 MB of XML, and a
/// binder row of a kilobyte each once read. No real project comes near (the
/// largest seen holds 2,509 items), so the manifest is refused while it is read,
/// before its rows fill memory.
#[test]
fn a_binder_past_any_real_one_is_refused_before_it_fills_memory() {
    let mut manifest = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ScrivenerProject Version=\"2.0\" Creator=\"SCRMAC-3.5.2-17487\"><Binder>",
    );
    for _ in 0..=100_000 {
        manifest.push_str("<BinderItem ID=\"1\"/>");
    }
    manifest.push_str("</Binder></ScrivenerProject>");
    let mut files = Files::new();
    files.put("Many.scrivx", manifest);
    files.put("Files/version.txt", "23");
    files.put_dir("Files/Data");
    let dir = temp();
    let folder = write_files(&files, dir.path(), "Many.scriv");
    let refused = refusal(run(&folder));
    assert!(
        matches!(
            refused,
            Refusal::DamagedManifest {
                damage: ManifestDamage::TooManyItems { limit: 100_000 },
                ..
            }
        ),
        "{refused:?}"
    );
}

/// Several manifests are each read to tell them apart, but not all held at once:
/// past what one project's manifests may list together, the rest are not read,
/// and say so, rather than a folder of copies taking memory a copy at a time.
#[test]
fn several_huge_manifests_are_not_all_held_in_memory() {
    let mut files = Files::new();
    for copy in 0..5 {
        let mut manifest = String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ScrivenerProject Version=\"2.0\" Creator=\"SCRMAC-3.5.2-17487\"><Binder>",
        );
        for i in 0..90_000u32 {
            manifest.push_str(&format!("<BinderItem ID=\"{}\"/>", i + copy));
        }
        manifest.push_str("</Binder></ScrivenerProject>");
        files.put(format!("Copy {copy}.scrivx"), manifest);
    }
    files.put("Files/version.txt", "23");
    files.put_dir("Files/Data");
    let dir = temp();
    let folder = write_files(&files, dir.path(), "Many.scriv");
    let Question::SeveralManifests { candidates, .. } = question(run(&folder)) else {
        panic!("expected the manifests to be asked about");
    };
    let counted: Vec<Option<u32>> = candidates.iter().map(|c| c.items).collect();
    assert!(counted.contains(&None), "{counted:?}");
}

/// Choosing among several manifests compares each copy's items with the chosen
/// one's documents. Twenty thousand items against twenty thousand documents must
/// take a moment, not a quadratic hour: the comparison lists the documents once.
#[test]
fn choosing_among_manifest_copies_takes_linear_time() {
    const ITEMS: u32 = 20_000;
    let best = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ScrivenerProject Version=\"1.0\" Creator=\"SCRMAC-2.5-25238\"><Binder><BinderItem ID=\"1\" Type=\"Text\"><Title>One</Title></BinderItem></Binder></ScrivenerProject>";
    let mut copy = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ScrivenerProject Version=\"1.0\" Creator=\"SCRMAC-2.5-25238\"><Binder>",
    );
    for i in 0..ITEMS {
        copy.push_str(&format!(
            "<BinderItem ID=\"{}\" Type=\"Text\"/>",
            1_000_000 + i
        ));
    }
    copy.push_str("</Binder></ScrivenerProject>");
    let mut files = Files::new();
    files.put("X.scrivx", best);
    files.put("X 2.scrivx", copy);
    files.put(
        "Files/binder.autosave",
        scrivener_fixtures::project::binder_copy("X.scrivx", best.as_bytes()),
    );
    files.put("Files/version.txt", "16");
    for i in 0..ITEMS {
        files.put(format!("Files/Docs/{}.rtf", i + 2), b"{\\rtf1 x}".to_vec());
    }
    let dir = temp();
    let zip = match files.zip(Some("X.scriv")) {
        Ok(zip) => zip,
        Err(e) => panic!("zipping the fixture: {e}"),
    };
    let path = write_file(&zip, dir.path(), "X.zip");
    let started = std::time::Instant::now();
    let read = read(run(&path));
    let seconds = started.elapsed().as_secs_f64();
    assert_eq!(read.project.binder.len(), 1);
    assert!(seconds < 20.0, "{seconds:.1} s");
}

/// An archive at the member ceiling, every member in one folder: indexing it
/// must take a moment. Learning whether each name was new by searching the
/// folder's names so far took a minute and more.
#[test]
fn an_archive_at_the_member_ceiling_is_indexed_in_linear_time() {
    let manifest = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ScrivenerProject Version=\"2.0\" Creator=\"SCRMAC-3.5.2-17487\"><Binder><BinderItem UUID=\"4A1D6B18-0952-42EB-A487-D17915DD3BA1\" Type=\"Text\"><Title>One</Title></BinderItem></Binder></ScrivenerProject>";
    let mut entries = vec![
        RawEntry::file("Wide.scriv/Wide.scrivx", manifest.as_bytes()),
        RawEntry::file("Wide.scriv/Files/version.txt", b"23"),
    ];
    for i in 0..99_990u32 {
        entries.push(RawEntry::file(
            &format!("Wide.scriv/Files/Junk/{i}.bin"),
            b"",
        ));
    }
    let dir = temp();
    let path = write_file(&raw_zip(&entries), dir.path(), "wide.zip");
    let started = std::time::Instant::now();
    let read = read(run(&path));
    let seconds = started.elapsed().as_secs_f64();
    assert_eq!(read.project.binder.len(), 1);
    assert!(seconds < 20.0, "{seconds:.1} s");
}

#[cfg(unix)]
mod on_unix {
    use std::os::unix::fs::{PermissionsExt, symlink};

    use scrivener_fixtures::{Generation, Platform, Spec};
    use scrivener_import::{Notice, Refusal};

    use crate::support::{read, refusal, run, temp, write, write_file};

    /// Running as root, permissions are not enforced and the unreadable-file
    /// tests prove nothing.
    fn permissions_hold(dir: &std::path::Path) -> bool {
        let probe = dir.join("probe");
        let _ = std::fs::write(&probe, b"x");
        let _ = std::fs::set_permissions(&probe, std::fs::Permissions::from_mode(0o000));
        let blocked = std::fs::read(&probe).is_err();
        let _ = std::fs::set_permissions(&probe, std::fs::Permissions::from_mode(0o644));
        blocked
    }

    /// A document that is a link is never followed, even to a file of the
    /// project: the import is refused, naming it.
    #[test]
    fn a_needed_document_that_is_a_link_refuses_the_import() {
        let dir = temp();
        let spec = Spec::novel(Generation::Format20, Platform::Mac);
        let folder = write(&spec, dir.path());
        let arrival = spec
            .items
            .iter()
            .position(|i| i.title.as_deref() == Some("Arrival"))
            .unwrap_or(0);
        let text = folder.join(spec.text_path(arrival));
        let outside = dir.path().join("elsewhere.rtf");
        let _ = std::fs::rename(&text, &outside);
        let _ = symlink(&outside, &text);
        assert_eq!(
            refusal(run(&folder)),
            Refusal::NeededMemberIsLink {
                member: spec.text_path(arrival)
            }
        );
    }

    /// An item's whole folder replaced by a link: its writing may be behind it,
    /// so the import is refused rather than reading the document as empty.
    #[test]
    fn an_item_folder_that_is_a_link_refuses_the_import() {
        let dir = temp();
        let spec = Spec::novel(Generation::Format20, Platform::Mac);
        let folder = write(&spec, dir.path());
        let arrival = spec
            .items
            .iter()
            .position(|i| i.title.as_deref() == Some("Arrival"))
            .unwrap_or(0);
        let item = folder.join(spec.payload_prefix(arrival).trim_end_matches('/'));
        let outside = dir.path().join("moved-away");
        let _ = std::fs::rename(&item, &outside);
        let _ = symlink(&outside, &item);
        assert!(matches!(
            refusal(run(&folder)),
            Refusal::NeededMemberIsLink { member } if member == spec.payload_prefix(arrival).trim_end_matches('/')
        ));
    }

    /// A link anywhere else in the project (a research folder pointing at the
    /// writer's pictures) is reported and never followed.
    #[test]
    fn a_link_elsewhere_is_reported_and_never_followed() {
        let dir = temp();
        let folder = write(
            &Spec::novel(Generation::Format20, Platform::Mac),
            dir.path(),
        );
        let _ = symlink("/", folder.join("Settings").join("everything"));
        let read = read(run(&folder));
        assert!(
            read.notices
                .iter()
                .any(|n| matches!(n, Notice::SymlinkNotFollowed { count: 1, .. }))
        );
    }

    /// A folder the import needs that cannot be listed refuses; one it does not
    /// need is reported and skipped.
    #[test]
    fn unreadable_folders_refuse_only_when_needed() {
        let dir = temp();
        if !permissions_hold(dir.path()) {
            return;
        }
        let spec = Spec::novel(Generation::Format20, Platform::Mac);
        let folder = write(&spec, dir.path());
        let arrival = spec
            .items
            .iter()
            .position(|i| i.title.as_deref() == Some("Arrival"))
            .unwrap_or(0);
        let item = folder.join(spec.payload_prefix(arrival));
        let _ = std::fs::set_permissions(&item, std::fs::Permissions::from_mode(0o000));
        let outcome = run(&folder);
        let _ = std::fs::set_permissions(&item, std::fs::Permissions::from_mode(0o755));
        assert!(matches!(
            refusal(outcome),
            Refusal::UnreadableMembers { count: 1, .. }
        ));

        let quicklook = folder.join("QuickLook");
        let _ = std::fs::set_permissions(&quicklook, std::fs::Permissions::from_mode(0o000));
        let outcome = run(&folder);
        let _ = std::fs::set_permissions(&quicklook, std::fs::Permissions::from_mode(0o755));
        assert!(
            read(outcome)
                .notices
                .iter()
                .any(|n| matches!(n, Notice::UnreadableFilesSkipped { .. }))
        );
    }

    /// The writer may pick a link: a manifest, an archive or a project folder
    /// linked from somewhere handier. The pick is followed to where it leads,
    /// and the project read there, as if picked there.
    #[test]
    fn a_picked_link_is_followed_to_the_project() {
        let dir = temp();
        let spec = Spec::novel(Generation::Format20, Platform::Mac);
        let folder = write(&spec, dir.path().join("Writing").as_path());
        let zip = write_file(
            &spec.build().zip(Some(&spec.folder())).unwrap_or_default(),
            dir.path(),
            "Lighthouse.zip",
        );
        let desk = dir.path().join("Desk");
        let _ = std::fs::create_dir_all(&desk);
        for (link, target) in [
            ("manifest link.scrivx", folder.join("Lighthouse.scrivx")),
            ("archive link.zip", zip),
            ("folder link", folder.clone()),
        ] {
            let at = desk.join(link);
            let _ = symlink(&target, &at);
            assert!(
                scrivener_import::looks_like_a_scrivener_project(&at)
                    == (link != "archive link.zip"),
                "{link}"
            );
            let outcome = run(&at);
            assert!(
                matches!(&outcome, scrivener_import::Analysis::Read(_)),
                "{link}: {}",
                crate::support::summary(&outcome)
            );
        }
    }

    /// A document the import needs that is a FIFO (or a device, or a socket) is
    /// never opened, and never taken for an empty document either: the import is
    /// refused, naming it.
    #[test]
    fn a_needed_document_that_is_a_fifo_refuses_the_import() {
        let dir = temp();
        let spec = Spec::novel(Generation::Format20, Platform::Mac);
        let folder = write(&spec, dir.path());
        let arrival = spec
            .items
            .iter()
            .position(|i| i.title.as_deref() == Some("Arrival"))
            .unwrap_or(0);
        let text = folder.join(spec.text_path(arrival));
        let _ = std::fs::remove_file(&text);
        let made = std::process::Command::new("mkfifo").arg(&text).status();
        if !made.is_ok_and(|s| s.success()) {
            return;
        }
        let outcome = run(&folder);
        assert!(
            matches!(
                &outcome,
                scrivener_import::Analysis::Refused {
                    refusal: Refusal::UnreadableMembers { count: 1, .. },
                    ..
                }
            ),
            "{}",
            crate::support::summary(&outcome)
        );
    }

    /// Folders that cannot be listed are remembered so a missing document can
    /// say which one hid it. Thousands of them, and thousands of documents to look
    /// for, must still take a moment: each document asks after its own folders,
    /// not after every unreadable folder in the project.
    #[test]
    fn many_unreadable_folders_are_looked_through_in_linear_time() {
        const COUNT: u32 = 4_000;
        let dir = temp();
        if !permissions_hold(dir.path()) {
            return;
        }
        let folder = dir.path().join("Locked.scriv");
        let mut manifest = String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ScrivenerProject Version=\"2.0\" Creator=\"SCRMAC-3.5.2-17487\"><Binder>",
        );
        for i in 0..COUNT {
            manifest.push_str(&format!(
                "<BinderItem UUID=\"{}\" Type=\"Text\"/>",
                scrivener_fixtures::project::uuid_for(7, i)
            ));
        }
        manifest.push_str("</Binder></ScrivenerProject>");
        let _ = std::fs::create_dir_all(folder.join("Files/Data"));
        let _ = std::fs::write(folder.join("Locked.scrivx"), manifest);
        let _ = std::fs::write(folder.join("Files/version.txt"), "23");
        let locked = folder.join("Settings/Locked");
        for i in 0..COUNT {
            let one = locked.join(format!("{i}"));
            let _ = std::fs::create_dir_all(&one);
            let _ = std::fs::set_permissions(&one, std::fs::Permissions::from_mode(0o000));
        }
        let started = std::time::Instant::now();
        let outcome = run(&folder);
        let seconds = started.elapsed().as_secs_f64();
        for i in 0..COUNT {
            let _ = std::fs::set_permissions(
                locked.join(format!("{i}")),
                std::fs::Permissions::from_mode(0o755),
            );
        }
        assert!(
            read(outcome)
                .notices
                .iter()
                .any(|n| matches!(n, Notice::UnreadableFilesSkipped { .. }))
        );
        assert!(seconds < 10.0, "{seconds:.1} s");
    }

    /// A FIFO in the project is never opened (opening one would block the read
    /// for ever): it is skipped and reported.
    #[test]
    fn a_fifo_is_never_opened() {
        let dir = temp();
        let folder = write(
            &Spec::novel(Generation::Format20, Platform::Mac),
            dir.path(),
        );
        let fifo = folder.join("Files").join("pipe");
        let made = std::process::Command::new("mkfifo").arg(&fifo).status();
        if !made.is_ok_and(|s| s.success()) {
            return;
        }
        let read = read(run(&folder));
        assert!(
            read.notices
                .iter()
                .any(|n| matches!(n, Notice::UnreadableFilesSkipped { .. }))
        );
    }
}

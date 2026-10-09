// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Scrivener's compile settings, read for the little the import takes from them.
//!
//! Each generation keeps them in its own file, and the import reads each only as far as it
//! must:
//!
//! * `Settings/compile.xml` (format 2.0): the project's title, abbreviated title, authors,
//!   subjects, company and language (`ProjectSettings/MetaData`), whether a compile removes
//!   struck-through text (`ProjectSettings/Options/DeleteStruckThroughText`), the
//!   replacements a compile makes (`ProjectSettings/Replacements`), and which compile format
//!   was used last (`LastUsedFormats` under `CurrentFileType`).
//! * `Settings/compile.plist` (Scrivener 2 for Mac, an XML property list): the e-book
//!   language (`SCRCompileEBookLanguageKey`), the struck-through switch
//!   (`SCRCompileDeleteStruckThroughTextKey`) and the variable replacements
//!   (`SCRCompileVariableReplacementsKey`, `SCRCompileProjectVariableReplacementsKey`: arrays
//!   of `variable` and `replacement` pairs). Its e-book authors and title are not read:
//!   in the corpus they are template residue (the template maker's name in writers'
//!   novels), never the writer's byline.
//! * `Settings/compile.ini` (Scrivener for Windows 1.x and the Linux beta, Qt settings):
//!   the language (`[MetaData] languageCode`), a language's English name.
//! * `Settings/Compile Formats/*.scrformat` (format 2.0): the styles a format deletes from a
//!   compile (`DeleteText`) or passes through as raw markup (`RawMarkup`), by name.
//!
//! Every value is kept as written; [`super::propose`] decides what it means. A file that
//! cannot be read is reported by the analysis like any settings file, and what it would have
//! said is simply not stated.

use crate::model::{CompileAuthor, CompileFile, CompileFormat, CompileReplacement};
use crate::xml::{self, XmlProblem, child, child_text, children, text};

/// The most styles one compile format is read with: far past any real one (the largest
/// measured defines under a hundred), and a bound on what a crafted one costs.
const MAX_FORMAT_STYLES: usize = 10_000;

/// The most characters of a style name kept.
const MAX_NAME_CHARS: usize = 200;

/// The most authors read from a compile's metadata: far past any byline (the corpus's
/// longest names one), and a bound on what a crafted list costs and proposes.
pub(crate) const MAX_AUTHORS: usize = 100;

/// The most subjects and the most replacements read from one compile's settings: the
/// corpus's largest holds six replacements and three subjects.
pub(crate) const MAX_LISTED: usize = 1_000;

/// What `Settings/compile.xml` states.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct CompileXml {
    pub(crate) title: Option<String>,
    pub(crate) abbreviated_title: Option<String>,
    pub(crate) authors: Vec<CompileAuthor>,
    pub(crate) surname: Option<String>,
    pub(crate) forename: Option<String>,
    pub(crate) language: Option<String>,
    pub(crate) delete_struck_through: Option<bool>,
    pub(crate) last_format: Option<String>,
    pub(crate) subjects: Vec<String>,
    pub(crate) company: Option<String>,
    pub(crate) replacements: Vec<CompileReplacement>,
}

/// Read `Settings/compile.xml`.
pub(crate) fn read_xml(part: &str, bytes: &[u8]) -> Result<CompileXml, XmlProblem> {
    let text_of = xml::text_of(bytes)?;
    let doc = xml::parse(part, text_of)?;
    let root = doc.root_element();
    let mut out = CompileXml::default();
    let settings = child(root, "ProjectSettings");
    if let Some(meta) = settings.and_then(|s| child(s, "MetaData")) {
        out.title = child_text(meta, "ProjectTitle");
        out.abbreviated_title = child_text(meta, "ProjectAbbreviatedTitle");
        out.surname = child_text(meta, "Surname");
        out.forename = child_text(meta, "Forename");
        out.language = child_text(meta, "EbookLanguage");
        out.company = child_text(meta, "Company");
        if let Some(subjects) = child(meta, "Subjects") {
            out.subjects = children(subjects, "Subject")
                .take(MAX_LISTED)
                .filter_map(text)
                .collect();
        }
        if let Some(authors) = child(meta, "Authors") {
            out.authors = children(authors, "Author")
                .take(MAX_AUTHORS)
                .map(|a| CompileAuthor {
                    name: text(a).unwrap_or_default(),
                    role: a.attribute("Role").map(str::to_string),
                    file_as: a.attribute("FileAs").map(str::to_string),
                })
                .collect();
        }
    }
    if let Some(replacements) = settings.and_then(|s| child(s, "Replacements")) {
        let yes = |node: roxmltree::Node<'_, '_>, name: &str| {
            node.attribute(name)
                .is_some_and(|v| v.trim().eq_ignore_ascii_case("yes"))
        };
        out.replacements = children(replacements, "Replacement")
            .take(MAX_LISTED)
            .map(|r| CompileReplacement {
                file: CompileFile::Xml,
                replace: child_text(r, "Replace").unwrap_or_default(),
                with: child_text(r, "With").unwrap_or_default(),
                regex: yes(r, "RegEx"),
                ignored: yes(r, "Ignore"),
            })
            .collect();
    }
    out.delete_struck_through = settings
        .and_then(|s| child(s, "Options"))
        .and_then(|o| child_text(o, "DeleteStruckThroughText"))
        .map(|v| v.trim().eq_ignore_ascii_case("yes"));
    let current = child_text(root, "CurrentFileType").map(|t| t.trim().to_string());
    out.last_format = match (current, child(root, "LastUsedFormats")) {
        (Some(kind), Some(last)) if !kind.is_empty() => last
            .children()
            .find(|c| c.is_element() && c.tag_name().name() == kind)
            .and_then(text)
            .map(|id| id.trim().to_string())
            .filter(|id| !id.is_empty()),
        _ => None,
    };
    Ok(out)
}

/// What `Settings/compile.plist` states: the e-book language, the struck-through switch and
/// the variable replacements.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct CompilePlist {
    pub(crate) language: Option<String>,
    pub(crate) delete_struck_through: Option<bool>,
    pub(crate) replacements: Vec<CompileReplacement>,
}

/// Read `Settings/compile.plist`, an XML property list: one top-level `dict` of `key`
/// elements, each followed by its value.
pub(crate) fn read_plist(part: &str, bytes: &[u8]) -> Result<CompilePlist, XmlProblem> {
    let text_of = xml::text_of(bytes)?;
    let doc = xml::parse(part, text_of)?;
    let mut out = CompilePlist::default();
    let Some(dict) = child(doc.root_element(), "dict") else {
        return Ok(out);
    };
    let mut entries = dict.children().filter(|c| c.is_element());
    while let Some(node) = entries.next() {
        if node.tag_name().name() != "key" {
            continue;
        }
        let key = text(node).unwrap_or_default();
        let Some(value) = entries.next() else {
            break;
        };
        match key.trim() {
            "SCRCompileEBookLanguageKey" if value.tag_name().name() == "string" => {
                out.language = text(value);
            }
            "SCRCompileVariableReplacementsKey" | "SCRCompileProjectVariableReplacementsKey"
                if value.tag_name().name() == "array" =>
            {
                for pair in children(value, "dict").take(MAX_LISTED) {
                    let (mut variable, mut replacement) = (None, None);
                    let mut fields = pair.children().filter(|c| c.is_element());
                    while let Some(field) = fields.next() {
                        if field.tag_name().name() != "key" {
                            continue;
                        }
                        let Some(value) = fields.next() else {
                            break;
                        };
                        if value.tag_name().name() != "string" {
                            continue;
                        }
                        match text(field).unwrap_or_default().trim() {
                            "variable" => variable = Some(text(value).unwrap_or_default()),
                            "replacement" => replacement = Some(text(value).unwrap_or_default()),
                            _ => {}
                        }
                    }
                    if let Some(replace) = variable.filter(|v| !v.trim().is_empty()) {
                        out.replacements.push(CompileReplacement {
                            file: CompileFile::Plist,
                            replace,
                            with: replacement.unwrap_or_default(),
                            regex: false,
                            ignored: false,
                        });
                    }
                }
            }
            "SCRCompileDeleteStruckThroughTextKey" => {
                out.delete_struck_through = match value.tag_name().name() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                };
            }
            _ => {}
        }
    }
    Ok(out)
}

/// The language `Settings/compile.ini` states: `[MetaData] languageCode`, a language's
/// English name as Qt spells it (`English`, `French`). Qt settings are text in the system's
/// encoding; only an ASCII name is read, so an undecodable file states nothing.
pub(crate) fn read_ini_language(bytes: &[u8]) -> Option<String> {
    let text = String::from_utf8_lossy(bytes);
    let mut section = String::new();
    for line in text.lines() {
        let line = line.trim();
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            section = name.trim().to_string();
            continue;
        }
        if !section.eq_ignore_ascii_case("MetaData") {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if key.trim() == "languageCode" {
            let value = value.trim().trim_matches('"').trim();
            return (!value.is_empty() && value.chars().all(|c| c.is_ascii_alphanumeric()))
                .then(|| value.to_string());
        }
    }
    None
}

/// Read one compile format saved inside the project: its id, and the styles it deletes from
/// a compile or passes through as raw markup.
pub(crate) fn read_format(part: &str, bytes: &[u8]) -> Result<CompileFormat, XmlProblem> {
    let text_of = xml::text_of(bytes)?;
    let doc = xml::parse(part, text_of)?;
    let root = doc.root_element();
    let mut out = CompileFormat {
        id: root.attribute("ID").unwrap_or_default().trim().to_string(),
        ..CompileFormat::default()
    };
    let Some(styles) = child(root, "Styles") else {
        return Ok(out);
    };
    let yes = |style: roxmltree::Node<'_, '_>, name: &str| {
        child_text(style, name).is_some_and(|v| v.trim().eq_ignore_ascii_case("yes"))
    };
    for style in children(styles, "Style").take(MAX_FORMAT_STYLES) {
        let name: String = style
            .attribute("Name")
            .unwrap_or_default()
            .trim()
            .chars()
            .take(MAX_NAME_CHARS)
            .collect();
        if name.is_empty() {
            continue;
        }
        if yes(style, "DeleteText") {
            out.deleted_styles.push(name);
        } else if yes(style, "RawMarkup") {
            out.raw_styles.push(name);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_xml_states_title_authors_language_and_the_struck_through_switch() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<CompileSettings>
    <ProjectSettings>
        <Options>
            <RemoveAnnotations>Yes</RemoveAnnotations>
            <DeleteStruckThroughText>Yes</DeleteStruckThroughText>
        </Options>
        <MetaData>
            <ProjectTitle>Les Veilleurs</ProjectTitle>
            <ProjectAbbreviatedTitle>Veilleurs</ProjectAbbreviatedTitle>
            <Authors>
                <Author FileAs="Doe, Jane" Role="aut">Jane Doe</Author>
                <Author Role="edt">Sam Roe</Author>
            </Authors>
            <Surname>Doe</Surname>
            <Forename>Jane</Forename>
            <EbookLanguage>fr</EbookLanguage>
            <Company>Harbour Press</Company>
            <Subjects>
                <Subject>Sea</Subject>
                <Subject>Lights</Subject>
            </Subjects>
        </MetaData>
        <Replacements>
            <Replacement Ignore="Yes" RegEx="Yes">
                <Replace>\s+\[@</Replace>
                <With>[@</With>
            </Replacement>
            <Replacement>
                <Replace>{Name}</Replace>
                <With>Mara</With>
            </Replacement>
        </Replacements>
    </ProjectSettings>
    <CurrentFileType>txt</CurrentFileType>
    <LastUsedFormats>
        <rtf>manuscript-courier</rtf>
        <txt>A3AAA52A-1166-4FF7-8952-3742980C806D</txt>
    </LastUsedFormats>
</CompileSettings>"#;
        let read = read_xml("compile.xml", xml.as_bytes()).expect("reads");
        assert_eq!(read.title.as_deref(), Some("Les Veilleurs"));
        assert_eq!(read.abbreviated_title.as_deref(), Some("Veilleurs"));
        assert_eq!(read.authors.len(), 2);
        assert_eq!(read.authors[0].role.as_deref(), Some("aut"));
        assert_eq!(read.authors[0].file_as.as_deref(), Some("Doe, Jane"));
        assert_eq!(read.authors[1].role.as_deref(), Some("edt"));
        assert_eq!(read.language.as_deref(), Some("fr"));
        assert_eq!(read.delete_struck_through, Some(true));
        assert_eq!(
            read.last_format.as_deref(),
            Some("A3AAA52A-1166-4FF7-8952-3742980C806D")
        );
        assert_eq!(read.company.as_deref(), Some("Harbour Press"));
        assert_eq!(read.subjects, ["Sea", "Lights"]);
        assert_eq!(
            read.replacements,
            [
                CompileReplacement {
                    file: CompileFile::Xml,
                    replace: r"\s+\[@".to_string(),
                    with: "[@".to_string(),
                    regex: true,
                    ignored: true,
                },
                CompileReplacement {
                    file: CompileFile::Xml,
                    replace: "{Name}".to_string(),
                    with: "Mara".to_string(),
                    regex: false,
                    ignored: false,
                },
            ]
        );
    }

    #[test]
    fn an_empty_compile_xml_states_nothing() {
        let read = read_xml(
            "compile.xml",
            b"<CompileSettings><ProjectSettings></ProjectSettings></CompileSettings>",
        )
        .expect("reads");
        assert_eq!(read, CompileXml::default());
        assert!(read_xml("compile.xml", b"<CompileSettings>").is_err());
    }

    #[test]
    fn a_property_list_states_its_language_and_switch() {
        let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>SCRCompileDeleteStruckThroughTextKey</key>
	<true/>
	<key>SCRCompileEBookAuthorsKey</key>
	<array/>
	<key>SCRCompileEBookLanguageKey</key>
	<string>de</string>
	<key>SCRCompileEBookTitleKey</key>
	<string>Novel Template</string>
	<key>SCRCompileVariableReplacementsKey</key>
	<array>
		<dict>
			<key>replacement</key>
			<string>&lt;$n:figure:$@&gt;</string>
			<key>variable</key>
			<string>!fig($@)</string>
		</dict>
	</array>
</dict>
</plist>"#;
        let read = read_plist("compile.plist", plist.as_bytes()).expect("reads");
        assert_eq!(read.language.as_deref(), Some("de"));
        assert_eq!(read.delete_struck_through, Some(true));
        assert_eq!(
            read.replacements,
            [CompileReplacement {
                file: CompileFile::Plist,
                replace: "!fig($@)".to_string(),
                with: "<$n:figure:$@>".to_string(),
                regex: false,
                ignored: false,
            }]
        );
    }

    #[test]
    fn a_qt_ini_states_its_language_in_its_metadata_section_only() {
        let ini = "[General]\nlanguageCode=Klingon\n\n[MetaData]\ntitle=\nlanguageCode=French\n";
        assert_eq!(read_ini_language(ini.as_bytes()).as_deref(), Some("French"));
        assert_eq!(read_ini_language(b"[MetaData]\nlanguageCode=\n"), None);
        assert_eq!(read_ini_language(b"[Other]\nlanguageCode=German\n"), None);
    }

    #[test]
    fn a_format_names_the_styles_it_deletes_and_passes_raw() {
        let format = r#"<?xml version="1.0" encoding="UTF-8"?>
<CompileFormat Name="PreTeXt" ID="A3AAA52A-1166-4FF7-8952-3742980C806D">
    <Styles>
        <Style Name="don't export" ID="B8CEF717-56B6-4440-8EDB-76419640B448" Type="Para+Char">
            <Format><![CDATA[{\rtf1 x}]]></Format>
            <DeleteText>Yes</DeleteText>
        </Style>
        <Style Name="raw-ptx" ID="11111111-2222-3333-4444-555555555555" Type="Para+Char">
            <RawMarkup>Yes</RawMarkup>
        </Style>
        <Style Name="Block Quote" ID="22222222-2222-3333-4444-555555555555" Type="Para"/>
    </Styles>
</CompileFormat>"#;
        let read = read_format("f.scrformat", format.as_bytes()).expect("reads");
        assert_eq!(read.id, "A3AAA52A-1166-4FF7-8952-3742980C806D");
        assert_eq!(read.deleted_styles, ["don't export"]);
        assert_eq!(read.raw_styles, ["raw-ptx"]);
    }
}

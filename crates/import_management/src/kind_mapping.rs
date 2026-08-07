// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Hand-written (do NOT blanket-regenerate): the DTO's flat `ImportRowKind` ↔
// `skribisto_model::CreateType` conversion, shared by `analyze_document_import`
// (builds the DTO the writer reviews) and `apply_document_import` (turns an
// accepted row back into a `CreateType` to resolve its storage combination).
//
// Lives here, outside both use cases, on purpose: no use case ever calls
// another use case in this codebase — shared logic lives in a module both call
// instead (see `work_management::work_io` for the same shape). Before this
// module existed, `apply_document_import_uc` imported these functions straight
// out of `analyze_document_import_uc`, which is exactly the cross-use-case
// reach this rule exists to prevent.

use crate::dtos::ImportRowKind;

/// The DTO's flat kind ↔ the model's create vocabulary.
///
/// Two enums for one idea, because a Qleany DTO cannot name a type from another
/// crate. Kept as an exhaustive `match` in both directions rather than a cast, so
/// adding a `CreateType` fails to compile here instead of silently importing
/// everything as a Scene.
pub(crate) fn kind_to_create_type(kind: &ImportRowKind) -> skribisto_model::CreateType {
    use skribisto_model::CreateType as C;
    match kind {
        ImportRowKind::Book => C::Book,
        ImportRowKind::Part => C::Part,
        ImportRowKind::Chapter => C::Chapter,
        ImportRowKind::Scene => C::Scene,
        ImportRowKind::Note => C::Note,
        ImportRowKind::NoteFolder => C::NoteFolder,
        ImportRowKind::Folder => C::Folder,
        ImportRowKind::Paratext => C::Paratext,
        ImportRowKind::ParatextFolder => C::ParatextFolder,
        ImportRowKind::EndOfBook => C::EndOfBook,
    }
}

pub(crate) fn create_type_to_kind(kind: skribisto_model::CreateType) -> ImportRowKind {
    use skribisto_model::CreateType as C;
    match kind {
        C::Book => ImportRowKind::Book,
        C::Part => ImportRowKind::Part,
        C::Chapter => ImportRowKind::Chapter,
        C::Scene => ImportRowKind::Scene,
        C::Note => ImportRowKind::Note,
        C::NoteFolder => ImportRowKind::NoteFolder,
        C::Folder => ImportRowKind::Folder,
        C::Paratext => ImportRowKind::Paratext,
        C::ParatextFolder => ImportRowKind::ParatextFolder,
        C::EndOfBook => ImportRowKind::EndOfBook,
    }
}

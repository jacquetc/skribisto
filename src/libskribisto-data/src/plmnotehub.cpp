/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmnotehub.cpp                                                   *
*  This file is part of Skribisto.                                    *
*                                                                         *
*  Skribisto is free software: you can redistribute it and/or modify  *
*  it under the terms of the GNU General Public License as published by   *
*  the Free Software Foundation, either version 3 of the License, or      *
*  (at your option) any later version.                                    *
*                                                                         *
*  Skribisto is distributed in the hope that it will be useful,       *
*  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
*  GNU General Public License for more details.                           *
*                                                                         *
*  You should have received a copy of the GNU General Public License      *
*  along with Skribisto.  If not, see <http://www.gnu.org/licenses/>. *
***************************************************************************/
#include "plmnotehub.h"
#include "tasks/plmsqlqueries.h"
#include "tools.h"
#include "plmdata.h"
#include <QDebug>

PLMNoteHub::PLMNoteHub(QObject *parent) : PLMPaperHub(parent, "tbl_note")
{}

PLMError PLMNoteHub::addNoteRelatedToSheet(int projectId, int sheetId) {
    PLMError error;

    error = this->addChildPaper(projectId, -1); // add child to project item


    IFOK(error) {
        int lastAddedNoteId = this->getLastAddedId();

        error = this->setSheetNoteRelationship(projectId, sheetId, lastAddedNoteId);

        error.addData("noteId", lastAddedNoteId);
    }
    return error;
}

// --------------------------------------------------------------------------------

QList<int>PLMNoteHub::getNotesFromSheetId(int projectId, int sheetId) const
{
    PLMError   error;
    QList<int> result;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, "tbl_sheet_note");

    error = queries.getValueByIds("l_note_code", out, "l_sheet_code", sheetId);

    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntInt(out).values();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

// --------------------------------------------------------------------------------

QList<int>PLMNoteHub::getSheetsFromNoteId(int projectId, int noteId) const
{
    PLMError   error;
    QList<int> result;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, "tbl_sheet_note");

    error = queries.getValueByIds("l_sheet_code", out, "l_note_code", noteId);

    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntInt(out).values();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

// --------------------------------------------------------------------------------

int PLMNoteHub::getSynopsisNoteId(int projectId, int sheetId) const
{
    PLMError error;
    int final = -2;
    QHash<int, int> result;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("b_synopsis",   true);
    where.insert("l_sheet_code", sheetId);

    PLMSqlQueries queries(projectId, "tbl_sheet_note");

    error = queries.getValueByIdsWhere("l_note_code", out, where);

    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator i = result.constBegin();

        while (i != result.constEnd()) {
            final = i.value();
            ++i;
        }

        if (result.isEmpty() || (final == 0)) {
            final = -2;
        }
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return final;
}

// --------------------------------------------------------------------------------

PLMError PLMNoteHub::setSheetNoteRelationship(int  projectId,
                                              int  sheetId,
                                              int  noteId,
                                              bool isSynopsis)
{
    PLMError error;

    QHash<int, int> result;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_sheet_code", sheetId);
    where.insert("l_note_code",  noteId);

    PLMSqlQueries queries(projectId, "tbl_sheet_note");


    // verify if the relationship doesn't yet exist
    error = queries.getValueByIdsWhere("l_sheet_note_id", out, where);

    int key = -2;

    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator i = result.constBegin();

        while (i != result.constEnd()) {
            key = i.key();
            ++i;
        }

        if (result.isEmpty() || (key == -2) || (key == 0)) {
            // no relationship exists, creating one

            int newId = -2;
            QHash<QString, QVariant> values;
            values.insert("l_sheet_code", sheetId);
            values.insert("l_note_code",  noteId);
            values.insert("b_synopsis",   isSynopsis);
            error = queries.add(values, newId);

            IFOK(error) {
                emit sheetNoteRelationshipAdded(projectId, sheetId, noteId);
                emit projectModified(projectId);
            }
        }
        else {
            // relationship exists, verifying b_synopsis

            QVariant variantResult;
            error = queries.get(key, "b_synopsis", variantResult);

            if (variantResult.toBool() == isSynopsis) {
                // nothing to do, quiting
                return error;
            }
            else {
                // set b_synopsis:

                error =  queries.set(key, "b_synopsis", isSynopsis);

                IFOK(error) {
                    emit sheetNoteRelationshipChanged(projectId, sheetId, noteId);
                    emit projectModified(projectId);
                }
            }
        }
    }
    return error;
}

// --------------------------------------------------------------------------------

PLMError PLMNoteHub::removeSheetNoteRelationship(int projectId, int sheetId, int noteId)
{
    PLMError error;

    QHash<int, int> result;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_sheet_code", sheetId);

    PLMSqlQueries queries(projectId, "tbl_sheet_note");

    error = queries.getValueByIdsWhere("l_note_code", out, where);

    int key = -2;

    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator i = result.constBegin();

        while (i != result.constEnd()) {
            key = i.key();
            ++i;
        }

        if (result.isEmpty() || (key == -2) || (key == 0)) {
            error.setSuccess(false);
            error.setErrorCode("E_TAG_no_note_sheet_relationship_to_remove");
            return error;
        }
    }

    IFOK(error) {
        error = queries.remove(key);
    }
    IFOK(error) {
        emit sheetNoteRelationshipRemoved(projectId, sheetId, noteId);
        emit projectModified(projectId);
    }

    return error;
}

// --------------------------------------------------------------------------------
///
/// \brief PLMNoteHub::getSynopsisFolderId
/// \param projectId
/// \return
/// Retrieve note id used as parent of synopsis. Create one if none found and
// move all
/// existing synopsis into the folder
int PLMNoteHub::getSynopsisFolderId(int projectId) {
    PLMError error;
    int result = -2;

    // retrieve

    int retrievedId = -2;

    for (const int noteId : getAllIds(projectId)) {
        QString propertyValue = plmdata->notePropertyHub()->getProperty(projectId,
                                                                        noteId,
                                                                        "is_synopsis_folder",
                                                                        QString());

        if (propertyValue == "true") {
            retrievedId = noteId;
            break;
        }
    }
    int folderId = retrievedId;

    // if none, create one
    if (folderId == -2) {
        error = this->addChildPaper(projectId, -1);

        IFOK(error) {
            folderId = error.getData("paperId", -2).toInt();

            // set properties :
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "is_renamable",
                                                    "false");
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "is_movable",
                                                    "false");
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "is_trashable",
                                                    "false");
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "can_add_paper",
                                                    "false");
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "is_openable",
                                                    "false");
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "is_copyable",
                                                    "false");
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "is_synopsis_folder",
                                                    "true");
        }


        // move all synopsis into the folder
        IFOK(error) {
            QList<int> synList;

            for (int noteId : getAllIds(projectId)) {
                bool syn = isSynopsis(projectId, noteId);

                if (syn) {
                    int parentId = getParentId(projectId, noteId);

                    if (parentId != folderId) {
                        synList << noteId;
                    }
                }
            }

            for (int synopsisId : synList) {
                qDebug() << "synList " << synopsisId;
                error = this->movePaperAsChildOf(projectId, synopsisId, folderId);

                // set properties :
                plmdata->notePropertyHub()->setProperty(projectId,
                                                        synopsisId,
                                                        "is_renamable",
                                                        "false");
                plmdata->notePropertyHub()->setProperty(projectId,
                                                        synopsisId,
                                                        "is_movable",
                                                        "false");
                plmdata->notePropertyHub()->setProperty(projectId,
                                                        synopsisId,
                                                        "can_add_paper",
                                                        "false");
                plmdata->notePropertyHub()->setProperty(projectId,
                                                        synopsisId,
                                                        "is_trashable",
                                                        "false");
            }
        }
    }

    IFOK(error) {
        result = folderId;
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

// --------------------------------------------------------------------------------

bool PLMNoteHub::isSynopsis(int projectId, int noteId)
{
    PLMError error;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_note_code", noteId);
    where.insert("b_synopsis",  true);

    QVariant variantResult;
    PLMSqlQueries queries(projectId, "tbl_sheet_note");

    // verify if the relationship doesn't yet exist
    error = queries.getValueByIdsWhere("l_sheet_note_id", out, where);

    int key = -2;

    IFOK(error) {
        QHash<int, int> result = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator i = result.constBegin();

        while (i != result.constEnd()) {
            key = i.key();
            ++i;
        }

        if (result.isEmpty() || (key == -2) || (key == 0)) {
            return false;
        }
        return true;
    }
    IFKO(error) {
        emit errorSent(error);

        return false;
    }

    return true;
}

// --------------------------------------------------------------------------------

PLMError PLMNoteHub::createSynopsis(int projectId, int sheetId) {
    PLMError error;

    int synopsisFolderId = this->getSynopsisFolderId(projectId);

    error = this->addChildPaper(projectId, synopsisFolderId); // add child to
    // project item
    int lastAddedNoteId = -2;

    IFOK(error) {
        lastAddedNoteId = this->getLastAddedId();

        error = this->setSheetNoteRelationship(projectId, sheetId, lastAddedNoteId, true);

        // set properties :
        plmdata->notePropertyHub()->setProperty(projectId,
                                                lastAddedNoteId,
                                                "is_renamable",
                                                "false");
        plmdata->notePropertyHub()->setProperty(projectId,
                                                lastAddedNoteId,
                                                "is_movable",
                                                "false");
        plmdata->notePropertyHub()->setProperty(projectId,
                                                lastAddedNoteId,
                                                "can_add_paper",
                                                "false");
        plmdata->notePropertyHub()->setProperty(projectId,
                                                lastAddedNoteId,
                                                "is_trashable",
                                                "false");
    }

    if (lastAddedNoteId == -2) {
        error.setSuccess(false);
    }
    IFOK(error) {
        error.addData("noteId", lastAddedNoteId);
    }
    return error;
}

// --------------------------------------------------------------------------------

// useful ?
QHash<QString, QVariant>PLMNoteHub::getNoteData(int projectId, int noteId) const
{
    PLMError error;

    QHash<QString, QVariant> var;
    QHash<QString, QVariant> result;
    QStringList fieldNames;

    fieldNames << "l_note_id" << "l_dna" << "l_sort_order" << "l_indent" <<
        "l_version" << "t_title" << "dt_created" << "dt_updated" << "dt_content" <<
        "b_trashed";
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.getMultipleValues(noteId, fieldNames, var);
    IFOK(error) {
        result = var;
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

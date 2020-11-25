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
#include "skr.h"
#include <QDebug>

PLMNoteHub::PLMNoteHub(QObject *parent) : PLMPaperHub(parent, "tbl_note", SKR::Note)
{



}

SKRResult PLMNoteHub::addNoteRelatedToSheet(int projectId, int sheetId) {
    SKRResult result(this);

    result = this->addChildPaper(projectId, -1); // add child to project item


    IFOK(result) {
        int lastAddedNoteId = this->getLastAddedId();

        result = this->setSheetNoteRelationship(projectId, sheetId, lastAddedNoteId);

        result.addData("noteId", lastAddedNoteId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// --------------------------------------------------------------------------------

QList<int>PLMNoteHub::getNotesFromSheetId(int projectId, int sheetId) const
{
    SKRResult   result;
    QList<int> list;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, "tbl_sheet_note");

    result = queries.getValueByIds("l_note_code", out, "l_sheet_code", sheetId);

    IFOK(result) {
        list = HashIntQVariantConverter::convertToIntInt(out).values();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// --------------------------------------------------------------------------------

QList<int>PLMNoteHub::getSheetsFromNoteId(int projectId, int noteId) const
{
    SKRResult   result;
    QList<int> list;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, "tbl_sheet_note");

    result = queries.getValueByIds("l_sheet_code", out, "l_note_code", noteId);

    IFOK(result) {
        list = HashIntQVariantConverter::convertToIntInt(out).values();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// --------------------------------------------------------------------------------

int PLMNoteHub::getSynopsisNoteId(int projectId, int sheetId) const
{
    SKRResult result(this);
    int final = -2;
    QHash<int, int> hash;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("b_synopsis",   true);
    where.insert("l_sheet_code", sheetId);

    PLMSqlQueries queries(projectId, "tbl_sheet_note");

    result = queries.getValueByIdsWhere("l_note_code", out, where);

    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator i = hash.constBegin();

        while (i != hash.constEnd()) {
            final = i.value();
            ++i;
        }

        if (hash.isEmpty() || (final == 0)) {
            final = -2;
        }
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return final;
}

// --------------------------------------------------------------------------------

SKRResult PLMNoteHub::setSheetNoteRelationship(int  projectId,
                                               int  sheetId,
                                               int  noteId,
                                               bool isSynopsis)
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_sheet_code", sheetId);
    where.insert("l_note_code",  noteId);

    PLMSqlQueries queries(projectId, "tbl_sheet_note");


    // verify if the relationship doesn't yet exist
    result = queries.getValueByIdsWhere("l_sheet_note_id", out, where);

    int key = -2;

    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator i = hash.constBegin();

        while (i != hash.constEnd()) {
            key = i.key();
            ++i;
        }

        if (hash.isEmpty() || (key == -2) || (key == 0)) {
            // no relationship exists, creating one

            int newId = -2;
            QHash<QString, QVariant> values;
            values.insert("l_sheet_code", sheetId);
            values.insert("l_note_code",  noteId);
            values.insert("b_synopsis",   isSynopsis);
            result = queries.add(values, newId);

            IFOK(result) {
                emit sheetNoteRelationshipAdded(projectId, sheetId, noteId);
                emit projectModified(projectId);
            }
        }
        else {
            // relationship exists, verifying b_synopsis

            QVariant variantResult;
            result = queries.get(key, "b_synopsis", variantResult);

            if (variantResult.toBool() == isSynopsis) {
                // nothing to do, quiting
                return result;
            }
            else {
                // set b_synopsis:

                result =  queries.set(key, "b_synopsis", isSynopsis);

                IFOK(result) {
                    emit sheetNoteRelationshipChanged(projectId, sheetId, noteId);
                    emit projectModified(projectId);
                }
            }
        }
    }
    return result;
}

// --------------------------------------------------------------------------------

SKRResult PLMNoteHub::removeSheetNoteRelationship(int projectId, int sheetId, int noteId)
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_sheet_code", sheetId);

    PLMSqlQueries queries(projectId, "tbl_sheet_note");

    result = queries.getValueByIdsWhere("l_note_code", out, where);

    int key = -2;

    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator i = hash.constBegin();

        while (i != hash.constEnd()) {
            key = i.key();
            ++i;
        }

        if (hash.isEmpty() || (key == -2) || (key == 0)) {
            result = SKRResult(SKRResult::Critical, this, "no_note_sheet_relationship_to_remove");
        }
    }

    IFOK(result) {
        result = queries.remove(key);
    }
    IFOK(result) {
        emit sheetNoteRelationshipRemoved(projectId, sheetId, noteId);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }

    return result;
}

// --------------------------------------------------------------------------------

int PLMNoteHub::getSheetFromSynopsisId(int projectId, int synopsisId) const
{
    SKRResult   result;
    int final = -2;
    QHash<int, QVariant> out;
    QHash<QString, QVariant> where;
    where.insert("l_note_code", synopsisId);
    where.insert("b_synopsis", true);

    PLMSqlQueries queries(projectId, "tbl_sheet_note");


    result = queries.getValueByIdsWhere("l_sheet_code", out, where);

    if(out.isEmpty()){
        return final;
    }

    IFOK(result) {
        final = out.values().first().toInt();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return final;
}

// --------------------------------------------------------------------------------
///
/// \brief PLMNoteHub::getSynopsisFolderId
/// \param projectId
/// \return
/// Retrieve note id used as parent of synopsis.
int PLMNoteHub::getSynopsisFolderId(int projectId) {
    SKRResult result(this);

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

    // if none
    if (retrievedId == -2) {
        result = SKRResult(SKRResult::Fatal, this, "synopsis_folder_not_found");
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return retrievedId;
}

// --------------------------------------------------------------------------------
///
/// \brief PLMNoteHub::cleanUpSynopsis
/// \param projectId
/// \return
/// Retrieve note id used as parent of synopsis. Create one if none found and move all
/// existing synopsis into the folder. Apply properties to folder and synopsis.
/// Delete unused sysnopsis. Sync titles between sheet and synopsis
SKRResult PLMNoteHub::cleanUpSynopsis(int projectId){

    SKRResult result(this);
    int value = -2;

    // retrieve synopsis folder Id

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
        result = this->addChildPaper(projectId, -1);

        IFOK(result) {
            folderId = result.getData("paperId", -2).toInt();

            // set properties :
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "is_renamable",
                                                    "false",
                                                    true);
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "is_movable",
                                                    "false",
                                                    true);
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "is_trashable",
                                                    "false",
                                                    true);
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "can_add_paper",
                                                    "false",
                                                    true);
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "can_add_sibling_paper",
                                                    "true",
                                                    true);
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "can_add_child_paper",
                                                    "false",
                                                    true);
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "is_openable",
                                                    "false",
                                                    true);
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "is_copyable",
                                                    "false",
                                                    true);
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    folderId,
                                                    "is_synopsis_folder",
                                                    "true",
                                                    true);
            this->addAttribute(projectId, folderId, "locked");
        }


        // move all synopsis into the folder
        IFOK(result) {
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
                //qDebug() << "synList " << synopsisId;
                result = this->movePaperAsChildOf(projectId, synopsisId, folderId);

                // set properties :
                plmdata->notePropertyHub()->setProperty(projectId,
                                                        synopsisId,
                                                        "is_renamable",
                                                        "false",
                                                        true);
                plmdata->notePropertyHub()->setProperty(projectId,
                                                        synopsisId,
                                                        "is_movable",
                                                        "false",
                                                        true);
                plmdata->notePropertyHub()->setProperty(projectId,
                                                        synopsisId,
                                                        "can_add_sibling_paper",
                                                        "false",
                                                        true);
                plmdata->notePropertyHub()->setProperty(projectId,
                                                        synopsisId,
                                                        "can_add_child_paper",
                                                        "false",
                                                        true);
                plmdata->notePropertyHub()->setProperty(projectId,
                                                        synopsisId,
                                                        "is_trashable",
                                                        "false",
                                                        true);
                this->addAttribute(projectId, synopsisId, "synopsis");
            }
        }
    }

    // upgrade properties in synopsis folder

    int propertyId = plmdata->notePropertyHub()->getPropertyId(projectId, folderId, "can_add_paper");

    if(propertyId != -2){
        plmdata->notePropertyHub()->removeProperty(projectId, propertyId);
    }

    plmdata->notePropertyHub()->setProperty(projectId,
                                            folderId,
                                            "can_add_sibling_paper",
                                            "true",
                                            true);
    plmdata->notePropertyHub()->setProperty(projectId,
                                            folderId,
                                            "can_add_child_paper",
                                            "false",
                                            true);

    //all synopsis:
    IFOK(result) {
        QList<int> synList;

        for (int noteId : getAllIds(projectId)) {
            bool syn = isSynopsis(projectId, noteId);

            if (syn) {
                synList << noteId;

            }
        }

        for (int synopsisId : synList) {

            // upgrade properties in all synopsis

            int propertyId = plmdata->notePropertyHub()->getPropertyId(projectId, synopsisId, "can_add_paper");

            if(propertyId != -2){
                plmdata->notePropertyHub()->removeProperty(projectId, propertyId);
            }


            plmdata->notePropertyHub()->setProperty(projectId,
                                                    synopsisId,
                                                    "can_add_sibling_paper",
                                                    "false",
                                                    true);
            plmdata->notePropertyHub()->setProperty(projectId,
                                                    synopsisId,
                                                    "can_add_child_paper",
                                                    "false",
                                                    true);


            // find sheet:
            int sheetId = plmdata->noteHub()->getSheetFromSynopsisId(projectId, synopsisId);
            if(sheetId == -2){
                // delete useless note
                 plmdata->noteHub()->removePaper(projectId, synopsisId);
            }
            else {
                // sync names in all synopsis
                QString sheetTitle = plmdata->sheetHub()->getTitle(projectId, sheetId);
                if(plmdata->noteHub()->getTitle(projectId, synopsisId) != sheetTitle){
                    plmdata->noteHub()->setTitle(projectId, synopsisId, sheetTitle);
                }
            }




        }
    }





    IFOK(result) {
        value = folderId;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}


// --------------------------------------------------------------------------------

bool PLMNoteHub::isSynopsis(int projectId, int noteId)
{
    SKRResult result(this);
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_note_code", noteId);
    where.insert("b_synopsis",  true);

    QVariant variantResult;
    PLMSqlQueries queries(projectId, "tbl_sheet_note");

    // verify if the relationship doesn't yet exist
    result = queries.getValueByIdsWhere("l_sheet_note_id", out, where);

    int key = -2;

    IFOK(result) {
        QHash<int, int> hash = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator i = hash.constBegin();

        while (i != hash.constEnd()) {
            key = i.key();
            ++i;
        }

        if (hash.isEmpty() || (key == -2) || (key == 0)) {
            return false;
        }
        return true;
    }
    IFKO(result) {
        emit errorSent(result);

        return false;
    }

    return true;
}

// --------------------------------------------------------------------------------

SKRResult PLMNoteHub::createSynopsis(int projectId, int sheetId) {
    SKRResult result(this);

    int synopsisFolderId = this->getSynopsisFolderId(projectId);

    result = this->addChildPaper(projectId, synopsisFolderId); // add child to
    // project item
    int lastAddedNoteId = -2;

    IFOK(result) {
        lastAddedNoteId = this->getLastAddedId();

        result = this->setSheetNoteRelationship(projectId, sheetId, lastAddedNoteId, true);

        // set properties :
        plmdata->notePropertyHub()->setProperty(projectId,
                                                lastAddedNoteId,
                                                "is_renamable",
                                                "false",
                                                true);
        plmdata->notePropertyHub()->setProperty(projectId,
                                                lastAddedNoteId,
                                                "is_movable",
                                                "false",
                                                true);
        plmdata->notePropertyHub()->setProperty(projectId,
                                                lastAddedNoteId,
                                                "can_add_sibling_paper",
                                                "false",
                                                true);
        plmdata->notePropertyHub()->setProperty(projectId,
                                                lastAddedNoteId,
                                                "can_add_child_paper",
                                                "false",
                                                true);
        plmdata->notePropertyHub()->setProperty(projectId,
                                                lastAddedNoteId,
                                                "is_trashable",
                                                "false",
                                                true);
        this->addAttribute(projectId, lastAddedNoteId, "synopsis");
    }

    if (lastAddedNoteId == -2) {
        result = SKRResult(SKRResult::Critical, this, "createSynopsis");
    }
    IFOK(result) {
        result.addData("noteId", lastAddedNoteId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// --------------------------------------------------------------------------------

// useful ?
QHash<QString, QVariant>PLMNoteHub::getNoteData(int projectId, int noteId) const
{
    SKRResult result(this);

    QHash<QString, QVariant> var;
    QHash<QString, QVariant> hash;
    QStringList fieldNames;

    fieldNames << "l_note_id" << "l_dna" << "l_sort_order" << "l_indent" <<
                  "l_version" << "t_title" << "dt_created" << "dt_updated" << "dt_content" <<
                  "b_trashed";
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getMultipleValues(noteId, fieldNames, var);
    IFOK(result) {
        hash = var;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

// --------------------------------------------------------------------------------

QList<QString>PLMNoteHub::getAttributes(int projectId, int paperId)
{
    QString attributes = plmdata->notePropertyHub()->getProperty(projectId,
                                                                 paperId,
                                                                 "attributes",
                                                                 "");

    return attributes.split(";", Qt::SkipEmptyParts);
}

// --------------------------------------------------------------------------------


bool PLMNoteHub::hasAttribute(int projectId, int paperId, const QString& attribute)
{
    QString attributes = plmdata->notePropertyHub()->getProperty(projectId,
                                                                 paperId,
                                                                 "attributes",
                                                                 "");

    return attributes.split(";", Qt::SkipEmptyParts).contains(attribute);
}

// --------------------------------------------------------------------------------

SKRResult PLMNoteHub::addAttribute(int projectId, int paperId, const QString& attribute)
{
    QString attributes = plmdata->notePropertyHub()->getProperty(projectId,
                                                                 paperId,
                                                                 "attributes",
                                                                 "");
    QStringList attributeList = attributes.split(";", Qt::SkipEmptyParts);

    if (attributeList.contains(attribute)) {
        return SKRResult();
    }

    attributeList << attribute;
    attributeList.sort();

    return plmdata->notePropertyHub()->setProperty(projectId,
                                                   paperId,
                                                   "attributes",
                                                   attributeList.join(";"));
}

// --------------------------------------------------------------------------------

SKRResult PLMNoteHub::removeAttribute(int projectId, int paperId, const QString& attribute)
{
    QString attributes = plmdata->notePropertyHub()->getProperty(projectId,
                                                                 paperId,
                                                                 "attributes",
                                                                 "");
    QStringList attributeList = attributes.split(";", Qt::SkipEmptyParts);

    if (!attributeList.contains(attribute)) {
        return SKRResult();
    }

    attributeList.removeAll(attribute);

    return plmdata->notePropertyHub()->setProperty(projectId,
                                                   paperId,
                                                   "attributes",
                                                   attributeList.join(";"));
}

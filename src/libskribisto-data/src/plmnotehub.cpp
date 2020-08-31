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

PLMNoteHub::PLMNoteHub(QObject *parent) : PLMPaperHub(parent, "tbl_note")
{

}


//--------------------------------------------------------------------------------

QList<int> PLMNoteHub::getNotesFromSheetId(int projectId, int sheetId) const
{
    PLMError error;
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

//--------------------------------------------------------------------------------

QList<int> PLMNoteHub::getSheetsFromNoteId(int projectId, int noteId) const
{
    PLMError error;
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

//--------------------------------------------------------------------------------

int PLMNoteHub::getSynopsisNoteId(int projectId, int sheetId) const
{

    PLMError error;
    int final = -2;
    QHash<int, int> result;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;
    where.insert("b_synopsis", true);
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

        if(result.isEmpty() || final == 0){
            final = -2;
        }
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return final;

}


//--------------------------------------------------------------------------------

PLMError PLMNoteHub::setSheetNoteRelationship(int projectId, int sheetId, int noteId, bool isSynopsys)
{

    PLMError error;

    QHash<int, int> result;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;
    where.insert("l_sheet_code", sheetId);

    PLMSqlQueries queries(projectId, "tbl_sheet_note");


    // verify if the relationship doesn't yet exist
    error = queries.getValueByIdsWhere("l_note_code", out, where);

    int key = -2;
    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator i = result.constBegin();
        while (i != result.constEnd()) {
            key = i.key();
            ++i;
        }

        if(result.isEmpty() || key == -2 || key == 0){
            // no relationship exists, creating one

            int newId = -2;
            QHash<QString, QVariant> values;
            values.insert("l_sheet_code", sheetId);
            values.insert("l_note_code", noteId);
            values.insert("b_synopsis", isSynopsys);
            error = queries.add(values, newId);

            IFOK(error){
            emit sheetNoteRelationshipAdded(projectId, sheetId, noteId);
            emit sheetNoteRelationshipChanged(projectId, sheetId, noteId);
            }
        }
        else {
            //relationship exists, verifying b_synopsys

            QVariant variantResult;
            error = queries.get(key, "b_synopsis", variantResult);

            if(variantResult.toBool() == isSynopsys){
                //nothing to do, quiting
                return error;
            }
            else {
                // set b_synopsis:

                    error =  queries.set(key, "b_synopsis", isSynopsys);

                    IFOK(error){
                emit sheetNoteRelationshipChanged(projectId, sheetId, noteId);
                }
            }


        }

    }
    return error;
}


//--------------------------------------------------------------------------------

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

        if(result.isEmpty() || key == -2 || key == 0){
            return error;
        }



        }

    IFOK(error) {
    error = queries.remove(key);
    }
    IFOK(error) {
        emit sheetNoteRelationshipRemoved(projectId, sheetId, noteId);
    }

    return error;
}

//--------------------------------------------------------------------------------

PLMError PLMNoteHub::createSynopsys(int projectId, int sheetId){
    PLMError error;
    error = this->addChildPaper(projectId, -1); // add child to project item


    IFOK(error){
        int lastAddedNoteId = this->getLastAddedId();

        error = this->setSheetNoteRelationship(projectId, sheetId, lastAddedNoteId, true);
    }

    return error;

}


//--------------------------------------------------------------------------------

// useful ?
QHash<QString, QVariant> PLMNoteHub::getNoteData(int projectId, int noteId) const
{
    PLMError error;

    QHash<QString, QVariant> var;
    QHash<QString, QVariant> result;
    QStringList fieldNames;
    fieldNames << "l_note_id" << "l_dna" << "l_sort_order" << "l_indent" <<
                  "l_version" << "t_title" << "dt_created" << "dt_updated" << "dt_content" <<
                  "b_deleted";
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

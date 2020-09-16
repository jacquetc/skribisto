/***************************************************************************
 *   Copyright (C) 2020 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skrtaghub.cpp                                                   *
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
#include "skrtaghub.h"
#include "tasks/plmsqlqueries.h"
#include "tools.h"
#include <QDateTime>

SKRTagHub::SKRTagHub(QObject *parent) : QObject(parent), m_last_added_id(-1)
{


}

//--------------------------------------------------------------------------------


QList<int> SKRTagHub::getAllTagIds(int projectId) const
{
    PLMError error;

    QList<int> result;
    QList<int> out;
    PLMSqlQueries queries(projectId, "tbl_tag");
    error = queries.getIds(out);
    IFOK(error) {
        result = out;
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

//--------------------------------------------------------------------------------


PLMError SKRTagHub::addTag(int projectId, const QString &tagName)
{
    PLMError error;

    int newId = -2;
    QHash<QString, QVariant> values;
    values.insert("t_name", tagName);

    PLMSqlQueries queries(projectId, "tbl_tag");
    error = queries.add(values, newId);


    IFOK(error){
        m_last_added_id = newId;
        emit tagAdded(projectId, newId);
        emit projectModified(projectId);

    }

    return error;
}

//--------------------------------------------------------------------------------


PLMError SKRTagHub::removeTag(int projectId, int tagId)
{
    PLMError error;

    int newId = -2;


    PLMSqlQueries queries(projectId, "tbl_tag");
    error = queries.remove(tagId);


    IFOK(error){
        emit tagAdded(projectId, newId);
        emit projectModified(projectId);

    }

    return error;
}

int SKRTagHub::getTagIdWithName(int projectId, const QString &tagName)
{
    int result = -2;
    PLMError error;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, "tbl_tag");

    error = queries.getValueByIds("l_tag_id", out, "t_name", tagName);
    IFOK(error) {
            QHash<int, QVariant>::const_iterator i = out.constBegin();
            while (i != out.constEnd()) {
                if(!i.value().isNull()){
                    result = i.key();
                }
                ++i;
            }
    }
    IFKO(error) {
        emit errorSent(error);
    }


    return result;
}

//--------------------------------------------------------------------------------


QString SKRTagHub::getTagName(int projectId, int tagId) const
{
    return get(projectId, tagId, "t_name").toString();

}

//--------------------------------------------------------------------------------


PLMError SKRTagHub::setTagName(int projectId, int tagId, const QString &tagName)
{
    PLMError error;

    if(tagName.isEmpty()){
        error.setSuccess(false);
        error.setErrorCode("E_TAG_name_is_missing");
    }

    IFOKDO(error, set(projectId, tagId, "t_name", tagName));

    IFOK(error) {
        emit nameChanged(projectId, tagId, tagName);
        emit projectModified(projectId);
    }
    return error;
}

//--------------------------------------------------------------------------------


bool SKRTagHub::doesTagNameAlreadyExist(int projectId, const QString &tagName)
{
    bool result = true;
    PLMError error;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, "tbl_tag");

    error = queries.getValueByIds("l_tag_id", out, "t_name", tagName);
    IFOK(error) {
        if(out.isEmpty()){
            result = false;
        }
    }
    IFKO(error) {
        emit errorSent(error);
    }


    return result;
}

//--------------------------------------------------------------------------------

QString SKRTagHub::getTagColor(int projectId, int tagId) const
{
    return get(projectId, tagId, "t_color").toString();

}

//--------------------------------------------------------------------------------

PLMError SKRTagHub::setTagColor(int projectId, int tagId, const QString &color)
{
    PLMError error = set(projectId, tagId, "t_color", color);

    IFOK(error) {
        emit colorChanged(projectId, tagId, color);
        emit projectModified(projectId);
    }
    return error;
}

// ------------------------------------------------------------

PLMError SKRTagHub::setCreationDate(int projectId, int tagId,
                                      const QDateTime& newDate)
{
    PLMError error = set(projectId, tagId, "dt_created", newDate);

    IFOK(error) {
        emit creationDateChanged(projectId, tagId, newDate);
        emit projectModified(projectId);
    }
    return error;
}

// ------------------------------------------------------------

QDateTime SKRTagHub::getCreationDate(int projectId, int tagId) const
{
    return get(projectId, tagId, "dt_created").toDateTime();
}
//--------------------------------------------------------------------------------

PLMError SKRTagHub::setUpdateDate(int projectId, int tagId, const QDateTime &newDate)
{
    PLMError error = set(projectId, tagId, "dt_updated", newDate);

    IFOK(error) {
        emit updateDateChanged(projectId, tagId, newDate);
        emit projectModified(projectId);
    }
    return error;
}

//--------------------------------------------------------------------------------

QDateTime SKRTagHub::getUpdateDate(int projectId, int tagId) const
{
    return get(projectId, tagId, "dt_updated").toDateTime();

}

//--------------------------------------------------------------------------------


PLMError SKRTagHub::set(int projectId, int tagId, const QString &fieldName, const QVariant &value, bool setCurrentDateBool)
{
    PLMError error;
    PLMSqlQueries queries(projectId,  "tbl_tag");

    queries.beginTransaction();
    error = queries.set(tagId, fieldName, value);

    if (setCurrentDateBool) {
        IFOKDO(error, queries.setCurrentDate(tagId, "dt_updated"));
    }

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return error;
}

//--------------------------------------------------------------------------------


QVariant SKRTagHub::get(int projectId, int tagId, const QString &fieldName) const
{
    PLMError error;
    QVariant var;
    QVariant result;
    PLMSqlQueries queries(projectId, "tbl_tag");

    error = queries.get(tagId, fieldName, var);
    IFOK(error) {
        result = var;
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

//--------------------------------------------------------------------------------

int SKRTagHub::getLastAddedId()
{
    return m_last_added_id;

}

//--------------------------------------------------------------------------------

int SKRTagHub::getTopPaperId(int projectId) const
{
    int result = -2;
    QList<int> list = this->getAllTagIds(projectId);
    if(!list.isEmpty()){
        result = list.first();
    }

    return result;
}

//--------------------------------------------------------------------------------
//--------Relationship---------------------------------------------------------
//--------------------------------------------------------------------------------


QList<int> SKRTagHub::getItemIdsFromTag(int projectId, int tagId, bool addSeparator) const
{
    PLMError error;

    QList<int> result;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;


    PLMSqlQueries queries(projectId, "tbl_sheet_note");



    // get l_sheet_code
    if(addSeparator){
        result.append(-30);
    }

    where.insert("l_tag_code", tagId);
    error = queries.getValueByIdsWhere("l_sheet_code", out, where);

    IFOK(error) {
        //filter null results
        QHash<int, QVariant>::const_iterator i = out.constBegin();
        while (i != out.constEnd()) {
            if(!i.value().isNull()){
                result.append(i.key());
            }
            ++i;
        }

    }


    // get l_note_code
    IFOK(error) {
        if(addSeparator){
            result.append(-31);
        }
        error = queries.getValueByIdsWhere("l_note_code", out, where);

        IFOK(error) {
            //filter null results
            QHash<int, QVariant>::const_iterator i = out.constBegin();
            while (i != out.constEnd()) {
                if(!i.value().isNull()){
                    result.append(i.key());
                }
                ++i;
            }
        }



    }

    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

//--------------------------------------------------------------------------------


QList<int> SKRTagHub::getTagsFromItemId(int projectId, SKRTagHub::ItemType itemType, int itemId) const
{
    PLMError error;

    QList<int> result;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    if(itemType == ItemType::Sheet){
        where.insert("l_sheet_code", itemId);
    }
    if(itemType == ItemType::Note){
        where.insert("l_note_code", itemId);
    }


    PLMSqlQueries queries(projectId, "tbl_tag_relationship");

    error = queries.getValueByIdsWhere("l_tag_relationship_id", out, where);

    IFOK(error) {

        //filter null results
        QHash<int, QVariant>::const_iterator i = out.constBegin();
        while (i != out.constEnd()) {
            if(!i.value().isNull()){
                result.append(i.key());
            }
            ++i;
        }
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;

}

//--------------------------------------------------------------------------------


PLMError SKRTagHub::setTagRelationship(int projectId, SKRTagHub::ItemType itemType, int itemId, int tagId)
{

    PLMError error;

    QHash<int, int> result;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    if(itemType == ItemType::Sheet){
        where.insert("l_sheet_code", itemId);
    }
    if(itemType == ItemType::Note){
        where.insert("l_note_code", itemId);
    }
    where.insert("l_tag_code", tagId);

    PLMSqlQueries queries(projectId, "tbl_tag_relationship");


    // verify if the relationship doesn't yet exist
    error = queries.getValueByIdsWhere("l_tag_relationship_id", out, where);

    int key = -2;
    IFOK(error) {
        //filter null results
        QHash<int, QVariant>::iterator i = out.begin();
        while (i != out.end()) {
            if(i.value().isNull()){
                i = out.erase(i);
            }
            else{
                ++i;
            }
        }


        result = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator j = result.constBegin();
        while (j != result.constEnd()) {
            key = j.key();
            ++j;
        }

        if(result.isEmpty() || key == -2 || key == 0){
            // no relationship exists, creating one

            int newId = -2;
            QHash<QString, QVariant> values;


            if(itemType == ItemType::Sheet){
                values.insert("l_sheet_code", itemId);
            }
            if(itemType == ItemType::Note){
                values.insert("l_note_code", itemId);
            }
            values.insert("l_tag_code", tagId);

            error = queries.add(values, newId);

            IFOK(error){
                emit tagRelationshipAdded(projectId, itemType, itemId, tagId);
                emit tagRelationshipChanged(projectId, itemType, itemId, tagId);
                emit projectModified(projectId);
            }
        }
        //relationship exists, do nothing


    }
    return error;
}

//--------------------------------------------------------------------------------

PLMError SKRTagHub::removeTagRelationship(int projectId, SKRTagHub::ItemType itemType, int itemId, int tagId)
{
    PLMError error;

    QHash<int, int> result;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;
    if(itemType == ItemType::Sheet){
        where.insert("l_sheet_code", itemId);
    }
    if(itemType == ItemType::Note){
        where.insert("l_note_code", itemId);
    }
    PLMSqlQueries queries(projectId,  "tbl_tag_relationship");

    error = queries.getValueByIdsWhere("l_tag_relationship_id", out, where);

    int key = -2;
    IFOK(error) {

        //filter null results
        QHash<int, QVariant>::iterator i = out.begin();
        while (i != out.end()) {
            if(i.value().isNull()){
                i = out.erase(i);
            }
            else{
                ++i;
            }
        }


        result = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator j = result.constBegin();
        while (j != result.constEnd()) {
            key = j.key();
            ++j;
        }

        if(result.isEmpty() || key == -2 || key == 0){
            error.setSuccess(false);
            error.setErrorCode("E_TAG_no_tag_relationship_to_remove");
            return error;
        }



    }

    IFOK(error) {
        error = queries.remove(key);
    }
    IFOK(error) {
        emit tagRelationshipRemoved(projectId, itemType, itemId, tagId);
        emit projectModified(projectId);
    }

    return error;
}

//--------------------------------------------------------------------------------


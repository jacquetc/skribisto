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
{}

// --------------------------------------------------------------------------------


QList<int>SKRTagHub::getAllTagIds(int projectId) const
{
    SKRResult result(this);

    QList<int> list;
    QList<int> out;
    PLMSqlQueries queries(projectId, "tbl_tag");

    result = queries.getIds(out);
    IFOK(result) {
        list = out;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// --------------------------------------------------------------------------------


SKRResult SKRTagHub::addTag(int projectId, const QString& tagName)
{
    SKRResult result(this);

    int newId = -2;
    QHash<QString, QVariant> values;

    values.insert("t_name", tagName);

    PLMSqlQueries queries(projectId, "tbl_tag");

    result = queries.add(values, newId);
    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }

    IFOK(result) {
        m_last_added_id = newId;
        result.addData("tagId", newId);
        emit tagAdded(projectId, newId);
        emit projectModified(projectId);
    }

    return result;
}

// --------------------------------------------------------------------------------


SKRResult SKRTagHub::removeTag(int projectId, int tagId)
{
    SKRResult result(this);

    int newId = -2;


    PLMSqlQueries queries(projectId, "tbl_tag");

    result = queries.remove(tagId);
    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }

    IFOK(result) {
        emit tagRemoved(projectId, newId);
        emit projectModified(projectId);
    }

    return result;
}

int SKRTagHub::getTagIdWithName(int projectId, const QString& tagName)
{
    int value = -2;
    SKRResult result(this);
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, "tbl_tag");

    result = queries.getValueByIds("l_tag_id", out, "t_name", tagName);
    IFOK(result) {
        QHash<int, QVariant>::const_iterator i = out.constBegin();

        while (i != out.constEnd()) {
            if (!i.value().isNull()) {
                value = i.key();
            }
            ++i;
        }
    }
    IFKO(result) {
        emit errorSent(result);
    }


    return value;
}

// --------------------------------------------------------------------------------


QString SKRTagHub::getTagName(int projectId, int tagId) const
{
    return get(projectId, tagId, "t_name").toString();
}

// --------------------------------------------------------------------------------


SKRResult SKRTagHub::setTagName(int projectId, int tagId, const QString& tagName)
{
    SKRResult result(this);

    if (tagName.isEmpty()) {
        result = SKRResult(SKRResult::Critical, this, "name_is_missing");
    }

    IFOKDO(result, set(projectId, tagId, "t_name", tagName));

    IFOK(result) {
        emit nameChanged(projectId, tagId, tagName);
        emit projectModified(projectId);
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return result;
}

// --------------------------------------------------------------------------------


bool SKRTagHub::doesTagNameAlreadyExist(int projectId, const QString& tagName)
{
    bool value = true;
    SKRResult result(this);
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, "tbl_tag");

    result = queries.getValueByIds("l_tag_id", out, "t_name", tagName);
    IFOK(result) {
        if (out.isEmpty()) {
            value = false;
        }
    }
    IFKO(result) {
        emit errorSent(result);
    }


    return value;
}

// --------------------------------------------------------------------------------

QString SKRTagHub::getTagColor(int projectId, int tagId) const
{
    return get(projectId, tagId, "t_color").toString();
}

// --------------------------------------------------------------------------------

SKRResult SKRTagHub::setTagColor(int projectId, int tagId, const QString& color)
{
    SKRResult result = set(projectId, tagId, "t_color", color);

    IFOK(result) {
        emit colorChanged(projectId, tagId, color);
        emit projectModified(projectId);
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return result;
}

// --------------------------------------------------------------------------------

QString SKRTagHub::getTagTextColor(int projectId, int tagId) const
{
    return get(projectId, tagId, "t_text_color").toString();
}

// --------------------------------------------------------------------------------

SKRResult SKRTagHub::setTagTextColor(int projectId, int tagId, const QString& color)
{
    SKRResult result = set(projectId, tagId, "t_text_color", color);

    IFOK(result) {
        emit textColorChanged(projectId, tagId, color);
        emit projectModified(projectId);
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return result;
}

// ------------------------------------------------------------

SKRResult SKRTagHub::setCreationDate(int projectId, int tagId,
                                     const QDateTime& newDate)
{
    SKRResult result = set(projectId, tagId, "dt_created", newDate);

    IFOK(result) {
        emit creationDateChanged(projectId, tagId, newDate);
        emit projectModified(projectId);
    }


    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ------------------------------------------------------------

QDateTime SKRTagHub::getCreationDate(int projectId, int tagId) const
{
    return get(projectId, tagId, "dt_created").toDateTime();
}

// --------------------------------------------------------------------------------

SKRResult SKRTagHub::setUpdateDate(int projectId, int tagId, const QDateTime& newDate)
{
    SKRResult result = set(projectId, tagId, "dt_updated", newDate);

    IFOK(result) {
        emit updateDateChanged(projectId, tagId, newDate);
        emit projectModified(projectId);
    }


    IFKO(result) {
        emit errorSent(result);
    }

    return result;
}

// --------------------------------------------------------------------------------

QDateTime SKRTagHub::getUpdateDate(int projectId, int tagId) const
{
    return get(projectId, tagId, "dt_updated").toDateTime();
}

// --------------------------------------------------------------------------------


SKRResult SKRTagHub::set(int             projectId,
                         int             tagId,
                         const QString & fieldName,
                         const QVariant& value,
                         bool            setCurrentDateBool)
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId,  "tbl_tag");

    queries.beginTransaction();
    result = queries.set(tagId, fieldName, value);

    if (setCurrentDateBool) {
        IFOKDO(result, queries.setCurrentDate(tagId, "dt_updated"));
    }

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// --------------------------------------------------------------------------------


QVariant SKRTagHub::get(int projectId, int tagId, const QString& fieldName) const
{
    SKRResult result(this);
    QVariant var;
    QVariant value;
    PLMSqlQueries queries(projectId, "tbl_tag");

    result = queries.get(tagId, fieldName, var);
    IFOK(result) {
        value = var;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return value;
}

// --------------------------------------------------------------------------------

int SKRTagHub::getLastAddedId()
{
    return m_last_added_id;
}

// --------------------------------------------------------------------------------

int SKRTagHub::getTopPaperId(int projectId) const
{
    int value      = -2;
    QList<int> list = this->getAllTagIds(projectId);

    if (!list.isEmpty()) {
        value = list.first();
    }

    return value;
}

// --------------------------------------------------------------------------------
// --------Relationship---------------------------------------------------------
// --------------------------------------------------------------------------------


QList<int>SKRTagHub::getItemIdsFromTag(int                 projectId,
                                       SKR::ItemType itemType,
                                       int                 tagId) const
{
    SKRResult result(this);

    QList<int> list;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;


    PLMSqlQueries queries(projectId, "tbl_tag_relationship");

    where.insert("l_tag_code", tagId);


    // get l_sheet_code
    if (itemType == SKR::Sheet) {
        result = queries.getValueByIdsWhere("l_sheet_code", out, where);

        IFOK(result) {
            // filter null results
            QHash<int, QVariant>::const_iterator i = out.constBegin();

            while (i != out.constEnd()) {
                if (!i.value().isNull()) {
                    list.append(i.key());
                }
                ++i;
            }
        }
    }


    // get l_note_code
    if (itemType == SKR::Note) {
        result = queries.getValueByIdsWhere("l_note_code", out, where);

        IFOK(result) {
            // filter null results
            QHash<int, QVariant>::const_iterator i = out.constBegin();

            while (i != out.constEnd()) {
                if (!i.value().isNull()) {
                    list.append(i.key());
                }
                ++i;
            }
        }
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

QList<int>SKRTagHub::getItemIdsFromTag(int projectId, int tagId, bool addSeparator) const
{
    SKRResult result(this);

    QList<int> list;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;


    PLMSqlQueries queries(projectId, "tbl_tag_relationship");

    where.insert("l_tag_code", tagId);

    // get l_sheet_code
    if (addSeparator) {
        list.append(-30);
    }

    result = queries.getValueByIdsWhere("l_sheet_code", out, where);

    IFOK(result) {
        // filter null results
        QHash<int, QVariant>::const_iterator i = out.constBegin();

        while (i != out.constEnd()) {
            if (!i.value().isNull()) {
                list.append(i.key());
            }
            ++i;
        }
    }


    // get l_note_code
    IFOK(result) {
        if (addSeparator) {
            list.append(-31);
        }
        result = queries.getValueByIdsWhere("l_note_code", out, where);

        IFOK(result) {
            // filter null lists
            QHash<int, QVariant>::const_iterator i = out.constBegin();

            while (i != out.constEnd()) {
                if (!i.value().isNull()) {
                    list.append(i.key());
                }
                ++i;
            }
        }
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// --------------------------------------------------------------------------------


QList<int>SKRTagHub::getTagsFromItemId(int                 projectId,
                                       SKR::ItemType itemType,
                                       int                 itemId) const
{
    SKRResult result(this);

    QList<int> list;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    if (itemType == SKR::Sheet) {
        where.insert("l_sheet_code", itemId);
    }

    if (itemType == SKR::Note) {
        where.insert("l_note_code", itemId);
    }


    PLMSqlQueries queries(projectId, "tbl_tag_relationship");

    result = queries.getValueByIdsWhere("l_tag_code", out, where);

    IFOK(result) {
        // filter null results
        QHash<int, QVariant>::const_iterator i = out.constBegin();

        while (i != out.constEnd()) {
            if (!i.value().isNull()) {
                list.append(i.value().toInt());
            }
            ++i;
        }
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// --------------------------------------------------------------------------------


SKRResult SKRTagHub::setTagRelationship(int                 projectId,
                                        SKR::ItemType itemType,
                                        int                 itemId,
                                        int                 tagId)
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    if (itemType == SKR::Sheet) {
        where.insert("l_sheet_code", itemId);
    }

    if (itemType == SKR::Note) {
        where.insert("l_note_code", itemId);
    }
    where.insert("l_tag_code", tagId);

    PLMSqlQueries queries(projectId, "tbl_tag_relationship");


    // verify if the relationship doesn't yet exist
    result = queries.getValueByIdsWhere("l_tag_relationship_id", out, where);

    int key = -2;

    IFOK(result) {
        // filter null results
        QHash<int, QVariant>::iterator i = out.begin();

        while (i != out.end()) {
            if (i.value().isNull()) {
                i = out.erase(i);
            }
            else {
                ++i;
            }
        }


        hash = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator j = hash.constBegin();

        while (j != hash.constEnd()) {
            key = j.key();
            ++j;
        }

        if (hash.isEmpty() || (key == -2) || (key == 0)) {
            // no relationship exists, creating one

            int newId = -2;
            QHash<QString, QVariant> values;


            if (itemType == SKR::Sheet) {
                values.insert("l_sheet_code", itemId);
            }

            if (itemType == SKR::Note) {
                values.insert("l_note_code", itemId);
            }
            values.insert("l_tag_code", tagId);

            result = queries.add(values, newId);
            IFKO(result) {
                queries.rollback();
            }
            IFOK(result) {
                queries.commit();
            }
            IFKO(result) {
                emit errorSent(result);
            }

            IFOK(result) {
                emit tagRelationshipAdded(projectId, itemType, itemId, tagId);
                emit tagRelationshipChanged(projectId, itemType, itemId, tagId);
                emit projectModified(projectId);
            }
        }

        // relationship exists, do nothing
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return result;
}

// --------------------------------------------------------------------------------

SKRResult SKRTagHub::removeTagRelationship(int                 projectId,
                                           SKR::ItemType itemType,
                                           int                 itemId,
                                           int                 tagId)
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;
    where.insert("l_tag_code", tagId);

    if (itemType == SKR::Sheet) {
        where.insert("l_sheet_code", itemId);
    }

    if (itemType == SKR::Note) {
        where.insert("l_note_code", itemId);
    }
    PLMSqlQueries queries(projectId,  "tbl_tag_relationship");

    result = queries.getValueByIdsWhere("l_tag_relationship_id", out, where);

    int key = -2;

    IFOK(result) {
        // filter null hashs
        QHash<int, QVariant>::iterator i = out.begin();

        while (i != out.end()) {
            if (i.value().isNull()) {
                i = out.erase(i);
            }
            else {
                ++i;
            }
        }


        hash = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator j = hash.constBegin();

        while (j != hash.constEnd()) {
            key = j.key();
            ++j;
        }

        if (hash.isEmpty() || (key == -2) || (key == 0)) {
            result = SKRResult(SKRResult::Critical, this, "no_tag_relationship_to_remove");
            return result;
        }
    }

    IFOK(result) {
        result = queries.remove(key);
    }
    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }

    IFOK(result) {
        emit tagRelationshipRemoved(projectId, itemType, itemId, tagId);
        emit projectModified(projectId);
    }

    return result;
}

// --------------------------------------------------------------------------------

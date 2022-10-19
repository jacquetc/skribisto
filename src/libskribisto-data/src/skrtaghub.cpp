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
#include "sql/plmsqlqueries.h"
#include "tools.h"
#include <QDateTime>

SKRTagHub::SKRTagHub(QObject *parent) : QObject(parent), m_last_added_id(-1)
{}

// --------------------------------------------------------------------------------


QVariantMap SKRTagHub::saveId(int projectId, int tagId) const
{
    SKRResult result(this);

    PLMSqlQueries queries(projectId, "tbl_tag");
    QStringList fieldNames = queries.getAllFieldTitles();

    QVariantMap allFields;

    for(const QString &fieldName : fieldNames) {
        allFields.insert(fieldName, this->get(projectId, tagId, fieldName));
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return allFields;
}

// --------------------------------------------------------------------------------

SKRResult SKRTagHub::restoreId(int projectId, int tagId, const QVariantMap &values)
{
    SKRResult result(this);

    QVariantMap::const_iterator i = values.constBegin();
    while (i != values.constEnd()) {
        result = set(projectId, tagId, i.key(), i.value(), false, false);
        ++i;
    }
    this->commit(projectId);


    IFOK(result) {
        // do like if a tag was added :
        emit tagAdded(projectId, tagId);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

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


SKRResult SKRTagHub::addTag(int projectId)
{
    SKRResult result(this);

    int newId = -2;
    QHash<QString, QVariant> values;

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



SKRResult SKRTagHub::setTagId(int projectId, int tagId, int newTagId)
{
    SKRResult result(this);

    IFOKDO(result, set(projectId, tagId, "l_tag_id", newTagId));

    IFOK(result) {
        emit tagIdChanged(projectId, tagId, newTagId);
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

// --------------------------------------------------------------------------------

SKRResult SKRTagHub::setTagRandomColors(int projectId, int tagId)
{
    QPair<QString, QString> pair = getTagRandomColors();


    SKRResult result = setTagColor(projectId, tagId, pair.first);

    result = setTagTextColor(projectId, tagId, pair.second);

    return result;
}

// ------------------------------------------------------------

QPair<QString, QString> SKRTagHub::getTagRandomColors()
{
    int randColorIndex = QRandomGenerator::global()->bounded(27);

    QMap<QString, QString> colorMap = getTagPresetColors();

    QPair<QString, QString> pair;
    pair.first = colorMap.keys().at(randColorIndex);
    pair.second = colorMap.values().at(randColorIndex);

    return pair;
}

// ------------------------------------------------------------

QMap<QString, QString> SKRTagHub::getTagPresetColors()
{
    QMap<QString, QString> colorMap;

    colorMap.insert("#FFFFFF", "#000000");
    colorMap.insert("#FFFAFA", "#000000");
    colorMap.insert("#F0FFFF", "#000000");
    colorMap.insert("#F5F5DC", "#000000");
    colorMap.insert("#000000", "#FFFFFF");
    colorMap.insert("#FF0000", "#000000");
    colorMap.insert("#8B0000", "#FFFFFF");
    colorMap.insert("#98FB98", "#000000");
    colorMap.insert("#7FFF00", "#000000");
    colorMap.insert("#008000", "#FFFFFF");
    colorMap.insert("#006400", "#FFFFFF");
    colorMap.insert("#E0FFFF", "#000000");
    colorMap.insert("#00FFFF", "#000000");
    colorMap.insert("#008B8B", "#FFFFFF");
    colorMap.insert("#0000FF", "#FFFFFF");
    colorMap.insert("#00008B", "#FFFFFF");
    colorMap.insert("#FFB6C1", "#000000");
    colorMap.insert("#FFC0CB", "#000000");
    colorMap.insert("#FF69B4", "#000000");
    colorMap.insert("#FF00FF", "#000000");
    colorMap.insert("#8B008B", "#FFFFFF");
    colorMap.insert("#FFFFE0", "#000000");
    colorMap.insert("#FFFF00", "#000000");
    colorMap.insert("#FFD700", "#000000");
    colorMap.insert("#FFA500", "#000000");
    colorMap.insert("#FF8C00", "#FFFFFF");
    colorMap.insert("#808080", "#FFFFFF");
    colorMap.insert("#A9A9A9", "#FFFFFF");

    return colorMap;
}

// ------------------------------------------------------------

void SKRTagHub::commit(int projectId)
{
    PLMSqlQueries queries(projectId, "tbl_tag");
    queries.commit();

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
                         bool            setCurrentDateBool, bool commit)
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
        if (commit) {
            queries.commit();
        }
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
    QVariant  var;
    QVariant  value;
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
    int value       = -2;
    QList<int> list = this->getAllTagIds(projectId);

    if (!list.isEmpty()) {
        value = list.first();
    }

    return value;
}

// --------------------------------------------------------------------------------
// --------Relationship---------------------------------------------------------
// --------------------------------------------------------------------------------


QList<int>SKRTagHub::getItemIdsFromTag(int projectId,
                                       int tagId) const
{
    SKRResult result(this);

    QList<int> list;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;


    PLMSqlQueries queries(projectId, "tbl_tag_relationship");

    where.insert("l_tag_code", tagId);


    result = queries.getValueByIdsWhere("l_tree_code", out, where);

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


    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// --------------------------------------------------------------------------------


QList<int>SKRTagHub::getTagsFromItemId(int projectId,
                                       int treeItemId) const
{
    SKRResult result(this);

    QList<int> list;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_tree_code", treeItemId);


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


SKRResult SKRTagHub::setTagRelationship(int projectId,
                                        int treeItemId,
                                        int tagId)
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_tree_code", treeItemId);
    where.insert("l_tag_code",  tagId);

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

            values.insert("l_tree_code", treeItemId);
            values.insert("l_tag_code",  tagId);

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
                emit tagRelationshipAdded(projectId, treeItemId, tagId);
                emit tagRelationshipChanged(projectId, treeItemId, tagId);
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

SKRResult SKRTagHub::removeTagRelationship(int projectId,
                                           int treeItemId,
                                           int tagId)
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_tag_code",  tagId);
    where.insert("l_tree_code", treeItemId);

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
        emit tagRelationshipRemoved(projectId, treeItemId, tagId);
        emit projectModified(projectId);
    }

    return result;
}

// --------------------------------------------------------------------------------

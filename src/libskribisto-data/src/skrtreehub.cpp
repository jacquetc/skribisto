/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrtreehub.cpp                                                   *
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
#include "skrtreehub.h"
#include "tasks/plmsqlqueries.h"
#include "tools.h"
#include "tasks/plmprojectmanager.h"

#include <QCollator>

SKRTreeHub::SKRTreeHub(QObject *parent) : QObject(parent), m_tableName("tbl_tree"), m_last_added_id(-1)
{
    connect(this, &SKRTreeHub::errorSent, this, &SKRTreeHub::setError, Qt::DirectConnection);
}

// ----------------------------------------------------------------------------------------

QHash<int, int>SKRTreeHub::getAllSortOrders(int projectId) const
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("l_sort_order", out, "", QVariant(), true);
    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntInt(out);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

// ----------------------------------------------------------------------------------------

QHash<int, int>SKRTreeHub::getAllIndents(int projectId) const
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("l_indent", out, "", QVariant(), true);
    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntInt(out);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

// ----------------------------------------------------------------------------------------

///
/// \brief SKRTreeHub::getAllIds
/// \param projectId
/// \return
/// Get sorted ids, trashed ids included
QList<int>SKRTreeHub::getAllIds(int projectId) const
{
    SKRResult result(this);

    QList<int> list;
    QList<int> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getSortedIds(out);
    IFOK(result) {
        list = out;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setTitle(int projectId, int treeItemId, const QString& newTitle)
{
    SKRResult result = set(projectId, treeItemId, "t_title", newTitle);

    IFOK(result) {
        emit titleChanged(projectId, treeItemId, newTitle);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QString SKRTreeHub::getTitle(int projectId, int treeItemId) const
{
    return get(projectId, treeItemId, "t_title").toString();
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setInternalTitle(int projectId, int treeItemId, const QString& newTitle)
{
    SKRResult result = set(projectId, treeItemId, "t_internal_title", newTitle);

    IFOK(result) {
        emit internalTitleChanged(projectId, treeItemId, newTitle);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QString SKRTreeHub::getInternalTitle(int projectId, int treeItemId) const
{
    return get(projectId, treeItemId, "t_internal_title").toString();
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setIndent(int projectId, int treeItemId, int newIndent)
{
    SKRResult result = set(projectId, treeItemId, "l_indent", newIndent);

    IFOK(result) {
        emit indentChanged(projectId, treeItemId, newIndent);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

int SKRTreeHub::getIndent(int projectId, int treeItemId) const
{
    int indent;

    if (treeItemId == -1) // is project item
        indent = -1;
    else {
        indent = get(projectId, treeItemId, "l_indent").toInt();
    }

    return indent;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setSortOrder(int projectId, int treeItemId, int newSortOrder)
{
    return this->setSortOrder(projectId, treeItemId, newSortOrder, true, true);
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setSortOrder(int projectId, int treeItemId, int newSortOrder, bool setCurrentdate, bool commit)
{
    SKRResult result = set(projectId, treeItemId, "l_sort_order", newSortOrder, setCurrentdate, commit);

    IFOK(result) {
        emit sortOrderChanged(projectId, treeItemId, newSortOrder);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

int SKRTreeHub::getSortOrder(int projectId, int treeItemId) const
{
    return get(projectId, treeItemId, "l_sort_order").toInt();
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setType(int projectId, int treeItemId, const QString& newType)
{
    SKRResult result = set(projectId, treeItemId, "t_type", newType);

    IFOK(result) {
        emit typeChanged(projectId, treeItemId, newType);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QString SKRTreeHub::getType(int projectId, int treeItemId) const
{
    return get(projectId, treeItemId, "t_type").toString();
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setPrimaryContent(int projectId, int treeItemId, const QString& newContent)
{
    return this->setPrimaryContent(projectId, treeItemId, newContent, true, true);
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setPrimaryContent(int            projectId,
                                        int            treeItemId,
                                        const QString& newContent,
                                        bool           setCurrentdate,
                                        bool           commit)
{
    SKRResult result = set(projectId, treeItemId, "m_primary_content", newContent, setCurrentdate, commit);

    IFOK(result) {
        emit primaryContentChanged(projectId, treeItemId, newContent);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QString SKRTreeHub::getPrimaryContent(int projectId, int treeItemId) const
{
    QString content = get(projectId, treeItemId, "m_primary_content").toString();

    return content;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setSecondaryContent(int projectId, int treeItemId, const QString& newContent)
{
    return this->setSecondaryContent(projectId, treeItemId, newContent, true, true);
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setSecondaryContent(int            projectId,
                                          int            treeItemId,
                                          const QString& newContent,
                                          bool           setCurrentdate,
                                          bool           commit)
{
    SKRResult result = set(projectId, treeItemId, "m_secondary_content", newContent, setCurrentdate, commit);

    IFOK(result) {
        emit secondaryContentChanged(projectId, treeItemId, newContent);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QString SKRTreeHub::getSecondaryContent(int projectId, int treeItemId) const
{
    QString content = get(projectId, treeItemId, "m_secondary_content").toString();

    return content;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setTrashedWithChildren(int projectId, int treeItemId, bool newTrashedState)
{
    SKRResult result(this);

    QList<int> childrenIdList = this->getAllChildren(projectId, treeItemId);


    // children deletion (or recovery)
    for (int& _id : childrenIdList) {
        IFOKDO(result, set(projectId, _id, "b_trashed", newTrashedState));

        // set date but those already deleted
        if (newTrashedState && this->getTrashedDate(projectId, _id).isNull()) {
            result = this->setTrashedDateToNow(projectId, _id);
            emit trashedChanged(projectId, _id, newTrashedState);
        }

        // restore
        else if (!newTrashedState) {
            result = this->setTrashedDateToNull(projectId, _id);
            emit trashedChanged(projectId, _id, newTrashedState);
        }

        // else ignore those already trashed
    }


    // do parent :
    IFOK(result) {
        result = set(projectId, treeItemId, "b_trashed", newTrashedState);

        // set date but those already deleted
        if (newTrashedState && this->getTrashedDate(projectId, treeItemId).isNull()) {
            result = this->setTrashedDateToNow(projectId, treeItemId);
        }

        // restore
        else if (!newTrashedState) {
            result = this->setTrashedDateToNull(projectId, treeItemId);
        }
    }
    IFOK(result) {
        emit trashedChanged(projectId, treeItemId, newTrashedState);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::untrashOnlyOneTreeItem(int projectId, int treeItemId)
{
    SKRResult result = set(projectId, treeItemId, "b_trashed", false);

    IFOKDO(result, this->setTrashedDateToNull(projectId, treeItemId));

    IFOK(result) {
        emit trashedChanged(projectId, treeItemId, false);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

bool SKRTreeHub::getTrashed(int projectId, int treeItemId) const
{
    return get(projectId, treeItemId, "b_trashed").toBool();
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setCreationDate(int projectId, int treeItemId, const QDateTime& newDate)
{
    SKRResult result = set(projectId, treeItemId, "dt_created", newDate);

    IFOK(result) {
        emit creationDateChanged(projectId, treeItemId, newDate);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QDateTime SKRTreeHub::getCreationDate(int projectId, int treeItemId) const
{
    return get(projectId, treeItemId, "dt_created").toDateTime();
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setUpdateDate(int projectId, int treeItemId, const QDateTime& newDate)
{
    SKRResult result = set(projectId, treeItemId, "dt_updated", newDate);

    IFOK(result) {
        emit updateDateChanged(projectId, treeItemId, newDate);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QDateTime SKRTreeHub::getUpdateDate(int projectId, int treeItemId) const
{
    return get(projectId, treeItemId, "dt_updated").toDateTime();
}

// ----------------------------------------------------------------------------------------

bool SKRTreeHub::hasChildren(int projectId, int treeItemId, bool trashedAreIncluded, bool notTrashedAreIncluded) const
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, m_tableName);

    // if last of id list:
    QList<int> idList;

    IFOKDO(result, queries.getSortedIds(idList));

    if (idList.isEmpty()) { // project item with nothing else
        return false;
    }

    if (treeItemId == idList.last()) {
        return false;
    }

    int indent = getIndent(projectId, treeItemId);

    int possibleFirstChildId;

    if (treeItemId == -1) {                  // project item
        possibleFirstChildId = idList.at(0); // first real treeItem in table
    }
    else {
        possibleFirstChildId = idList.at(idList.indexOf(treeItemId) + 1);
    }


    int possibleFirstChildIndent = getIndent(projectId, possibleFirstChildId);

    // verify indent of child
    if (indent == possibleFirstChildIndent - 1) {
        // verify if at least one child is not trashed
        bool haveOneNotTrashedChild = false;
        bool haveOneTrashedChild    = false;
        int  firstChildIndex        = idList.indexOf(possibleFirstChildId);

        for (int i = firstChildIndex; i < idList.count(); i++) {
            int childId = idList.at(i);
            int indent  = getIndent(projectId, childId);

            if (indent < possibleFirstChildIndent) {
                break;
            }

            if (indent == possibleFirstChildIndent) {
                if (getTrashed(projectId, childId) == false) {
                    haveOneNotTrashedChild = true;
                }
                else {
                    haveOneTrashedChild = true;
                }
            }
        }

        if (haveOneTrashedChild && trashedAreIncluded) {
            return true;
        }

        if (haveOneNotTrashedChild && notTrashedAreIncluded) {
            return true;
        }
        return false;
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return false;
}

// ----------------------------------------------------------------------------------------

int SKRTreeHub::getTopTreeItemId(int projectId) const
{
    int value       = -2;
    QList<int> list = this->getAllIds(projectId);

    for (int id : qAsConst(list)) {
        if (!this->getTrashed(projectId, id)) {
            value = id;
            break;
        }
    }


    return value;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::getError()
{
    return m_error;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::set(int             projectId,
                          int             treeItemId,
                          const QString & fieldName,
                          const QVariant& value,
                          bool            setCurrentDateBool,
                          bool            commit)
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();
    result = queries.set(treeItemId, fieldName, value);

    if (setCurrentDateBool) {
        IFOKDO(result, queries.setCurrentDate(treeItemId, "dt_updated"));
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

// ----------------------------------------------------------------------------------------

QVariant SKRTreeHub::get(int projectId, int treeItemId, const QString& fieldName) const
{
    SKRResult result(this);
    QVariant  var;
    QVariant  value;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(treeItemId, fieldName, var);
    IFOK(result) {
        value = var;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return value;
}

// ----------------------------------------------------------------------------------------

int SKRTreeHub::getLastAddedId()
{
    return m_last_added_id;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::addTreeItem(const QHash<QString, QVariant>& values, int projectId)
{
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();
    int newId        = -1;
    SKRResult result = queries.add(values, newId);

    IFOKDO(result, queries.renumberSortOrder());
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
        result.addData("treeItemId", newId);
        emit treeItemAdded(projectId, newId);
        emit projectModified(projectId);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::addTreeItemAbove(int projectId, int targetId, const QString& type)
{
    int target_indent = getIndent(projectId, targetId);

    SKRResult result(this);
    int finalSortOrder = this->getValidSortOrderBeforeTree(projectId, targetId);

    // finally add treeItem
    QHash<QString, QVariant> values;

    values.insert("l_sort_order", finalSortOrder);
    values.insert("l_indent",     target_indent);
    values.insert("t_type",       type);
    IFOKDO(result, addTreeItem(values, projectId));
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::addTreeItemBelow(int projectId, int targetId, const QString& type)
{
    int target_indent = getIndent(projectId, targetId);

    SKRResult result(this);
    int finalSortOrder = this->getValidSortOrderAfterTree(projectId, targetId);

    // finally add treeItem
    QHash<QString, QVariant> values;

    values.insert("l_sort_order", finalSortOrder);
    values.insert("l_indent",     target_indent);
    values.insert("t_type",       type);
    IFOKDO(result, addTreeItem(values, projectId));
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::addChildTreeItem(int projectId, int targetId, const QString& type)
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, m_tableName);


    int target_sort_order = getSortOrder(projectId, targetId);
    int target_indent     = getIndent(projectId, targetId);

    // for invalid parent ("root")
    if (targetId == -2) {
        result = SKRResult(SKRResult::Critical, this, "invalid_root_parent");
        return result;
    }

    // for project item as parent :
    if (targetId == 0) {
        target_indent = 0;

        // get the highest sort order
        QHash<int, QVariant> sortOrderResult;
        result = queries.getValueByIds("l_sort_order",
                                       sortOrderResult,
                                       QString(),
                                       QVariant(),
                                       true);

        target_sort_order = 0;

        for (const QVariant& sortOrder : sortOrderResult.values()) {
            target_sort_order = qMax(sortOrder.toInt(), target_sort_order);
        }
    }

    // find next node with the same indentation
    QHash<int, QVariant> hash;
    QHash<QString, QVariant> where;

    where.insert("l_indent <=",    target_indent);
    where.insert("l_sort_order >", target_sort_order);
    result = queries.getValueByIdsWhere("l_sort_order", hash, where, true);
    int finalSortOrder = 0;

    // if node after
    if (!hash.isEmpty()) {
        QHash<int, QVariant>::const_iterator i = hash.constBegin();
        int lowestSort                         = 0;
        lowestSort = i.value().toInt();

        while (i != hash.constEnd()) {
            int sort = i.value().toInt();

            if (sort < lowestSort) {
                lowestSort = sort;
            }

            ++i;
        }

        finalSortOrder = lowestSort - 1;

        // if tree is empty

        if (finalSortOrder == -1) {
            finalSortOrder = 0;
        }
    }

    // if no node after (bottom of tree)
    else {
        QList<int> idList;
        IFOKDO(result, queries.getSortedIds(idList));

        if (idList.isEmpty()) {
            finalSortOrder = 1000;
        } else {
            int lastId = idList.last();
            QHash<int, QVariant> hash2;
            IFOKDO(result,
                   queries.getValueByIds("l_sort_order", hash2, "id",
                                         QVariant(lastId)));
            finalSortOrder = hash2.begin().value().toInt() + 1;
        }
    }

    // finally add treeItem
    QHash<QString, QVariant> values;

    values.insert("l_sort_order", finalSortOrder);
    values.insert("l_indent",     target_indent + 1);
    values.insert("t_type",       type);
    IFOKDO(result, addTreeItem(values, projectId));
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::removeTreeItem(int projectId, int targetId)
{
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();
    SKRResult result = queries.remove(targetId);

    IFOKDO(result, queries.renumberSortOrder());
    IFOKDO(result, queries.trimTreePropertyTable());
    IFOKDO(result, queries.trimTagRelationshipTable());
    IFOKDO(result, queries.trimTreeRelationshipTable());
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
        emit treeItemRemoved(projectId, targetId);
        emit projectModified(projectId);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::moveTreeItem(int sourceProjectId, int sourceTreeItemId, int targetTreeItemId, bool after)
{
    SKRResult result(this);

    // TODO: adapt to multiple projects
    int targetProjectId = sourceProjectId;

    QList<int> childrenList = this->getAllChildren(sourceProjectId, sourceTreeItemId);

    PLMSqlQueries queries(sourceProjectId, m_tableName);

    if (targetTreeItemId == 0) { // means end of list, so add to end
        after = true;
        int lastChildrenId = this->getAllIds(targetProjectId).last();
        targetTreeItemId = lastChildrenId;
    }


    int targetSortOrder = this->getSortOrder(targetProjectId, targetTreeItemId);


    if (after && this->hasChildren(targetProjectId, targetTreeItemId, true, true)) {
        // find the child at the most down of the target
        int lastChildrenId = this->getAllChildren(targetProjectId, targetTreeItemId).last();
        targetTreeItemId = lastChildrenId;
        targetSortOrder  = this->getSortOrder(targetProjectId, lastChildrenId);
    }

    targetSortOrder = targetSortOrder + (after ? 1 : -(childrenList.count() + 1));
    result          = setSortOrder(sourceProjectId, sourceTreeItemId, targetSortOrder, true, false);

    for (int childId : qAsConst(childrenList)) {
        targetSortOrder += 1;
        result           = setSortOrder(sourceProjectId, childId, targetSortOrder, true, false);
    }

    childrenList.prepend(sourceTreeItemId);

    IFOKDO(result, queries.renumberSortOrder())

    IFKO(result) {
        queries.rollback();
        emit errorSent(result);
    }
    IFOK(result) {
        queries.commit();
    }
    IFOK(result) {
        emit treeItemMoved(sourceProjectId, childrenList, targetProjectId, targetTreeItemId);
        emit projectModified(sourceProjectId);

        if (sourceProjectId != targetProjectId) {
            emit projectModified(targetProjectId);
        }
    }

    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::moveTreeItemUp(int projectId, int treeItemId)
{
    SKRResult result(this);

    PLMSqlQueries queries(projectId, m_tableName);

    // get treeItem before this :

    QHash<int, QVariant> sortOrderResult;

    result = queries.getValueByIds("l_sort_order",
                                   sortOrderResult,
                                   QString(),
                                   QVariant(),
                                   true);


    QList<int> idList;

    IFOKDO(result, queries.getSortedIds(idList));

    if (idList.first() == treeItemId) {
        result = SKRResult(SKRResult::Critical, this, "first_in_idList_cant_move_up");
    }
    int targetTreeItemId = -2;

    IFOK(result) {
        // find treeItem before with same indent
        int possibleTargetTreeItemId = -2;

        for (int i = idList.indexOf(treeItemId) - 1; i >= 0; --i) {
            possibleTargetTreeItemId = idList.at(i);

            if (this->getIndent(projectId,
                                possibleTargetTreeItemId) ==
                this->getIndent(projectId, treeItemId)) {
                targetTreeItemId = possibleTargetTreeItemId;
                break;
            }
        }

        if (possibleTargetTreeItemId == -2) {
            result = SKRResult(SKRResult::Critical, this, "possibleTargetTreeItemId_is_-2");
        }
        IFOK(result) {
            int targetIndent   = this->getIndent(projectId, targetTreeItemId);
            int treeItemIndent = this->getIndent(projectId, treeItemId);

            if (treeItemIndent  != targetIndent) {
                result = SKRResult(SKRResult::Critical, this, "indents_dont_match");
            }
        }
    }
    IFOKDO(result, this->moveTreeItem(projectId, treeItemId, targetTreeItemId))


    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::moveTreeItemDown(int projectId, int treeItemId)
{
    SKRResult result(this);

    PLMSqlQueries queries(projectId, m_tableName);

    // get treeItem before this :

    QHash<int, QVariant> sortOrderResult;

    result = queries.getValueByIds("l_sort_order",
                                   sortOrderResult,
                                   QString(),
                                   QVariant(),
                                   true);


    QList<int> idList;

    IFOKDO(result, queries.getSortedIds(idList));

    if (idList.last() == treeItemId) {
        result = SKRResult(SKRResult::Critical, this, "last_in_idList_cant_move_down");
    }
    int targetTreeItemId = -2;

    IFOK(result) {
        // find treeItem after with same indent
        int possibleTargetTreeItemId = -2;

        for (int i = idList.indexOf(treeItemId) + 1; i < idList.count(); ++i) {
            possibleTargetTreeItemId = idList.at(i);

            if (this->getIndent(projectId,
                                possibleTargetTreeItemId) ==
                this->getIndent(projectId, treeItemId)) {
                targetTreeItemId = possibleTargetTreeItemId;
                break;
            }
        }

        if (possibleTargetTreeItemId == -2) {
            result = SKRResult(SKRResult::Critical, this, "possibleTargetTreeItemId_is_-2");
        }
        IFOK(result) {
            int targetIndent   = this->getIndent(projectId, targetTreeItemId);
            int treeItemIndent = this->getIndent(projectId, treeItemId);

            if (treeItemIndent  != targetIndent) {
                result = SKRResult(SKRResult::Critical, this, "indents_dont_match");
            }
        }
    }
    IFOKDO(result, this->moveTreeItem(projectId, treeItemId, targetTreeItemId, true))


    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::moveTreeItemAsChildOf(int projectId, int noteId, int targetParentId, int wantedSortOrder)
{
    SKRResult result(this);

    int validSortOrder = getValidSortOrderAfterTree(projectId, targetParentId);

    if (wantedSortOrder == -1) {
        wantedSortOrder = validSortOrder;
    }

    if (wantedSortOrder > validSortOrder) {
        result = SKRResult(SKRResult::Critical, this, "wantedSortOrder_is_outside_scope_of_parent");
    }
    IFOK(result) {
        result = this->setSortOrder(projectId, noteId, wantedSortOrder);
    }
    IFOK(result) {
        int parentIndent = this->getIndent(projectId, targetParentId);

        result = this->setIndent(projectId, noteId, parentIndent + 1);
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

int SKRTreeHub::getParentId(int projectId, int treeItemId)
{
    int parentId = -2;

    // get indents
    QHash<int, int> indentList = getAllIndents(projectId);
    QList<int> sortedIdList    = getAllIds(projectId);


    // determine direct ancestor

    int indent = indentList.value(treeItemId);

    if (indent == 0) {
        return -1;
    }

    for (int i = sortedIdList.indexOf(treeItemId); i >= 0; i--) {
        int id = sortedIdList.at(i);

        //        if (id == treeItemId) {
        //            continue;
        //        }

        int idIndent = indentList.value(id);

        if (idIndent == indent - 1) {
            parentId = id;
            break;
        }
    }


    return parentId;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::renumberSortOrders(int projectId)
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.renumberSortOrder();
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

int SKRTreeHub::getValidSortOrderBeforeTree(int projectId, int treeItemId) const
{
    int target_sort_order = getSortOrder(projectId, treeItemId);

    int finalSortOrder = target_sort_order - 1;

    return finalSortOrder;
}

// ----------------------------------------------------------------------------------------

int SKRTreeHub::getValidSortOrderAfterTree(int projectId, int treeItemId) const
{
    int target_sort_order = getSortOrder(projectId, treeItemId);
    int target_indent     = getIndent(projectId, treeItemId);

    // find next node with the same indentation
    QHash<int, QVariant> hash;
    QHash<QString, QVariant> where;

    where.insert("l_indent <=",    target_indent);
    where.insert("l_sort_order >", target_sort_order);
    PLMSqlQueries queries(projectId, m_tableName);
    SKRResult     result = queries.getValueByIdsWhere("l_sort_order", hash, where, true);
    int finalSortOrder   = 0;

    // if node after
    if (!hash.isEmpty()) {
        QHash<int, QVariant>::const_iterator i = hash.constBegin();
        int lowestSort                         = 0;
        lowestSort = i.value().toInt();

        while (i != hash.constEnd()) {
            int sort = i.value().toInt();

            if (sort < lowestSort) {
                lowestSort = sort;
            }

            ++i;
        }

        finalSortOrder = lowestSort - 1;

        // if tree is empty
        if (finalSortOrder == -1) {
            finalSortOrder = 0;
        }
    }

    // if no node after (bottom of tree)
    else {
        QList<int> idList;
        IFOKDO(result, queries.getSortedIds(idList));

        if (idList.isEmpty()) {
            result = SKRResult(SKRResult::Critical, this, "idList_is_empty");
        }

        int lastId = idList.last();
        QHash<int, QVariant> hash2;
        IFOKDO(result,
               queries.getValueByIds("l_sort_order", hash2, "id", QVariant(lastId)));
        finalSortOrder = hash2.begin().value().toInt() + 1;
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return finalSortOrder;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::sortAlphabetically(int projectId, int parentTreeItemId)
{
    SKRResult result(this);

    QList<int> directChildren = this->getAllDirectChildren(projectId, parentTreeItemId, false, true);
    QList<int> allIds         = this->getAllIds(projectId);

    QStringList titleList;

    for (int directChildId : qAsConst(directChildren)) {
        titleList << this->getTitle(projectId, directChildId);
    }

    QCollator collator;
    collator.setCaseSensitivity(Qt::CaseInsensitive);
    collator.setNumericMode(true);
    std::sort(titleList.begin(), titleList.end(), collator);


    QMultiHash<QString, int> allTitlesWithIds;

    for (int directChildId : qAsConst(directChildren)) {
        allTitlesWithIds.insert(this->getTitle(projectId, directChildId), directChildId);
    }

    QList<int> newOrderedDirectChildrenList;

    for (const QString& title : qAsConst(titleList)) {
        QMultiHash<QString, int>::iterator i = allTitlesWithIds.begin();

        while (i != allTitlesWithIds.end()) {
            if (i.key() == title) {
                newOrderedDirectChildrenList.append(i.value());
                i = allTitlesWithIds.erase(i);
            }
            else {
                ++i;
            }
        }
    }


    int currentParentId = -1;
    QList<int> children;

    bool parentPassed = false;

    for (int id : qAsConst(allIds)) {
        if (directChildren.contains(id)) {
            // first loop
            if (currentParentId != -1) {
                int insertionIndex = newOrderedDirectChildrenList.indexOf(currentParentId) + 1;

                for (int child : children) {
                    newOrderedDirectChildrenList.insert(insertionIndex, child);
                    insertionIndex += 1;
                }
            }
            children.clear();
            currentParentId = id;
        }
        else {
            children.append(id);
        }
    }

    int newSortOrder = this->getSortOrder(projectId, parentTreeItemId) + 1;

    for (int id : qAsConst(newOrderedDirectChildrenList)) {
        IFOKDO(result, this->setSortOrder(projectId, id, newSortOrder, true, false));
        newSortOrder += 1;
    }


    IFOKDO(result, this->renumberSortOrders(projectId));
    PLMSqlQueries queries(projectId, m_tableName);

    IFKO(result) {
        queries.rollback();
        emit errorSent(result);
    }
    IFOK(result) {
        queries.commit();

        for (int id : qAsConst(newOrderedDirectChildrenList)) {
            emit sortOrderChanged(projectId, id, this->getSortOrder(projectId, id));
        }
        emit projectModified(projectId);
    }

    return result;
}

// ----------------------------------------------------------------------------------------

QList<int>SKRTreeHub::getAllChildren(int projectId, int treeItemId)
{
    QList<int> childrenList;

    // get indents
    QHash<int, int> indentList = getAllIndents(projectId);
    QList<int> sortedIdList    = getAllIds(projectId);


    // determine children

    int  parentIndent = indentList.value(treeItemId);
    bool parentPassed = false;

    for (int id : qAsConst(sortedIdList)) {
        if (id == treeItemId) {
            parentPassed = true;
            continue;
        }

        if (parentPassed) {
            int idIndent = indentList.value(id);

            if (idIndent > parentIndent) {
                childrenList.append(id);
            }

            if (idIndent <= parentIndent) {
                break;
            }
        }
    }

    return childrenList;
}

// ----------------------------------------------------------------------------------------

QList<int>SKRTreeHub::getAllDirectChildren(int  projectId,
                                           int  treeItemId,
                                           bool trashedAreIncluded,
                                           bool notTrashedAreIncluded)
{
    QList<int> childrenList;

    // get indents
    QHash<int, int> indentList = getAllIndents(projectId);
    QList<int> sortedIdList    = getAllIds(projectId);


    // determine children

    int  parentIndent = indentList.value(treeItemId);
    bool parentPassed = false;

    for (int id : qAsConst(sortedIdList)) {
        if (id == treeItemId) {
            parentPassed = true;
            continue;
        }

        if (parentPassed) {
            int idIndent = indentList.value(id);

            if (idIndent == parentIndent + 1) {
                bool isTrashed = this->getTrashed(projectId, id);

                if (trashedAreIncluded && isTrashed) {
                    childrenList.append(id);
                }

                if (notTrashedAreIncluded && !isTrashed) {
                    childrenList.append(id);
                }
            }

            if (idIndent <= parentIndent) {
                break;
            }
        }
    }

    return childrenList;
}

// ----------------------------------------------------------------------------------------

QList<int>SKRTreeHub::getAllAncestors(int projectId, int treeItemId)
{
    QList<int> ancestorsList;

    // get indents
    QHash<int, int> indentList = getAllIndents(projectId);
    QList<int> sortedIdList    = getAllIds(projectId);


    // determine ancestors

    int indent = indentList.value(treeItemId);


    for (int i = sortedIdList.indexOf(treeItemId); i >= 0; i--) {
        int id = sortedIdList.at(i);

        //        if (id == treeItemId) {
        //            continue;
        //        }

        int idIndent = indentList.value(id);

        if (idIndent == indent - 1) {
            if (indent == -1) {
                break;
            }

            ancestorsList.append(id);

            indent = idIndent;
        }
    }

    return ancestorsList;
}

// ----------------------------------------------------------------------------------------

QList<int>SKRTreeHub::getAllSiblings(int projectId, int treeItemId)
{
    QList<int> siblingsList;

    // get indents
    QHash<int, int> indentList = getAllIndents(projectId);
    QList<int> sortedIdList    = getAllIds(projectId);
    int treeItemSortedIdIndex  = sortedIdList.indexOf(treeItemId);


    // determine siblings

    int indent = indentList.value(treeItemId);

    // min sibling index
    int minSiblingIndex = treeItemSortedIdIndex;

    for (int i = treeItemSortedIdIndex; i >= 0; i--) {
        int id = sortedIdList.at(i);

        //        if (id == treeItemId) {
        //            continue;
        //        }

        int idIndent = indentList.value(id);

        if ((idIndent == indent - 1) || (indent == -1)) {
            break;
        }
        minSiblingIndex = i;
    }

    // min sibling index
    int maxSiblingIndex = treeItemSortedIdIndex;

    for (int i = treeItemSortedIdIndex; i < sortedIdList.count(); i++) {
        int id = sortedIdList.at(i);

        //        if (id == treeItemId) {
        //            continue;
        //        }

        int idIndent = indentList.value(id);

        if ((idIndent == indent - 1) || (indent == -1)) {
            break;
        }
        maxSiblingIndex = i;
    }

    // alone, so no siblings
    if ((minSiblingIndex == treeItemSortedIdIndex) &&
        (maxSiblingIndex == treeItemSortedIdIndex)) {
        return siblingsList;
    }

    // same level

    for (int i = minSiblingIndex; i <= maxSiblingIndex; i++) {
        int id = sortedIdList.at(i);

        //        if (id == treeItemId) {
        //            continue;
        //        }

        int idIndent = indentList.value(id);

        if (idIndent == indent) {
            siblingsList.append(id);
        }
    }

    siblingsList.removeAll(treeItemId);


    return siblingsList;
}

// ----------------------------------------------------------------------------------------

QDateTime SKRTreeHub::getTrashedDate(int projectId, int treeItemId) const
{
    return get(projectId, treeItemId, "dt_trashed").toDateTime();
}

// ----------------------------------------------------------------------------------------

QList<int>SKRTreeHub::getTreeRelationshipSourcesFromReceiverId(int projectId, int receiverTreeItemId) const
{
    SKRResult  result;
    QList<int> list;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, "tbl_tree_relationship");

    result = queries.getValueByIds("l_tree_source_code", out, "l_tree_receiver_code", receiverTreeItemId);

    IFOK(result) {
        list = HashIntQVariantConverter::convertToIntInt(out).values();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// ----------------------------------------------------------------------------------------

QList<int>SKRTreeHub::getTreeRelationshipReceiversFromSourceId(int projectId, int sourceTreeItemId) const
{
    SKRResult  result;
    QList<int> list;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, "tbl_tree_relationship");

    result = queries.getValueByIds("l_tree_receiver_code", out, "l_tree_source_code", sourceTreeItemId);

    IFOK(result) {
        list = HashIntQVariantConverter::convertToIntInt(out).values();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setTreeRelationship(int projectId, int sourceTreeItemId, int receiverTreeItemId)
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_tree_receiver_code", receiverTreeItemId);
    where.insert("l_tree_source_code",   sourceTreeItemId);

    PLMSqlQueries queries(projectId, "tbl_tree_relationship");


    // verify if the relationship doesn't yet exist
    result = queries.getValueByIdsWhere("l_tree_relationship_id", out, where);

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
            values.insert("l_tree_receiver_code", receiverTreeItemId);
            values.insert("l_tree_source_code",   sourceTreeItemId);
            result = queries.add(values, newId);

            IFOK(result) {
                emit treeRelationshipAdded(projectId, sourceTreeItemId, receiverTreeItemId);
                emit projectModified(projectId);
            }
        }
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::removeTreeRelationship(int projectId, int sourceTreeItemId, int receiverTreeItemId)
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_tree_receiver_code", receiverTreeItemId);

    PLMSqlQueries queries(projectId, "tbl_tree_relationship");

    result = queries.getValueByIdsWhere("l_tree_source_code", out, where);

    int key = -2;

    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator i = hash.constBegin();

        while (i != hash.constEnd()) {
            key = i.key();
            ++i;
        }

        if (hash.isEmpty() || (key == -2) || (key == 0)) {
            result = SKRResult(SKRResult::Critical, this, "no_tree_relationship_to_remove");
        }
    }

    IFOK(result) {
        result = queries.remove(key);
    }
    IFOK(result) {
        emit treeRelationshipRemoved(projectId, sourceTreeItemId, receiverTreeItemId);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }

    return result;
}

// ----------------------------------------------------------------------------------------

int SKRTreeHub::getPreviousTreeItemIdOfTheSameType(int projectId, int treeItemId)
{
    SKRResult result(this);
    int previousTreeItemId = -1;

    QList<int> allIds = this->getAllIds(projectId);

    int thisItemIndex    = allIds.indexOf(treeItemId);
    QString thisItemType = this->getType(projectId, treeItemId);

    if (thisItemIndex == -1) {
        result = SKRResult(SKRResult::Critical, this, "no_index_found");
    }

    if (thisItemIndex <= 1) {
        return -1;
    }

    IFOK(result) {
        for (int i = thisItemIndex - 1; i >= 0; i--) {
            int potentiallyPreviousItemId = allIds.at(i);

            if (!this->getTrashed(projectId, potentiallyPreviousItemId)) {
                QString previousItemType = this->getType(projectId, potentiallyPreviousItemId);

                if (previousItemType == thisItemType) {
                    previousTreeItemId = potentiallyPreviousItemId;
                    break;
                }
            }
        }
    }
    IFKO(result) {
        emit errorSent(result);
    }

    return previousTreeItemId;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setTrashedDateToNow(int projectId, int treeItemId)
{
    return set(projectId, treeItemId, "dt_trashed", QDateTime::currentDateTime());
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setTrashedDateToNull(int projectId, int treeItemId)
{
    return set(projectId, treeItemId, "dt_trashed", "NULL");
}

// ----------------------------------------------------------------------------------------


void SKRTreeHub::setError(const SKRResult& result)
{
    m_error = result;
}

// ----------------------------------------------------------------------------------------

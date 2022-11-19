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
#include "sql/plmsqlqueries.h"
#include "tools.h"
#include "skrdata.h"

#include <QCollator>
#include <QHash>

SKRTreeHub::SKRTreeHub(QObject *parent) : QObject(parent), m_tableName("tbl_tree"), m_last_added_address(TreeItemAddress()), m_cutCopy(
                                                                                                             CutCopy())
{
    connect(this,                  &SKRTreeHub::errorSent,        this, &SKRTreeHub::setError, Qt::DirectConnection);

    // reset m_cutCopy
    connect(skrdata->projectHub(), &PLMProjectHub::projectClosed, this, [this](int projectId) {
        // clear current cutCopy
        if (!m_cutCopy.treeItemAddresses.isEmpty() && m_cutCopy.treeItemAddresses.first().projectId == projectId) {
            m_cutCopy = CutCopy();
        }
    });

    connect(skrdata->projectHub(), &PLMProjectHub::projectLoaded, this, &SKRTreeHub::resetCache);

    connect(this, &SKRTreeHub::treeReset, this, &SKRTreeHub::resetCache);
    connect(this, &SKRTreeHub::allValuesChanged, this, &SKRTreeHub::resetCacheByAddress);
    connect(this, &SKRTreeHub::treeItemAdded, this, &SKRTreeHub::resetCacheByAddress);
    connect(this, &SKRTreeHub::treeItemsAdded, this, [this](QList<TreeItemAddress> treeItemAddresses){
        if(!treeItemAddresses.isEmpty()){
            this->resetCache(treeItemAddresses.first().projectId);
        }
    });
    connect(this, &SKRTreeHub::treeItemAboutToBeRemoved, this, &SKRTreeHub::resetCacheByAddress);
    connect(this, &SKRTreeHub::treeItemRemoved, this, &SKRTreeHub::resetCacheByAddress);
    connect(this, &SKRTreeHub::treeItemMoved, this, [this](QList<TreeItemAddress> sourceTreeItemAddresses,
            const TreeItemAddress &targetTreeItemAddress){
    });
    connect(this, &SKRTreeHub::allValuesChanged, this, &SKRTreeHub::resetCacheByAddress);

}

// ----------------------------------------------------------------------------------------

void SKRTreeHub::resetCache(int projectId)
{
    m_getAllIds_cache.remove(projectId);
    m_getAllIds_cache.insert(projectId, this->getAllIds(projectId));

    m_getAllSortOrders_cache.remove(projectId);
    m_getAllSortOrders_cache.insert(projectId, this->getAllSortOrders(projectId));

    m_getAllIndents_cache.remove(projectId);
    m_getAllIndents_cache.insert(projectId, this->getAllIndents(projectId));
}

void SKRTreeHub::resetCacheByAddress(const TreeItemAddress &treeItemAddress)
{
    this->resetCache(treeItemAddress.projectId);
}
// ----------------------------------------------------------------------------------------

QHash<TreeItemAddress, int>SKRTreeHub::getAllSortOrders(int projectId) const
{
    if(m_getAllSortOrders_cache.contains(projectId)){
        return m_getAllSortOrders_cache.value(projectId);
    }

    SKRResult result(this);

    QHash<TreeItemAddress, int> final;
    QHash<int, int> hash;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("l_sort_order", out, "", QVariant(), true);
    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntInt(out);

        QHash<int, int>::const_iterator i = hash.constBegin();
        while(i != hash.constEnd()){
            final.insert(TreeItemAddress(projectId, i.key()), i.value());
            i++;
        }
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return final;
}

// ----------------------------------------------------------------------------------------

QHash<TreeItemAddress, int>SKRTreeHub::getAllIndents(int projectId) const
{


    if(m_getAllIndents_cache.contains(projectId)){
        return m_getAllIndents_cache.value(projectId);
    }

    SKRResult result(this);

    QHash<int, int> hash;
    QHash<TreeItemAddress, int> final;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("l_indent", out, "", QVariant(), true);
    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntInt(out);


        QHash<int, int>::const_iterator i = hash.constBegin();
        while(i != hash.constEnd()){
            final.insert(TreeItemAddress(projectId, i.key()), i.value());
            i++;
        }

    }
    IFKO(result) {
        emit errorSent(result);
    }
    return final;
}

// ----------------------------------------------------------------------------------------

///
/// \brief SKRTreeHub::getAllIds
/// \param projectId
/// \return
/// Get sorted ids, trashed ids included
QList<TreeItemAddress>SKRTreeHub::getAllIds(int projectId) const
{
    SKRResult result(this);


    if(m_getAllIds_cache.contains(projectId)){
        return m_getAllIds_cache.value(projectId);
    }

    QList<TreeItemAddress> list;

    QList<int> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getSortedIds(out);
    IFOK(result) {
        for(int itemId: out){
            list << TreeItemAddress(projectId, itemId);
        }
    }
    IFKO(result) {
        emit errorSent(result);
    }



    return list;
}

//-------------------------------------------------------

QList<TreeItemAddress> SKRTreeHub::getAllTrashedIds(int projectId) const
{

    SKRResult result(this);

    QList<TreeItemAddress> list;


    QHash<int, QVariant> out;
    QHash<int, int> hash;
    QHash<QString, QVariant> where;

    where.insert("b_trashed =", 1);

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIdsWhere("l_tree_id", out, where);

    hash = HashIntQVariantConverter::convertToIntInt(out);

    IFOK(result) {

        QHash<int, int>::const_iterator i = hash.constBegin();
        while(i != hash.constEnd()){
            list << TreeItemAddress(projectId, i.value());
            i++;
        }

    }
    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

//-------------------------------------------------------

QList<QVariantMap> SKRTreeHub::saveTree(int projectId) const
{
    SKRResult result(this);

    PLMSqlQueries queries(projectId, m_tableName);
    QStringList fieldNames = queries.getAllFieldTitles();

    QVariantMap allFields;
    QList<QVariantMap> list;

    for(const TreeItemAddress &treeItemAddress : this->getAllIds(projectId)){

        for(const QString &fieldName : fieldNames) {
            allFields.insert(fieldName, this->get(treeItemAddress, fieldName));
        }

        list.append(allFields);
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return list;

}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::restoreTree(int projectId, QList<QVariantMap> allValues)
{
    SKRResult result(this);

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.injectDirectSql("PRAGMA foreign_keys = 0");
    result = queries.injectDirectSql("DELETE FROM tbl_tree");

    for(const QVariantMap &values : allValues){

        QHash<QString, QVariant> hash;
        QVariantMap::const_iterator i = values.constBegin();
        while (i != values.constEnd()) {
            hash.insert(i.key(), i.value());
            ++i;
        }
        int newId;
        queries.add(hash, newId);

    }
    result = queries.injectDirectSql("PRAGMA foreign_keys = 1");

    IFOK(result) {
        queries.commit();
        emit treeReset(projectId);
        emit projectModified(projectId);
    }
    IFKO(result) {
        queries.rollback();
        emit errorSent(result);
    }
    return result;

}

// ----------------------------------------------------------------------------------------

QVariantMap SKRTreeHub::saveId(const TreeItemAddress &treeItemAddress) const
{
    SKRResult result(this);

    PLMSqlQueries queries(treeItemAddress.projectId, m_tableName);
    QStringList fieldNames = queries.getAllFieldTitles();

    QVariantMap allFields;

    for(const QString &fieldName : fieldNames) {
        allFields.insert(fieldName, this->get(treeItemAddress, fieldName));
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return allFields;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::restoreId(const TreeItemAddress &treeItemAddress, const QVariantMap &values)
{
    SKRResult result(this);

    QVariantMap::const_iterator i = values.constBegin();
    while (i != values.constEnd()) {
        result = set(treeItemAddress, i.key(), i.value(), false, false);
        ++i;
    }
    this->commit(treeItemAddress.projectId);


    IFOK(result) {
        // do like if a tree item was added :
        emit treeItemAdded(treeItemAddress);
        emit projectModified(treeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setTreeItemId(const TreeItemAddress &treeItemAddress, int newId)
{

    SKRResult result = set(treeItemAddress, "l_tree_id", newId);

    IFOK(result) {
        emit treeItemIdChanged(treeItemAddress, newId);
        emit projectModified(treeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setTitle(const TreeItemAddress &treeItemAddress, const QString& newTitle)
{
    SKRResult result = set(treeItemAddress, "t_title", newTitle);

    IFOK(result) {
        emit titleChanged(treeItemAddress, newTitle);
        emit projectModified(treeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QString SKRTreeHub::getTitle(const TreeItemAddress &treeItemAddress) const
{
    return get(treeItemAddress, "t_title").toString();
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setInternalTitle(const TreeItemAddress &treeItemAddress, const QString& internalTitle)
{

    SKRResult result = set(treeItemAddress, "t_internal_title", internalTitle);

    IFOK(result) {
        emit internalTitleChanged(treeItemAddress, internalTitle);
        emit projectModified(treeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::removeInternalTitleFromAll(int projectId, const QString& internalTitle)
{
    SKRResult result(this);

    QList<TreeItemAddress> idList = this->getAllIds(projectId);

    for (const TreeItemAddress &treeItemAddress : qAsConst(idList)) {
        if (this->getInternalTitle(treeItemAddress) == internalTitle) {
            result = this->setInternalTitle(treeItemAddress, "");
        }
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QList<TreeItemAddress>SKRTreeHub::getIdsWithInternalTitle(int projectId, const QString& internalTitle) const
{
    SKRResult result(this);
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("t_internal_title", internalTitle);

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIdsWhere("t_title", out, where);

    QList<TreeItemAddress> treeItemAddresses;
    for(int itemId: out.keys()){
        treeItemAddresses << TreeItemAddress(projectId, itemId);

    }

    return treeItemAddresses;
}

// ----------------------------------------------------------------------------------------

QString SKRTreeHub::getInternalTitle(const TreeItemAddress &treeItemAddress) const
{
    return get(treeItemAddress, "t_internal_title").toString();
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setIndent(const TreeItemAddress &treeItemAddress, int newIndent)
{
     return this->setIndent(treeItemAddress, newIndent, true, true);
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setIndent(const TreeItemAddress &treeItemAddress, int newIndent, bool setCurrentdate, bool commit)
{
    SKRResult result = set(treeItemAddress, "l_indent", newIndent, setCurrentdate, commit);

    IFOK(result) {
        emit indentChanged(treeItemAddress, newIndent);
        emit projectModified(treeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

int SKRTreeHub::getIndent(const TreeItemAddress &treeItemAddress) const
{
    int indent;

    if (treeItemAddress.itemId == -1) // is project item
        indent = -1;
    else {
        indent = get(treeItemAddress, "l_indent").toInt();
    }

    return indent;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setSortOrder(const TreeItemAddress &treeItemAddress, int newSortOrder)
{
    return this->setSortOrder(treeItemAddress, newSortOrder, true, true);
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setSortOrder(const TreeItemAddress &treeItemAddress, int newSortOrder, bool setCurrentdate, bool commit)
{
    SKRResult result = set(treeItemAddress, "l_sort_order", newSortOrder, setCurrentdate, commit);

    IFOK(result) {
        emit sortOrderChanged(treeItemAddress, newSortOrder);
        emit projectModified(treeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

int SKRTreeHub::getSortOrder(const TreeItemAddress &treeItemAddress) const
{
    return get(treeItemAddress, "l_sort_order").toInt();
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setType(const TreeItemAddress &treeItemAddress, const QString& newType)
{
    SKRResult result = set(treeItemAddress, "t_type", newType);

    IFOK(result) {
        emit typeChanged(treeItemAddress, newType);
        emit projectModified(treeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QString SKRTreeHub::getType(const TreeItemAddress &treeItemAddress) const
{
    return get(treeItemAddress, "t_type").toString();
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setPrimaryContent(const TreeItemAddress &treeItemAddress, const QString& newContent)
{
    return this->setPrimaryContent(treeItemAddress, newContent, true, true);
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setPrimaryContent(const TreeItemAddress &treeItemAddress,
                                        const QString& newContent,
                                        bool           setCurrentdate,
                                        bool           commit)
{
    SKRResult result = set(treeItemAddress, "m_primary_content", newContent, setCurrentdate, commit);

    IFOK(result) {
        emit primaryContentChanged(treeItemAddress, newContent);
        emit projectModified(treeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QString SKRTreeHub::getPrimaryContent(const TreeItemAddress &treeItemAddress) const
{
    QString content = get(treeItemAddress, "m_primary_content").toString();

    return content;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setSecondaryContent(const TreeItemAddress &treeItemAddress, const QString& newContent)
{
    return this->setSecondaryContent(treeItemAddress, newContent, true, true);
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setSecondaryContent(const TreeItemAddress &treeItemAddress,
                                          const QString& newContent,
                                          bool           setCurrentdate,
                                          bool           commit)
{
    SKRResult result = set(treeItemAddress, "m_secondary_content", newContent, setCurrentdate, commit);

    IFOK(result) {
        emit secondaryContentChanged(treeItemAddress, newContent);
        emit projectModified(treeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QString SKRTreeHub::getSecondaryContent(const TreeItemAddress &treeItemAddress) const
{
    QString content = get(treeItemAddress, "m_secondary_content").toString();

    return content;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setTrashedWithChildren(const TreeItemAddress &treeItemAddress, bool newTrashedState)
{
    SKRResult result(this);

    QDateTime parenTrashDate = this->getTrashedDate(treeItemAddress);
    QList<TreeItemAddress> childTreeItemAddresses = this->getAllChildren(treeItemAddress);

    QDateTime now = QDateTime::currentDateTime();


    // children deletion (or recovery)
    for (const TreeItemAddress &childTreeItemAddress : childTreeItemAddresses) {
        IFOKDO(result, set(childTreeItemAddress, "b_trashed", newTrashedState));

        // set date but those already deleted
        if (newTrashedState && this->getTrashedDate(childTreeItemAddress).isNull()) {
            result = this->setTrashedDateToNow(childTreeItemAddress, now);
            emit trashedChanged(childTreeItemAddress, newTrashedState);
        }

        // restore those deleted at the same time
        else if (!newTrashedState && this->getTrashedDate(childTreeItemAddress) == parenTrashDate) {
            result = this->setTrashedDateToNull(childTreeItemAddress);
            emit trashedChanged(childTreeItemAddress, newTrashedState);
        }

    }


    // do parent :
    IFOK(result) {
        result = set(treeItemAddress, "b_trashed", newTrashedState);

        // set date but those already deleted
        if (newTrashedState && this->getTrashedDate(treeItemAddress).isNull()) {
            result = this->setTrashedDateToNow(treeItemAddress, now);
        }

        // restore
        else if (!newTrashedState) {
            result = this->setTrashedDateToNull(treeItemAddress);
        }
    }

    IFOK(result) {
        emit trashedChanged(treeItemAddress, newTrashedState);
        emit projectModified(treeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::untrashOnlyOneTreeItem(const TreeItemAddress &treeItemAddress)
{
    SKRResult result = set(treeItemAddress, "b_trashed", false);

    IFOKDO(result, this->setTrashedDateToNull(treeItemAddress));

    IFOK(result) {
        emit trashedChanged(treeItemAddress, false);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

bool SKRTreeHub::getTrashed(const TreeItemAddress &treeItemAddress) const
{
    return get(treeItemAddress, "b_trashed").toBool();
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setCreationDate(const TreeItemAddress &treeItemAddress, const QDateTime& newDate)
{
    SKRResult result = set(treeItemAddress, "dt_created", newDate);

    IFOK(result) {
        emit creationDateChanged(treeItemAddress, newDate);
        emit projectModified(treeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QDateTime SKRTreeHub::getCreationDate(const TreeItemAddress &treeItemAddress) const
{
    return get(treeItemAddress, "dt_created").toDateTime();
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setUpdateDate(const TreeItemAddress &treeItemAddress, const QDateTime& newDate)
{
    SKRResult result = set(treeItemAddress, "dt_updated", newDate);

    IFOK(result) {
        emit updateDateChanged(treeItemAddress, newDate);
        emit projectModified(treeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

QDateTime SKRTreeHub::getUpdateDate(const TreeItemAddress &treeItemAddress) const
{
    return get(treeItemAddress, "dt_updated").toDateTime();
}

// ----------------------------------------------------------------------------------------

int SKRTreeHub::row(const TreeItemAddress &treeItemAddress) const
{
    int sortOrder = this->getSortOrder(treeItemAddress);
    const TreeItemAddress &parentTreeItemAddress = this->getParentId(treeItemAddress);
    int parentSortOrder = this->getSortOrder(parentTreeItemAddress);

    return (sortOrder - parentSortOrder) / 1000 - 1;

}

// ----------------------------------------------------------------------------------------

bool SKRTreeHub::hasChildren(const TreeItemAddress &treeItemAddress, bool trashedAreIncluded, bool notTrashedAreIncluded) const
{
    SKRResult result(this);
    PLMSqlQueries queries(treeItemAddress.projectId, m_tableName);

    // if last of id list:
    QList<int> idList;

    IFOKDO(result, queries.getSortedIds(idList));

    if (idList.isEmpty()) { // project item with nothing else
        return false;
    }

    if (treeItemAddress.itemId == idList.last()) {
        return false;
    }

    int indent = getIndent(treeItemAddress);

    int possibleFirstChildId;

    if (treeItemAddress.itemId == -1) {                  // project item
        possibleFirstChildId = idList.at(0); // first real treeItem in table
    }
    else {
        possibleFirstChildId = idList.at(idList.indexOf(treeItemAddress.itemId) + 1);
    }


    int possibleFirstChildIndent = getIndent(TreeItemAddress(treeItemAddress.projectId, possibleFirstChildId));

    // verify indent of child
    if (indent == possibleFirstChildIndent - 1) {
        // verify if at least one child is not trashed
        bool haveOneNotTrashedChild = false;
        bool haveOneTrashedChild    = false;
        int  firstChildIndex        = idList.indexOf(possibleFirstChildId);

        for (int i = firstChildIndex; i < idList.count(); i++) {
            int childId = idList.at(i);
            const TreeItemAddress &childTreeItemAddress = TreeItemAddress(treeItemAddress.projectId, childId);
            int indent  = getIndent(childTreeItemAddress);

            if (indent < possibleFirstChildIndent) {
                break;
            }

            if (indent == possibleFirstChildIndent) {
                if (getTrashed(childTreeItemAddress) == false) {
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

TreeItemAddress SKRTreeHub::getTopTreeItemId(int projectId) const
{
    TreeItemAddress value       = TreeItemAddress();
    QList<TreeItemAddress> list = this->getAllIds(projectId);

    for (const TreeItemAddress &treeItemAddress  : qAsConst(list)) {
        if (!this->getTrashed(treeItemAddress)) {
            value = treeItemAddress;
            break;
        }
    }


    return value;
}

// ----------------------------------------------------------------------------------------

QList<TreeItemAddress> SKRTreeHub::filterOutChildren(QList<TreeItemAddress> treeItemAddresses) const
{
    QList<TreeItemAddress> finalList;

    QSet<TreeItemAddress> childrenSet;

    for(const TreeItemAddress &treeItemAddress : treeItemAddresses){
        for(const TreeItemAddress &childTreeItemAddress : this->getAllChildren(treeItemAddress)){
            childrenSet.insert(childTreeItemAddress);
        }
    }
    for(const TreeItemAddress &treeItemAddress : treeItemAddresses){
        if(!childrenSet.contains(treeItemAddress)){
            finalList.append(treeItemAddress);
        }
    }
    return finalList;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::getError()
{
    return m_error;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::set(const TreeItemAddress &treeItemAddress,
                          const QString & fieldName,
                          const QVariant& value,
                          bool            setCurrentDateBool,
                          bool            commit)
{
    SKRResult result(this);
    PLMSqlQueries queries(treeItemAddress.projectId, m_tableName);

    queries.beginTransaction();
    result = queries.set(treeItemAddress.itemId, fieldName, value);

    if (setCurrentDateBool) {
        IFOKDO(result, queries.setCurrentDate(treeItemAddress.itemId, "dt_updated"));
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

QVariant SKRTreeHub::get(const TreeItemAddress &treeItemAddress, const QString& fieldName) const
{
    SKRResult result(this);
    QVariant  var;
    QVariant  value;
    PLMSqlQueries queries(treeItemAddress.projectId, m_tableName);

    result = queries.get(treeItemAddress.itemId, fieldName, var);
    IFOK(result) {
        value = var;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return value;
}

// ----------------------------------------------------------------------------------------

void SKRTreeHub::commit(int projectId)
{
    PLMSqlQueries queries(projectId, m_tableName);
    queries.commit();

}

// ----------------------------------------------------------------------------------------

TreeItemAddress SKRTreeHub::getLastAddedAddress()
{
    return m_last_added_address;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::addTreeItem(int projectId, int sortOrder, int indent, const QString &type, const QString &title, const QString &internalTitle, bool renumber)
{
    QHash<QString, QVariant> hash;
    hash.insert("l_sort_order", sortOrder);
    hash.insert("l_indent", indent);
    hash.insert("t_type", type);
    hash.insert("t_title", title);
    hash.insert("t_internal_title", internalTitle);

    return addTreeItem(hash, projectId, renumber);

}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::addTreeItem(const QHash<QString, QVariant>& values, int projectId, bool renumber)
{
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();
    int newId        = -1;
    SKRResult result = queries.add(values, newId);

    if(renumber){
        IFOKDO(result, queries.renumberSortOrder());
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
        m_last_added_address = TreeItemAddress(projectId, newId);
        result.addData("treeItemAddress", QVariant::fromValue<TreeItemAddress>(TreeItemAddress(projectId, newId)));
        emit treeItemAdded(TreeItemAddress(projectId, newId));
        emit projectModified(projectId);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::addTreeItemAbove(const TreeItemAddress &targetTreeItemAddress, const QString& type)
{
    int target_indent = getIndent(targetTreeItemAddress);

    SKRResult result(this);
    int finalSortOrder = this->getValidSortOrderBeforeTree(targetTreeItemAddress);

    // finally add treeItem
    QHash<QString, QVariant> values;

    values.insert("l_sort_order", finalSortOrder);
    values.insert("l_indent",     target_indent);
    values.insert("t_type",       type);
    IFOKDO(result, addTreeItem(values, targetTreeItemAddress.projectId));
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::addTreeItemBelow(const TreeItemAddress &targetTreeItemAddress, const QString& type)
{
    int target_indent = getIndent(targetTreeItemAddress);

    SKRResult result(this);
    int finalSortOrder = this->getValidSortOrderAfterTree(targetTreeItemAddress);

    // finally add treeItem
    QHash<QString, QVariant> values;

    values.insert("l_sort_order", finalSortOrder);
    values.insert("l_indent",     target_indent);
    values.insert("t_type",       type);
    IFOKDO(result, addTreeItem(values, targetTreeItemAddress.projectId));
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::addChildTreeItem(const TreeItemAddress &targetTreeItemAddress, const QString& type)
{
    SKRResult result(this);
    PLMSqlQueries queries(targetTreeItemAddress.projectId, m_tableName);


    int target_sort_order = getSortOrder(targetTreeItemAddress);
    int target_indent     = getIndent(targetTreeItemAddress);

    // for invalid parent ("root")
    if (!targetTreeItemAddress.isValid()) {
        result = SKRResult(SKRResult::Critical, this, "invalid_root_parent");
        return result;
    }

    // for project item as parent :
    if (targetTreeItemAddress.itemId == 0) {
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
    IFOKDO(result, addTreeItem(values, targetTreeItemAddress.projectId));
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::removeTreeItem(const TreeItemAddress &targetTreeItemAddress)
{
    PLMSqlQueries queries(targetTreeItemAddress.projectId, m_tableName);
    emit treeItemAboutToBeRemoved(targetTreeItemAddress);

    queries.beginTransaction();
    SKRResult result = queries.remove(targetTreeItemAddress.itemId);

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
        emit treeItemRemoved(targetTreeItemAddress);
        emit projectModified(targetTreeItemAddress.projectId);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::moveTreeItem(const TreeItemAddress &sourceTreeItemAddress,
                                   const TreeItemAddress &targetTreeItemAddress, bool after)
{
    SKRResult result(this);

    // TODO: adapt to multiple projects


    QList<TreeItemAddress> childrenList = this->getAllChildren(sourceTreeItemAddress);

    PLMSqlQueries queries(sourceTreeItemAddress.projectId, m_tableName);

    TreeItemAddress targetAddress = targetTreeItemAddress;

    if (targetAddress.itemId == 0) { // means end of list, so add to end
        after = true;
        TreeItemAddress lastChildrenId = this->getAllIds(targetTreeItemAddress.projectId).last();
        targetAddress = lastChildrenId;
    }


    int targetSortOrder = this->getSortOrder(targetAddress);
    int targetIndent = this->getIndent(targetAddress);
    int sourceIndent = this->getIndent(sourceTreeItemAddress);
    int indentDelta = targetIndent - sourceIndent;


    if (after && this->hasChildren(targetAddress, true, true)) {
        // find the child at the most down of the target
        TreeItemAddress lastChildrenId = this->getAllChildren(targetAddress).last();
        targetAddress = lastChildrenId;
        targetSortOrder  = this->getSortOrder(targetAddress);
    }

    targetSortOrder = targetSortOrder + (after ? 1 : -999);
    result          = setSortOrder(sourceTreeItemAddress, targetSortOrder, true, false);
    result           = setIndent(sourceTreeItemAddress, sourceIndent + indentDelta);

    for (const TreeItemAddress &childTreeItemAddress : qAsConst(childrenList)) {
        targetSortOrder += 1;
        result           = setSortOrder(childTreeItemAddress, targetSortOrder, true, false);

        int childIndent = this->getIndent(childTreeItemAddress);
        result           = setIndent(childTreeItemAddress, childIndent + indentDelta);
    }

    childrenList.prepend(sourceTreeItemAddress);

    IFOKDO(result, queries.renumberSortOrder())

            IFKO(result) {
        queries.rollback();
        emit errorSent(result);
    }
    IFOK(result) {
        queries.commit();
    }
    IFOK(result) {
        emit treeItemMoved(childrenList, targetAddress);
        emit projectModified(sourceTreeItemAddress.projectId);

        if (sourceTreeItemAddress != targetAddress) {
            emit projectModified(targetAddress.projectId);
        }
    }

    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::moveTreeItemUp(const TreeItemAddress &treeItemAddress)
{
    SKRResult result(this);

    PLMSqlQueries queries(treeItemAddress, m_tableName);

    // get treeItem before this :

    QHash<int, QVariant> sortOrderResult;

    result = queries.getValueByIds("l_sort_order",
                                   sortOrderResult,
                                   QString(),
                                   QVariant(),
                                   true);


    QList<int> idList;

    IFOKDO(result, queries.getSortedIds(idList));

    if (idList.first() == treeItemAddress.itemId) {
        result = SKRResult(SKRResult::Critical, this, "first_in_idList_cant_move_up");
    }
    TreeItemAddress targetTreeItemAddress;

    IFOK(result) {
        // find treeItem before with same indent
        TreeItemAddress possibleTargetTreeItemAddress;

        for (int i = idList.indexOf(treeItemAddress.itemId) - 1; i >= 0; --i) {
            possibleTargetTreeItemAddress.set(treeItemAddress.projectId, idList.at(i));

            if (this->getIndent(possibleTargetTreeItemAddress) ==
                    this->getIndent(treeItemAddress)) {
                targetTreeItemAddress = possibleTargetTreeItemAddress;
                break;
            }
        }

        if (possibleTargetTreeItemAddress.isValid()) {
            result = SKRResult(SKRResult::Critical, this, "possibleTargetTreeItemAddress_is_invalid");
        }
        IFOK(result) {
            int targetIndent   = this->getIndent(targetTreeItemAddress);
            int treeItemIndent = this->getIndent(treeItemAddress);

            if (treeItemIndent  != targetIndent) {
                result = SKRResult(SKRResult::Critical, this, "indents_dont_match");
            }
        }
    }
    IFOKDO(result, this->moveTreeItem(treeItemAddress, targetTreeItemAddress))


            IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::moveTreeItemDown(const TreeItemAddress &treeItemAddress)
{
    SKRResult result(this);

    PLMSqlQueries queries(treeItemAddress, m_tableName);

    // get treeItem before this :

    QHash<int, QVariant> sortOrderResult;

    result = queries.getValueByIds("l_sort_order",
                                   sortOrderResult,
                                   QString(),
                                   QVariant(),
                                   true);


    QList<int> idList;

    IFOKDO(result, queries.getSortedIds(idList));

    if (idList.last() == treeItemAddress.itemId) {
        result = SKRResult(SKRResult::Critical, this, "last_in_idList_cant_move_down");
    }
    TreeItemAddress targetTreeItemAddress;

    IFOK(result) {
        // find treeItem after with same indent
        TreeItemAddress possibleTargetTreeItemAddress;

        for (int i = idList.indexOf(treeItemAddress.itemId) + 1; i < idList.count(); ++i) {
            possibleTargetTreeItemAddress.set(treeItemAddress.projectId, idList.at(i));

            if (this->getIndent(possibleTargetTreeItemAddress) ==
                    this->getIndent(treeItemAddress)) {
                targetTreeItemAddress = possibleTargetTreeItemAddress;
                break;
            }
        }

        if (possibleTargetTreeItemAddress.isValid()) {
            result = SKRResult(SKRResult::Critical, this, "possibleTargetTreeItemAddress_is_invalid");
        }
        IFOK(result) {
            int targetIndent   = this->getIndent(targetTreeItemAddress);
            int treeItemIndent = this->getIndent(treeItemAddress);

            if (treeItemIndent  != targetIndent) {
                result = SKRResult(SKRResult::Critical, this, "indents_dont_match");
            }
        }
    }
    IFOKDO(result, this->moveTreeItem(treeItemAddress, targetTreeItemAddress, true))


            IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::moveTreeItemAsChildOf(const TreeItemAddress &sourceTreeItemAddress,
                                            const TreeItemAddress &targetTreeItemAddress,
                                            bool sendSignal,
                                            int  wantedSortOrder)
{
    SKRResult result(this);


    QList<TreeItemAddress> childrenList  = this->getAllChildren(sourceTreeItemAddress);
    int originalSourceIndent =  this->getIndent(sourceTreeItemAddress);

    if (sourceTreeItemAddress.projectId == targetTreeItemAddress.projectId) {
        int validSortOrder = getValidSortOrderAfterTree(targetTreeItemAddress);


        if (wantedSortOrder == -1) {
            wantedSortOrder = validSortOrder;
        }

        if (wantedSortOrder > validSortOrder) {
            result = SKRResult(SKRResult::Critical, this, "wantedSortOrder_is_outside_scope_of_parent");
        }
        IFOK(result) {
            result = this->setSortOrder(sourceTreeItemAddress, wantedSortOrder);
        }
        IFOK(result) {
            int parentIndent = this->getIndent(targetTreeItemAddress);

            result = this->setIndent(sourceTreeItemAddress, parentIndent + 1);

            int i = 0;

            for (const TreeItemAddress &childTreeItemAddress : qAsConst(childrenList)) {
                result = this->setSortOrder(childTreeItemAddress, wantedSortOrder + i);
                i++;

                int orignalSourceChildIndent = this->getIndent(childTreeItemAddress);
                int delta                    = orignalSourceChildIndent - originalSourceIndent;

                result = this->setIndent(childTreeItemAddress, parentIndent + 1 + delta);
            }
        }
    }
    else {
        // TODO: move between different projects
    }

    IFKO(result) {
        emit errorSent(result);
    }
    IFOK(result) {
        this->renumberSortOrders(sourceTreeItemAddress.projectId);
        emit treeItemMoved(QList<TreeItemAddress>() << sourceTreeItemAddress << childrenList,
                           targetTreeItemAddress);
        emit projectModified(sourceTreeItemAddress.projectId);

        if (sourceTreeItemAddress.projectId != targetTreeItemAddress.projectId) {
            this->renumberSortOrders(targetTreeItemAddress.projectId);
            emit projectModified(targetTreeItemAddress.projectId);
        }
    }


    return result;
}

// ----------------------------------------------------------------------------------------

TreeItemAddress SKRTreeHub::getParentId(const TreeItemAddress &treeItemAddress) const
{
    TreeItemAddress parentId;

    // get indents
    QHash<TreeItemAddress, int> indentList = getAllIndents(treeItemAddress.projectId);
    QList<TreeItemAddress> sortedIdList    = getAllIds(treeItemAddress.projectId);


    // determine direct ancestor

    int indent = indentList.value(treeItemAddress);

    if (indent == 0) {
        return TreeItemAddress();
    }

    for (int i = sortedIdList.indexOf(treeItemAddress); i >= 0; i--) {
        TreeItemAddress id = sortedIdList.at(i);

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

int SKRTreeHub::getValidSortOrderBeforeTree(const TreeItemAddress &treeItemAddress) const
{
    int target_sort_order = getSortOrder(treeItemAddress);

    int finalSortOrder = target_sort_order - 1;

    return finalSortOrder;
}

// ----------------------------------------------------------------------------------------

int SKRTreeHub::getValidSortOrderAfterTree(const TreeItemAddress &treeItemAddress) const
{
    int target_sort_order = getSortOrder(treeItemAddress);
    int target_indent     = getIndent(treeItemAddress);

    // find next node with the same indentation
    QHash<int, QVariant> hash;
    QHash<QString, QVariant> where;

    where.insert("l_indent <=",    target_indent);
    where.insert("l_sort_order >", target_sort_order);
    PLMSqlQueries queries(treeItemAddress, m_tableName);
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

        finalSortOrder = lowestSort - 999;

        // if tree is empty
        if (finalSortOrder == -999) {
            finalSortOrder = 1;
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

SKRResult SKRTreeHub::sortAlphabetically(const TreeItemAddress &parentTreeItemAddress)
{
    SKRResult result(this);

    const QList<TreeItemAddress> &directChildren = this->getAllDirectChildren(parentTreeItemAddress, true, true);
    const QList<TreeItemAddress> &allIds         = this->getAllIds(parentTreeItemAddress.projectId);


    QStringList titleList;

    if (directChildren.isEmpty()) {
        return result;
    }

    for (const TreeItemAddress &directChildTreeItemAddress : qAsConst(directChildren)) {
        titleList << this->getTitle(directChildTreeItemAddress);
    }

    QCollator collator;
    collator.setCaseSensitivity(Qt::CaseInsensitive);
    collator.setNumericMode(true);
    std::sort(titleList.begin(), titleList.end(), collator);


    QMultiHash<QString, TreeItemAddress> allTitlesWithIds;

    for (const TreeItemAddress &directChildTreeItemAddress : qAsConst(directChildren)) {
        allTitlesWithIds.insert(this->getTitle(directChildTreeItemAddress), directChildTreeItemAddress);
    }

    QList<TreeItemAddress> newOrderedDirectChildrenList;

    for (const QString& title : qAsConst(titleList)) {
        QMultiHash<QString, TreeItemAddress>::iterator i = allTitlesWithIds.begin();

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


    QList<TreeItemAddress> newOrderedIdListPlusChildren;

    bool parentPassed = false;

    for (const TreeItemAddress &treeItemAddress : qAsConst(newOrderedDirectChildrenList)) {
        QList<TreeItemAddress> children = skrdata->treeHub()->getAllChildren(treeItemAddress);

        newOrderedIdListPlusChildren.append(treeItemAddress);
        newOrderedIdListPlusChildren.append(children);
    }


    int newSortOrder = this->getSortOrder(parentTreeItemAddress) + 1;

    for (const TreeItemAddress &treeItemAddress : qAsConst(newOrderedIdListPlusChildren)) {
        IFOKDO(result, this->setSortOrder(treeItemAddress, newSortOrder, true, false));
        newSortOrder += 1;
    }


    IFOKDO(result, this->renumberSortOrders(parentTreeItemAddress.projectId));
    PLMSqlQueries queries(parentTreeItemAddress, m_tableName);

    IFKO(result) {
        queries.rollback();
        emit errorSent(result);
    }
    IFOK(result) {
        queries.commit();

        for (const TreeItemAddress &treeItemAddress : qAsConst(newOrderedDirectChildrenList)) {
            emit sortOrderChanged(parentTreeItemAddress, this->getSortOrder(treeItemAddress));
        }
        emit projectModified(parentTreeItemAddress.projectId);
    }

    return result;
}

// ----------------------------------------------------------------------------------------

QList<TreeItemAddress>SKRTreeHub::getAllChildren(const TreeItemAddress &treeItemAddress) const
{
    QList<TreeItemAddress> childrenList;

    // get indents
    const QHash<TreeItemAddress, int> &indentList = getAllIndents(treeItemAddress.projectId);
    const QList<TreeItemAddress> &sortedIdList    = getAllIds(treeItemAddress.projectId);


    // determine children

    int  parentIndent = indentList.value(treeItemAddress);
    bool parentPassed = false;

    for (const TreeItemAddress &id : qAsConst(sortedIdList)) {
        if (id == treeItemAddress) {
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

QList<TreeItemAddress>SKRTreeHub::getAllDirectChildren(const TreeItemAddress &treeItemAddress,
                                           bool trashedAreIncluded,
                                           bool notTrashedAreIncluded) const
{



    QList<TreeItemAddress> childrenList;

    // get indents
    const QHash<TreeItemAddress, int> &indentList = getAllIndents(treeItemAddress.projectId);
    const QList<TreeItemAddress> &sortedIdList    = getAllIds(treeItemAddress.projectId);


    // determine children

    int  parentIndent = indentList.value(treeItemAddress);
    bool parentPassed = false;

    for (const TreeItemAddress &id : qAsConst(sortedIdList)) {
        if (id == treeItemAddress) {
            parentPassed = true;
            continue;
        }

        if (parentPassed) {
            int idIndent = indentList.value(id);

            if (idIndent == parentIndent + 1) {
                bool isTrashed = this->getTrashed(id);

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

QList<TreeItemAddress>SKRTreeHub::getAllAncestors(const TreeItemAddress &treeItemAddress) const
{
    QList<TreeItemAddress> ancestorsList;

    // get indents
    const QHash<TreeItemAddress, int> &indentList = getAllIndents(treeItemAddress.projectId);
    const QList<TreeItemAddress> &sortedIdList    = getAllIds(treeItemAddress.projectId);

    // determine ancestors

    int indent = indentList.value(treeItemAddress);


    for (int i = sortedIdList.indexOf(treeItemAddress); i >= 0; i--) {
        const TreeItemAddress &id = sortedIdList.at(i);

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

QList<TreeItemAddress>SKRTreeHub::getAllSiblings(const TreeItemAddress &treeItemAddress, bool treeItemIncluded)
{
    QList<TreeItemAddress> siblingsList;

    // get indents
    const QHash<TreeItemAddress, int> &indentList = getAllIndents(treeItemAddress.projectId);
    const QList<TreeItemAddress> &sortedIdList    = getAllIds(treeItemAddress.projectId);
    int treeItemSortedIdIndex  = sortedIdList.indexOf(treeItemAddress);


    // determine siblings

    int indent = indentList.value(treeItemAddress);

    // min sibling index
    int minSiblingIndex = treeItemSortedIdIndex;

    for (int i = treeItemSortedIdIndex; i >= 0; i--) {
        const TreeItemAddress &id = sortedIdList.at(i);

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
        const TreeItemAddress &id = sortedIdList.at(i);

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
        const TreeItemAddress &id = sortedIdList.at(i);

        //        if (id == treeItemId) {
        //            continue;
        //        }

        int idIndent = indentList.value(id);

        if (idIndent == indent) {
            siblingsList.append(id);
        }
    }

    if (!treeItemIncluded) {
        siblingsList.removeAll(treeItemAddress);
    }


    return siblingsList;
}

// ----------------------------------------------------------------------------------------

QDateTime SKRTreeHub::getTrashedDate(const TreeItemAddress &treeItemAddress) const
{
    return get(treeItemAddress, "dt_trashed").toDateTime();
}

// ----------------------------------------------------------------------------------------

QList<TreeItemAddress>SKRTreeHub::getTreeRelationshipSourcesFromReceiverId(const TreeItemAddress &receiverTreeItemAddress) const
{
    SKRResult  result;
    QList<TreeItemAddress> list;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(receiverTreeItemAddress, "tbl_tree_relationship");

    result = queries.getValueByIds("l_tree_source_code", out, "l_tree_receiver_code", receiverTreeItemAddress.itemId);

    IFOK(result) {
        for(int i : HashIntQVariantConverter::convertToIntInt(out).values()){
            list << TreeItemAddress(receiverTreeItemAddress.projectId, i);
        }

    }
    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// ----------------------------------------------------------------------------------------

QList<TreeItemAddress>SKRTreeHub::getTreeRelationshipReceiversFromSourceId(const TreeItemAddress &sourceTreeItemAddress) const
{
    SKRResult  result;
    QList<TreeItemAddress> list;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(sourceTreeItemAddress, "tbl_tree_relationship");

    result = queries.getValueByIds("l_tree_receiver_code", out, "l_tree_source_code", sourceTreeItemAddress.itemId);

    IFOK(result) {
        for(int i : HashIntQVariantConverter::convertToIntInt(out).values()){

            list << TreeItemAddress(sourceTreeItemAddress.projectId, i);
        }
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setTreeRelationship(const TreeItemAddress &sourceTreeItemAddress,
                                          const TreeItemAddress &receiverTreeItemAddress)
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_tree_receiver_code", receiverTreeItemAddress.itemId);
    where.insert("l_tree_source_code",   sourceTreeItemAddress.itemId);

    PLMSqlQueries queries(sourceTreeItemAddress, "tbl_tree_relationship");


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
            values.insert("l_tree_receiver_code", receiverTreeItemAddress.itemId);
            values.insert("l_tree_source_code",   sourceTreeItemAddress.itemId);
            result = queries.add(values, newId);

            IFOK(result) {
                emit treeRelationshipAdded(sourceTreeItemAddress, receiverTreeItemAddress);
                emit projectModified(sourceTreeItemAddress.projectId);
            }
        }
    }
    return result;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::removeTreeRelationship(const TreeItemAddress &sourceTreeItemAddress,
                                             const TreeItemAddress &receiverTreeItemAddress)
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;

    QHash<QString, QVariant> where;

    where.insert("l_tree_receiver_code", receiverTreeItemAddress.itemId);

    PLMSqlQueries queries(sourceTreeItemAddress, "tbl_tree_relationship");

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
        emit treeRelationshipRemoved(sourceTreeItemAddress, receiverTreeItemAddress);
        emit projectModified(sourceTreeItemAddress.projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }

    return result;
}



// ----------------------------------------------------------------------------------------

TreeItemAddress SKRTreeHub::getPreviousTreeItemIdOfTheSameType(const TreeItemAddress &treeItemAddress) const
{
    SKRResult result(this);
    TreeItemAddress previousTreeItemId;

    QList<TreeItemAddress> allIds = this->getAllIds(treeItemAddress.projectId);

    int thisItemIndex    = allIds.indexOf(treeItemAddress);
    QString thisItemType = this->getType(treeItemAddress);

    if (thisItemIndex == -1) {
        result = SKRResult(SKRResult::Critical, this, "no_index_found");
    }

    if (thisItemIndex <= 1) {
        return TreeItemAddress();
    }

    IFOK(result) {
        for (int i = thisItemIndex - 1; i >= 0; i--) {
            TreeItemAddress potentiallyPreviousItemId = allIds.at(i);

            if (!this->getTrashed(potentiallyPreviousItemId)) {
                QString previousItemType = this->getType(potentiallyPreviousItemId);

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

SKRResult SKRTreeHub::duplicateTreeItem(const TreeItemAddress &treeItemAddress, bool duplicateChildren, bool renumber)
{
    SKRResult result(this);
    QList<TreeItemAddress> resultTreeItemIdList;

    QHash<QString, QVariant> values;

    int validSortOrder = getValidSortOrderAfterTree(treeItemAddress);

    values.insert("t_title",             getTitle(treeItemAddress));
    values.insert("l_indent",            getIndent(treeItemAddress));
    values.insert("l_sort_order",        validSortOrder);
    values.insert("t_type",              getType(treeItemAddress));
    values.insert("m_primary_content",   getPrimaryContent(treeItemAddress));
    values.insert("m_secondary_content", getSecondaryContent(treeItemAddress));

    result = this->addTreeItem(values, treeItemAddress.projectId, false);

    TreeItemAddress newTreeItemId =  result.getData("treeItemAddress", QVariant::fromValue(TreeItemAddress())).value<TreeItemAddress>();
    resultTreeItemIdList << newTreeItemId;

    if(duplicateChildren){
        QList<TreeItemAddress> childrenList = getAllChildren(treeItemAddress);
        QList<TreeItemAddress> childrenNotTrashed;
        for(const TreeItemAddress &chilTreeItemAddress: childrenList){
            if(!getTrashed(chilTreeItemAddress)){
                childrenNotTrashed.append(chilTreeItemAddress);
            }
        }
        childrenList = childrenNotTrashed;

        int sortOrderOffset = 1;
        for(const TreeItemAddress &chilTreeItemAddress : childrenList){

            QHash<QString, QVariant> childValues;

            childValues.insert("t_title",             getTitle(chilTreeItemAddress));
            childValues.insert("l_indent",            getIndent(chilTreeItemAddress));
            childValues.insert("l_sort_order",        validSortOrder + sortOrderOffset);
            childValues.insert("t_type",              getType(chilTreeItemAddress));
            childValues.insert("m_primary_content",   getPrimaryContent(chilTreeItemAddress));
            childValues.insert("m_secondary_content", getSecondaryContent(chilTreeItemAddress));
            result = this->addTreeItem(childValues, chilTreeItemAddress.projectId, false);

            resultTreeItemIdList << result.getData("treeItemAddress", QVariant::fromValue(TreeItemAddress())).value<TreeItemAddress>();

            sortOrderOffset += 1;

        }
    }
    if(renumber){
        IFOKDO(result, renumberSortOrders(treeItemAddress.projectId))
    }
    IFOK(result) {
        result.addData("treeItemAddressList", QVariant::fromValue<QList<TreeItemAddress> >(resultTreeItemIdList));
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ----------------------------------------------------------------------------------------

void SKRTreeHub::cut(QList<TreeItemAddress> treeItemAddresses)
{
    if(treeItemAddresses.isEmpty()){
        return;
    }

    // unset old treeItems
    m_cutCopy.hasRunOnce = true;

    for (const TreeItemAddress &treeItemAddress : qAsConst(m_cutCopy.treeItemAddresses)) {
        emit cutCopyChanged(treeItemAddress, false);
    }

    // set new treeItems
    m_cutCopy = CutCopy(CutCopy::Cut, treeItemAddresses);

    for (const TreeItemAddress &treeItemAddress : qAsConst(m_cutCopy.treeItemAddresses)) {
        emit cutCopyChanged(treeItemAddress, true);
    }
}

// ----------------------------------------------------------------------------------------

void SKRTreeHub::copy(QList<TreeItemAddress> treeItemAddresses)
{
    if(treeItemAddresses.isEmpty()){
        return;
    }
    // unset old treeItems
    m_cutCopy.hasRunOnce = true;

    for (const TreeItemAddress &treeItemAddress : qAsConst(m_cutCopy.treeItemAddresses)) {
        emit cutCopyChanged(treeItemAddress, false);
    }

    // set new treeItems
    m_cutCopy = CutCopy(CutCopy::Copy, treeItemAddresses);

    for (const TreeItemAddress &treeItemAddress :  qAsConst(m_cutCopy.treeItemAddresses)) {
        emit cutCopyChanged(treeItemAddress, true);
    }
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::paste(const TreeItemAddress &parentTreeItemAddress , bool copyChildren)
{
    SKRResult  result(this);


    if(m_cutCopy.treeItemAddresses.isEmpty()){
        result = SKRResult(SKRResult::Warning ,this ,"No cut or copied tree items");
        return result;
    }

    QList<TreeItemAddress> treeItemIdList;
    QList<TreeItemAddress> originalTreeItemIdList = m_cutCopy.treeItemAddresses;

    if (m_cutCopy.type != CutCopy::Type::None) {
        if (m_cutCopy.type == CutCopy::Type::Cut) {
            if (parentTreeItemAddress.projectId == m_cutCopy.treeItemAddresses.first().projectId) {
                for (const TreeItemAddress &sourceTreeItemAddress : qAsConst(originalTreeItemIdList)) {
                    result = this->moveTreeItemAsChildOf(sourceTreeItemAddress,
                                                         parentTreeItemAddress,
                                                         false);
                    result = this->renumberSortOrders(parentTreeItemAddress.projectId);
                    treeItemIdList << sourceTreeItemAddress;
                }
            }

            // TODO: case if projects are different

            // become a Copy after first paste
            m_cutCopy.treeItemAddresses = treeItemIdList;
            m_cutCopy.type        = CutCopy::Type::Copy;
        }
        else if (m_cutCopy.type == CutCopy::Type::Copy) {
            for (const TreeItemAddress &sourceTreeItemAddress : qAsConst(originalTreeItemIdList)) {
                if (parentTreeItemAddress.projectId == m_cutCopy.treeItemAddresses.first().projectId) {
                    result = this->duplicateTreeItem(sourceTreeItemAddress, copyChildren, false);
                    QList<TreeItemAddress> newTreeItemIdList = result.getData("treeItemAddressList",
                                                                  QVariant::fromValue<QList<TreeItemAddress> >(QList<TreeItemAddress>())).value<QList<TreeItemAddress> >();
                    treeItemIdList << newTreeItemIdList.first();
                    IFOKDO(result,
                           this->moveTreeItemAsChildOf(newTreeItemIdList.first(), parentTreeItemAddress,
                                                       false))
                            result = this->renumberSortOrders(parentTreeItemAddress.projectId);
                }

                // TODO: case if projects are different
            }
        }
    }
    IFOK(result) {
        result.addData("treeItemAddressList", QVariant::fromValue<QList<TreeItemAddress>> (treeItemIdList));

        // unset old treeItems
        m_cutCopy.hasRunOnce = true;

        for (const TreeItemAddress &treeItemAddress : qAsConst(originalTreeItemIdList)) {
            emit cutCopyChanged(treeItemAddress, false);
        }
    }
    IFKO(result) {
        emit errorSent(result);
    }

    IFOK(result) {
        if (m_cutCopy.type == CutCopy::Type::Cut) {
            emit treeItemMoved(treeItemIdList, parentTreeItemAddress);

            emit projectModified(m_cutCopy.treeItemAddresses.first().projectId);

            if (m_cutCopy.treeItemAddresses.first().projectId != parentTreeItemAddress.projectId) {
                emit projectModified(parentTreeItemAddress.projectId);
            }
        }
        else if (m_cutCopy.type == CutCopy::Type::Copy) {
            emit treeItemsAdded(treeItemIdList);

            if (m_cutCopy.treeItemAddresses.first().projectId == parentTreeItemAddress.projectId) {
                emit projectModified(m_cutCopy.treeItemAddresses.first().projectId);
            }
            else {
                emit projectModified(parentTreeItemAddress.projectId);
            }
        }
    }

    return result;
}

bool SKRTreeHub::isCutCopy(const TreeItemAddress &treeItemAddress) const {


    if(m_cutCopy.treeItemAddresses.isEmpty()){
        return false;
    }

    if ((m_cutCopy.treeItemAddresses.first().projectId == treeItemAddress.projectId) && !m_cutCopy.hasRunOnce) {
        return m_cutCopy.treeItemAddresses.contains(treeItemAddress);
    }

    return false;
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setTrashedDateToNow(const TreeItemAddress &treeItemAddress, const QDateTime &forcedDate)
{
    QDateTime date;
    if(forcedDate.isValid()){
        date = forcedDate;
    }
    else {
        date = QDateTime::currentDateTime();
    }

    return set(treeItemAddress, "dt_trashed", date);
}

// ----------------------------------------------------------------------------------------

SKRResult SKRTreeHub::setTrashedDateToNull(const TreeItemAddress &treeItemAddress)
{
    return set(treeItemAddress, "dt_trashed", "NULL");
}

// ----------------------------------------------------------------------------------------


void SKRTreeHub::setError(const SKRResult& result)
{
    m_error = result;
}

// ----------------------------------------------------------------------------------------

/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrtreehub.h                                                   *
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
#ifndef SKRTREEHUB_H
#define SKRTREEHUB_H

#include <QObject>
#include <QString>
#include <QHash>
#include <QList>
#include <QVariant>
#include <QDateTime>

#include "skribisto_data_global.h"
#include "skrresult.h"
#include "skrpropertyhub.h"
#include "treeitemaddress.h"

struct CutCopy
{
    Q_GADGET

public:

    enum Type {
        Cut,
        Copy,
        None
    };
    Q_ENUM(Type)

    explicit CutCopy(CutCopy::Type type, QList<TreeItemAddress> treeItemAddresses) {
        this->type        = type;
        this->treeItemAddresses = treeItemAddresses;
        this->hasRunOnce  = false;
    }

    CutCopy() {
        this->type       = Type::None;
        this->hasRunOnce = false;
    }

    CutCopy::Type type;
    QList<TreeItemAddress>    treeItemAddresses;
    bool          hasRunOnce;
};
Q_DECLARE_METATYPE(CutCopy)


class EXPORT SKRTreeHub : public QObject {
    Q_OBJECT

public:

    explicit SKRTreeHub(QObject *parent = nullptr);
    QHash<TreeItemAddress, int> getAllSortOrders(int projectId) const;
    QHash<TreeItemAddress, int> getAllIndents(int projectId) const;
    Q_INVOKABLE QList<TreeItemAddress>getAllIds(int projectId) const;
    Q_INVOKABLE QList<TreeItemAddress>getAllTrashedIds(int projectId) const;
    QList<QVariantMap> saveTree(int projectId) const;
    SKRResult restoreTree(int projectId, QList<QVariantMap> allValues);
    Q_INVOKABLE QVariantMap saveId(const TreeItemAddress &treeItemAddress) const;
    Q_INVOKABLE SKRResult restoreId(const TreeItemAddress &treeItemAddress, const QVariantMap &values);

    Q_INVOKABLE SKRResult setTreeItemId(const TreeItemAddress &treeItemAddress,
                                   int newId);

    Q_INVOKABLE SKRResult setTitle(const TreeItemAddress &treeItemAddress,
                                   const QString& newTitle);
    Q_INVOKABLE QString   getTitle(const TreeItemAddress &treeItemAddress) const;

    Q_INVOKABLE SKRResult setInternalTitle(const TreeItemAddress &treeItemAddress,
                                           const QString& internalTitle);

    Q_INVOKABLE SKRResult removeInternalTitleFromAll(int            projectId,
                                                     const QString& internalTitle);

    Q_INVOKABLE QList<TreeItemAddress>getIdsWithInternalTitle(int            projectId,
                                                  const QString& internalTitle) const;
    Q_INVOKABLE QString   getInternalTitle(const TreeItemAddress &treeItemAddress) const;

    SKRResult             setIndent(const TreeItemAddress &treeItemAddress,
                                    int newIndent);
    SKRResult             setIndent(const TreeItemAddress &treeItemAddress,
                                    int newIndent, bool setCurrentdate, bool commit);
    Q_INVOKABLE int       getIndent(const TreeItemAddress &treeItemAddress) const;
    SKRResult             setSortOrder(const TreeItemAddress &treeItemAddress,
                                       int newSortOrder);
    SKRResult             setSortOrder(const TreeItemAddress &treeItemAddress,
                                       int  newSortOrder,
                                       bool setCurrentdate,
                                       bool commit);
    int                 getSortOrder(const TreeItemAddress &treeItemAddress) const;
    SKRResult           setType(const TreeItemAddress &treeItemAddress,
                                const QString& newType);
    Q_INVOKABLE QString getType(const TreeItemAddress &treeItemAddress) const;


    Q_INVOKABLE SKRResult setPrimaryContent(const TreeItemAddress &treeItemAddress,
                                            const QString& newContent);
    Q_INVOKABLE SKRResult setPrimaryContent(const TreeItemAddress &treeItemAddress,
                                            const QString& newContent,
                                            bool           setCurrentdate,
                                            bool           commit);
    Q_INVOKABLE QString   getPrimaryContent(const TreeItemAddress &treeItemAddress) const;

    Q_INVOKABLE SKRResult setSecondaryContent(const TreeItemAddress &treeItemAddress,
                                              const QString& newContent);
    Q_INVOKABLE SKRResult setSecondaryContent(const TreeItemAddress &treeItemAddress,
                                              const QString& newContent,
                                              bool           setCurrentdate,
                                              bool           commit);
    Q_INVOKABLE QString   getSecondaryContent(const TreeItemAddress &treeItemAddress) const;

    Q_INVOKABLE SKRResult setTrashedWithChildren(const TreeItemAddress &treeItemAddress,
                                                 bool newTrashedState);
    Q_INVOKABLE SKRResult untrashOnlyOneTreeItem(const TreeItemAddress &treeItemAddress);
    Q_INVOKABLE bool      getTrashed(const TreeItemAddress &treeItemAddress) const;
    SKRResult             setCreationDate(const TreeItemAddress &treeItemAddress,
                                          const QDateTime& newDate);
    QDateTime             getCreationDate(const TreeItemAddress &treeItemAddress) const;
    SKRResult             setUpdateDate(const TreeItemAddress &treeItemAddress,
                                        const QDateTime& newDate);
    QDateTime             getUpdateDate(const TreeItemAddress &treeItemAddress) const;

    int row(const TreeItemAddress &treeItemAddress) const;


    Q_INVOKABLE bool hasChildren(const TreeItemAddress &treeItemAddress,
                                 bool trashedAreIncluded    = false,
                                 bool notTrashedAreIncluded = true) const;

    Q_INVOKABLE TreeItemAddress getTopTreeItemId(int projectId) const;

    QList<TreeItemAddress> filterOutChildren(QList<TreeItemAddress> treeItemAddresses) const;

    SKRResult       getError();
    SKRResult       set(const TreeItemAddress &treeItemAddress,
                        const QString & fieldName,
                        const QVariant& value,
                        bool            setCurrentDateBool = true,
                        bool            commit             = true);
    QVariant get(const TreeItemAddress &treeItemAddress,
                 const QString& fieldName) const;


    Q_INVOKABLE TreeItemAddress       getLastAddedAddress();

    SKRResult             addTreeItem(int projectId, int sortOrder, int indent, const QString &type, const QString &title, const QString &internalTitle, bool renumber = true);
    SKRResult             addTreeItem(const QHash<QString, QVariant>& values,
                                      int                             projectId, bool renumber= true);
    Q_INVOKABLE SKRResult addTreeItemAbove(const TreeItemAddress &targetTreeItemAddress,
                                           const QString& type);
    Q_INVOKABLE SKRResult addTreeItemBelow(const TreeItemAddress &targetTreeItemAddress,
                                           const QString& type);
    Q_INVOKABLE SKRResult addChildTreeItem(const TreeItemAddress &targetTreeItemAddress,
                                           const QString& type);
    Q_INVOKABLE SKRResult removeTreeItem(const TreeItemAddress &treeItemAddress);


    Q_INVOKABLE SKRResult moveTreeItem(const TreeItemAddress &sourceTreeItemAddress,
                                       const TreeItemAddress &targetTreeItemAddress,
                                       bool after = false);

    Q_INVOKABLE SKRResult moveTreeItemUp(const TreeItemAddress &treeItemAddress);
    Q_INVOKABLE SKRResult moveTreeItemDown(const TreeItemAddress &treeItemAddress);
    Q_INVOKABLE SKRResult moveTreeItemAsChildOf(const TreeItemAddress &sourceTreeItemAddress,
                                                const TreeItemAddress &targetTreeItemAddress,
                                                bool sendSignal      = true,
                                                int  wantedSortOrder = -1);
    Q_INVOKABLE TreeItemAddress getParentId(const TreeItemAddress &treeItemAddress) const;


    SKRResult             renumberSortOrders(int projectId);
    int                   getValidSortOrderBeforeTree(const TreeItemAddress &treeItemAddress) const;
    int                   getValidSortOrderAfterTree(const TreeItemAddress &treeItemAddress) const;

    Q_INVOKABLE SKRResult sortAlphabetically(const TreeItemAddress &parentTreeItemAddress);


    Q_INVOKABLE QList<TreeItemAddress>getAllChildren(const TreeItemAddress &treeItemAddress) const;

    Q_INVOKABLE QList<TreeItemAddress>getAllDirectChildren(const TreeItemAddress &treeItemAddress,
                                               bool trashedAreIncluded    = false,
                                               bool notTrashedAreIncluded = true) const;
    Q_INVOKABLE QList<TreeItemAddress>getAllAncestors(const TreeItemAddress &treeItemAddress) const;
    QList<TreeItemAddress>            getAllSiblings(const TreeItemAddress &treeItemAddress,
                                         bool treetemIncluded = false);
    Q_INVOKABLE QDateTime getTrashedDate(const TreeItemAddress &treeItemAddress) const;


    Q_INVOKABLE QList<TreeItemAddress>getTreeRelationshipSourcesFromReceiverId(const TreeItemAddress &receiverTreeItemAddress) const;
    Q_INVOKABLE QList<TreeItemAddress>getTreeRelationshipReceiversFromSourceId(const TreeItemAddress &sourceTreeItemAddress) const;
    Q_INVOKABLE SKRResult setTreeRelationship(const TreeItemAddress &sourceTreeItemAddress,
                                              const TreeItemAddress &receiverTreeItemAddress);
    Q_INVOKABLE SKRResult removeTreeRelationship(const TreeItemAddress &sourceTreeItemAddress,
                                                 const TreeItemAddress &receiverTreeItemAddress);

    Q_INVOKABLE TreeItemAddress       getPreviousTreeItemIdOfTheSameType(const TreeItemAddress &treeItemAddress) const;

    Q_INVOKABLE SKRResult duplicateTreeItem(const TreeItemAddress &treeItemAddress,
                                            bool duplicateChildren=true, bool renumber=true);
    Q_INVOKABLE void      cut(QList<TreeItemAddress> treeItemAddresses);
    Q_INVOKABLE void      copy(QList<TreeItemAddress> treeItemAddresses);
    Q_INVOKABLE SKRResult paste(const TreeItemAddress &parentTreeItemAddress,
                                bool copyChildren=true);

    Q_INVOKABLE bool isCutCopy(const TreeItemAddress &treeItemAddress) const;

private:
    void commit(int projectId);

    SKRResult setTrashedDateToNow(const TreeItemAddress &treeItemAddress, const QDateTime &forcedDate = QDateTime());
    SKRResult setTrashedDateToNull(const TreeItemAddress &treeItemAddress);

private slots:

    void setError(const SKRResult& result);

signals:

    void errorSent(const SKRResult& result) const;
    void projectModified(int projectId); // for save
    void treeReset(int projectId); // for save
    void allValuesChanged(const TreeItemAddress &treeItemAddress);
    void treeItemIdChanged(const TreeItemAddress &treeItemAddress,
                           int newId);
    void titleChanged(const TreeItemAddress &treeItemAddress,
                      const QString& newTitle);
    void internalTitleChanged(const TreeItemAddress &treeItemAddress,
                              const QString& newTitle);
    void indentChanged(const TreeItemAddress &treeItemAddress,
                       int newIndent);
    void sortOrderChanged(const TreeItemAddress &treeItemAddress,
                          int newSortOrder);
    void typeChanged(const TreeItemAddress &treeItemAddress,
                     QString newType);
    void primaryContentChanged(const TreeItemAddress &treeItemAddress,
                               const QString& newContent);
    void secondaryContentChanged(const TreeItemAddress &treeItemAddress,
                                 const QString& newContent);
    void trashedChanged(const TreeItemAddress &treeItemAddress,
                        bool newTrashedState);
    void creationDateChanged(const TreeItemAddress &treeItemAddress,
                             const QDateTime& newDate);
    void updateDateChanged(const TreeItemAddress &treeItemAddress,
                           const QDateTime& newDate);
    void treeItemAdded(const TreeItemAddress &treeItemAddress);
    void treeItemsAdded(QList<TreeItemAddress> treeItemAddresses);
    void treeItemAboutToBeRemoved(const TreeItemAddress &treeItemAddress);
    void treeItemRemoved(const TreeItemAddress &treeItemAddress);
    void treeItemMoved(QList<TreeItemAddress> sourceTreeItemAddresses,
const TreeItemAddress &targetTreeItemAddress);


    void treeRelationshipChanged(const TreeItemAddress &sourceTreeItemAddress,
                                 const TreeItemAddress &receiverTreeItemAddress);
    void treeRelationshipRemoved(const TreeItemAddress &sourceTreeItemAddress,
                                 const TreeItemAddress &receiverTreeItemAddress);
    void treeRelationshipAdded(const TreeItemAddress &sourceTreeItemAddress,
                               const TreeItemAddress &receiverTreeItemAddress);

    void cutCopyChanged(const TreeItemAddress &treeItemAddress,
                        bool value);

protected:
private:
    SKRResult m_error;
    QString m_tableName;
    TreeItemAddress m_last_added_address;
    SKRPropertyHub *m_propertyHub;
    CutCopy m_cutCopy;
    QHash<int, QList<TreeItemAddress>>m_getAllIds_cache;
    QHash<int,QHash<TreeItemAddress, int>> m_getAllSortOrders_cache;
    QHash<int,QHash<TreeItemAddress, int>> m_getAllIndents_cache;

private slots:
    void resetCache(int projectId);
    void resetCacheByAddress(const TreeItemAddress &treeItemAddress);
};

#endif // SKRTREEHUB_H

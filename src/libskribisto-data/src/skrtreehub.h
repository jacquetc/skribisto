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

    explicit CutCopy(CutCopy::Type type, const int projectId, const QList<int>treeItemIds) {
        this->type        = type;
        this->projectId   = projectId;
        this->treeItemIds = treeItemIds;
        this->hasRunOnce  = false;
    }

    CutCopy() {
        this->type = Type::None;
        this->projectId   = -2;
        this->hasRunOnce  = false;
    }

    CutCopy::Type type;
    int           projectId;
    QList<int>    treeItemIds;
    bool          hasRunOnce;
};
Q_DECLARE_METATYPE(CutCopy)


class EXPORT SKRTreeHub : public QObject {
    Q_OBJECT

public:

    explicit SKRTreeHub(QObject *parent = nullptr);
    QHash<int, int>       getAllSortOrders(int projectId) const;
    QHash<int, int>       getAllIndents(int projectId) const;
    Q_INVOKABLE QList<int>getAllIds(int projectId) const;

    Q_INVOKABLE SKRResult setTitle(int            projectId,
                                   int            treeItemId,
                                   const QString& newTitle);
    Q_INVOKABLE QString   getTitle(int projectId,
                                   int treeItemId) const;

    Q_INVOKABLE SKRResult setInternalTitle(int            projectId,
                                           int            treeItemId,
                                           const QString& internalTitle);

    Q_INVOKABLE SKRResult removeInternalTitleFromAll(int            projectId,
                                                     const QString& internalTitle);

    Q_INVOKABLE QList<int>getIdsWithInternalTitle(int            projectId,
                                                  const QString& internalTitle) const;
    Q_INVOKABLE QString   getInternalTitle(int projectId,
                                           int treeItemId) const;

    SKRResult             setIndent(int projectId,
                                    int treeItemId,
                                    int newIndent);
    Q_INVOKABLE int       getIndent(int projectId,
                                    int treeItemId) const;
    SKRResult             setSortOrder(int projectId,
                                       int treeItemId,
                                       int newSortOrder);
    SKRResult             setSortOrder(int  projectId,
                                       int  treeItemId,
                                       int  newSortOrder,
                                       bool setCurrentdate,
                                       bool commit);
    int                 getSortOrder(int projectId,
                                     int treeItemId) const;
    SKRResult           setType(int            projectId,
                                int            treeItemId,
                                const QString& newType);
    Q_INVOKABLE QString getType(int projectId,
                                int treeItemId) const;


    Q_INVOKABLE SKRResult setPrimaryContent(int            projectId,
                                            int            treeItemId,
                                            const QString& newContent);
    Q_INVOKABLE SKRResult setPrimaryContent(int            projectId,
                                            int            treeItemId,
                                            const QString& newContent,
                                            bool           setCurrentdate,
                                            bool           commit);
    Q_INVOKABLE QString   getPrimaryContent(int projectId,
                                            int treeItemId) const;

    Q_INVOKABLE SKRResult setSecondaryContent(int            projectId,
                                              int            treeItemId,
                                              const QString& newContent);
    Q_INVOKABLE SKRResult setSecondaryContent(int            projectId,
                                              int            treeItemId,
                                              const QString& newContent,
                                              bool           setCurrentdate,
                                              bool           commit);
    Q_INVOKABLE QString   getSecondaryContent(int projectId,
                                              int treeItemId) const;

    Q_INVOKABLE SKRResult setTrashedWithChildren(int  projectId,
                                                 int  treeItemId,
                                                 bool newTrashedState);
    Q_INVOKABLE SKRResult untrashOnlyOneTreeItem(int projectId,
                                                 int treeItemId);
    Q_INVOKABLE bool      getTrashed(int projectId,
                                     int treeItemId) const;
    SKRResult             setCreationDate(int              projectId,
                                          int              treeItemId,
                                          const QDateTime& newDate);
    QDateTime             getCreationDate(int projectId,
                                          int treeItemId) const;
    SKRResult             setUpdateDate(int              projectId,
                                        int              treeItemId,
                                        const QDateTime& newDate);
    QDateTime             getUpdateDate(int projectId,
                                        int treeItemId) const;


    Q_INVOKABLE bool hasChildren(int  projectId,
                                 int  treeItemId,
                                 bool trashedAreIncluded    = false,
                                 bool notTrashedAreIncluded = true) const;

    Q_INVOKABLE int getTopTreeItemId(int projectId) const;

    SKRResult       getError();
    SKRResult       set(int             projectId,
                        int             treeItemId,
                        const QString & fieldName,
                        const QVariant& value,
                        bool            setCurrentDateBool = true,
                        bool            commit             = true);
    QVariant get(int            projectId,
                 int            treeItemId,
                 const QString& fieldName) const;

    Q_INVOKABLE int       getLastAddedId();

    SKRResult             addTreeItem(const QHash<QString, QVariant>& values,
                                      int                             projectId);
    Q_INVOKABLE SKRResult addTreeItemAbove(int            projectId,
                                           int            targetId,
                                           const QString& type);
    Q_INVOKABLE SKRResult addTreeItemBelow(int            projectId,
                                           int            targetId,
                                           const QString& type);
    Q_INVOKABLE SKRResult addChildTreeItem(int            projectId,
                                           int            targetId,
                                           const QString& type);
    Q_INVOKABLE SKRResult removeTreeItem(int projectId,
                                         int targetId);


    Q_INVOKABLE SKRResult moveTreeItem(int  sourceProjectId,
                                       int  sourceTreeItemId,
                                       int  targetTreeItemId,
                                       bool after = false);

    Q_INVOKABLE SKRResult moveTreeItemUp(int projectId,
                                         int treeItemId);
    Q_INVOKABLE SKRResult moveTreeItemDown(int projectId,
                                           int treeItemId);
    Q_INVOKABLE SKRResult moveTreeItemAsChildOf(int projectId,
                                                int noteId,
                                                int targetParentId,
                                                int wantedSortOrder = -1);
    Q_INVOKABLE int getParentId(int projectId,
                                int treeItemId);


    SKRResult             renumberSortOrders(int projectId);
    int                   getValidSortOrderBeforeTree(int projectId,
                                                      int treeItemId) const;
    int                   getValidSortOrderAfterTree(int projectId,
                                                     int treeItemId) const;

    Q_INVOKABLE SKRResult sortAlphabetically(int projectId,
                                             int parentTreeItemId);


    Q_INVOKABLE QList<int>getAllChildren(int projectId,
                                         int treeItemId);

    Q_INVOKABLE QList<int>getAllDirectChildren(int  projectId,
                                               int  treeItemId,
                                               bool trashedAreIncluded    = false,
                                               bool notTrashedAreIncluded = true);
    Q_INVOKABLE QList<int>getAllAncestors(int projectId,
                                          int treeItemId);
    QList<int>            getAllSiblings(int  projectId,
                                         int  treeItemId,
                                         bool treetemIncluded = false);
    Q_INVOKABLE QDateTime getTrashedDate(int projectId,
                                         int treeItemId) const;


    Q_INVOKABLE QList<int>getTreeRelationshipSourcesFromReceiverId(int projectId,
                                                                   int receiverTreeItemId) const;
    Q_INVOKABLE QList<int>getTreeRelationshipReceiversFromSourceId(int projectId,
                                                                   int sourceTreeItemId) const;
    Q_INVOKABLE SKRResult setTreeRelationship(int projectId,
                                              int sourceTreeItemId,
                                              int receiverTreeItemId);
    Q_INVOKABLE SKRResult removeTreeRelationship(int projectId,
                                                 int sourceTreeItemId,
                                                 int receiverTreeItemId);

    Q_INVOKABLE int       getPreviousTreeItemIdOfTheSameType(int projectId,
                                                             int treeItemId);

    Q_INVOKABLE SKRResult duplicateTreeItem(int projectId,
                                            int treeItemId);
    Q_INVOKABLE void      cut(int       projectId,
                              QList<int>treeItemIds);
    Q_INVOKABLE void      copy(int       projectId,
                               QList<int>treeItemIds);
    Q_INVOKABLE SKRResult paste(int targetProjectId,
                                int parentTreeItemId);

    Q_INVOKABLE SKRResult addQuickNote(int            projectId,
                                       int            receiverTreeItemId,
                                       const QString& type,
                                       const QString& noteName);

    Q_INVOKABLE bool isCutCopy(int projectId, int treeItemId) const;
private:

    SKRResult setTrashedDateToNow(int projectId,
                                  int treeItemId);
    SKRResult setTrashedDateToNull(int projectId,
                                   int treeItemId);

private slots:

    void setError(const SKRResult& result);

signals:

    void errorSent(const SKRResult& result) const;
    void projectModified(int projectId); // for save
    void treeItemIdChanged(int projectId,
                           int treeItemId,
                           int newId);
    void titleChanged(int            projectId,
                      int            treeItemId,
                      const QString& newTitle);
    void internalTitleChanged(int            projectId,
                              int            treeItemId,
                              const QString& newTitle);
    void indentChanged(int projectId,
                       int treeItemId,
                       int newIndent);
    void sortOrderChanged(int projectId,
                          int treeItemId,
                          int newSortOrder);
    void typeChanged(int     projectId,
                     int     treeItemId,
                     QString newType);
    void primaryContentChanged(int            projectId,
                               int            treeItemId,
                               const QString& newContent);
    void secondaryContentChanged(int            projectId,
                                 int            treeItemId,
                                 const QString& newContent);
    void trashedChanged(int  projectId,
                        int  treeItemId,
                        bool newTrashedState);
    void creationDateChanged(int              projectId,
                             int              treeItemId,
                             const QDateTime& newDate);
    void updateDateChanged(int              projectId,
                           int              treeItemId,
                           const QDateTime& newDate);
    void treeItemAdded(int projectId,
                       int treeItemId);
    void treeItemsAdded(int projectId,
                       QList<int> treeItemIds);
    void treeItemRemoved(int projectId,
                         int treeItemId);
    void treeItemMoved(int       sourceProjectId,
                       QList<int>sourceTreeItemIds,
                       int       targetProjectId,
                       int       targetTreeItemId);


    void treeRelationshipChanged(int projectId,
                                 int sourceTreeItemId,
                                 int receiverTreeItemId);
    void treeRelationshipRemoved(int projectId,
                                 int sourceTreeItemId,
                                 int receiverTreeItemId);
    void treeRelationshipAdded(int projectId,
                               int sourceTreeItemId,
                               int receiverTreeItemId);

    void cutCopyChanged(int projectId,
                        int treeItemId,
                        bool value);

protected:

    SKRResult m_error;
    QString m_tableName;
    int m_last_added_id;
    SKRPropertyHub *m_propertyHub;
    CutCopy m_cutCopy;
};

#endif // SKRTREEHUB_H

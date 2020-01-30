/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmsheetlistproxymodel.h
*                                                  *
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
#ifndef PLMSHEETLISTPROXYMODEL_H
#define PLMSHEETLISTPROXYMODEL_H

#include <QSortFilterProxyModel>
#include "plmsheetitem.h"
#include "./skribisto_data_global.h"

class EXPORT PLMSheetListProxyModel : public QSortFilterProxyModel {
    Q_OBJECT
    Q_PROPERTY(int projectIdFilter MEMBER m_projectIdFilter WRITE setProjectIdFilter NOTIFY projectIdFilterChanged)
    Q_PROPERTY(int parentIdFilter MEMBER m_parentIdFilter WRITE setParentIdFilter NOTIFY parentIdFilterChanged)
    Q_PROPERTY(bool showDeletedFilter MEMBER m_showDeletedFilter WRITE setShowDeletedFilter NOTIFY showDeletedFilterChanged)
    Q_PROPERTY(int forcedCurrentIndex MEMBER m_forcedCurrentIndex WRITE setForcedCurrentIndex NOTIFY forcedCurrentIndexChanged)

public:

    explicit PLMSheetListProxyModel(QObject *parent = nullptr);

    Qt::ItemFlags flags(const QModelIndex& index) const;

    QVariant      data(const QModelIndex& index,
                       int                role) const;
    bool          setData(const QModelIndex& index,
                          const QVariant   & value,
                          int                role);

    Q_INVOKABLE void moveItem(int from, int to);
    Q_INVOKABLE int goUp();
    Q_INVOKABLE int getItemIndent(int projectId, int paperId);
    Q_INVOKABLE QString getItemName(int projectId, int paperId);
    void setProjectIdFilter(int projectIdFilter);
    void setParentIdFilter(int parentIdFilter);
    void clearFilters();

    Q_INVOKABLE void addItemAtEnd(int projectId, int parentPaperId, int visualIndex);
    Q_INVOKABLE void moveUp(int projectId, int paperId, int visualIndex);
    Q_INVOKABLE void moveDown(int projectId, int paperId, int visualIndex);
    Q_INVOKABLE void setForcedCurrentIndex(int forcedCurrentIndex);
    Q_INVOKABLE void setForcedCurrentIndex(int projectId, int paperId);
    Q_INVOKABLE bool hasChildren(int projectId, int paperId);
    Q_INVOKABLE int findVisualIndex(int projectId, int paperId);

    Q_INVOKABLE int getLastOfHistory(int projectId);
    Q_INVOKABLE void removeLastOfHistory(int projectId);
    Q_INVOKABLE void addHistory(int projectId, int paperId);

    Q_INVOKABLE void setCurrentPaperId(int projectId, int paperId);
signals:
    void projectIdFilterChanged(int projectIdFilter);
    void parentIdFilterChanged(int paperIdFilter);
    Q_INVOKABLE void forcedCurrentIndexChanged(int forcedCurrentIndex);
    Q_INVOKABLE void showDeletedFilterChanged(bool showDeleted);


public slots:

    Q_INVOKABLE void setShowDeletedFilter(bool showDeleted);
    Q_INVOKABLE void setParentFilter(int projectId, int parentId);
    Q_INVOKABLE void clearHistory(int projectId);
protected:
    bool filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const;

private:
    PLMSheetItem *getItem(int projectId, int paperId);

private slots:
    void loadProjectSettings(int projectId);
    void saveProjectSettings(int projectId);
private:
    bool m_showDeletedFilter;
    int m_projectIdFilter;
    int m_parentIdFilter;
    int m_forcedCurrentIndex;
    QHash<int,QList<int> > m_historyList;
};

#endif // PLMSHEETLISTPROXYMODEL_H

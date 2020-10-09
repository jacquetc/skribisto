/***************************************************************************
 *   Copyright (C) 2020 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skrsearchtaglistproxymodel.h                                                   *
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
#ifndef SKRSEARCHTAGLISTPROXYMODEL_H
#define SKRSEARCHTAGLISTPROXYMODEL_H

#include <QObject>
#include <QSortFilterProxyModel>
#include "skrtagitem.h"
#include "./skribisto_data_global.h"

class EXPORT SKRSearchTagListProxyModel : public QSortFilterProxyModel
{
    Q_OBJECT
    Q_PROPERTY(int projectIdFilter MEMBER m_projectIdFilter WRITE setProjectIdFilter NOTIFY projectIdFilterChanged)
    Q_PROPERTY(int sheetIdFilter MEMBER m_sheetIdFilter WRITE setSheetIdFilter NOTIFY sheetIdFilterChanged)
    Q_PROPERTY(int noteIdFilter MEMBER m_noteIdFilter WRITE setNoteIdFilter NOTIFY noteIdFilterChanged)
    Q_PROPERTY(QString textFilter MEMBER m_textFilter WRITE setTextFilter NOTIFY textFilterChanged)
    Q_PROPERTY(int forcedCurrentIndex MEMBER m_forcedCurrentIndex WRITE setForcedCurrentIndex NOTIFY forcedCurrentIndexChanged)

public:
    explicit SKRSearchTagListProxyModel(QObject *parent = nullptr);


    Qt::ItemFlags flags(const QModelIndex& index) const;

    QVariant      data(const QModelIndex& index,
                       int                role) const;
    bool          setData(const QModelIndex& index,
                          const QVariant   & value,
                          int                role);

    Q_INVOKABLE QString getItemName(int projectId, int paperId);
    void setProjectIdFilter(int projectIdFilter);
    void clearFilters();

    Q_INVOKABLE void setForcedCurrentIndex(int forcedCurrentIndex);
    Q_INVOKABLE void setForcedCurrentIndex(int projectId, int paperId);
    Q_INVOKABLE int findVisualIndex(int projectId, int paperId);
    Q_INVOKABLE void setCurrentPaperId(int projectId, int paperId);
    void setTextFilter(const QString &value);

    void setSheetIdFilter(int sheetIdFilter);
    void setNoteIdFilter(int noteIdFilter);

signals:
    void projectIdFilterChanged(int projectIdFilter);
    void sheetIdFilterChanged(int sheetIdFilter);
    void noteIdFilterChanged(int noteIdFilter);
    void textFilterChanged(const QString &value);
    Q_INVOKABLE void forcedCurrentIndexChanged(int forcedCurrentIndex);

protected:
    bool filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const;

private:
    SKRTagItem *getItem(int projectId, int tagId);

private slots:
    void loadProjectSettings(int projectId);
    void saveProjectSettings(int projectId);
    void populateRelationshipList();
private:
    QString m_textFilter;
    int m_projectIdFilter, m_sheetIdFilter, m_noteIdFilter;
    int m_forcedCurrentIndex;
    QList<int> m_relationshipList;
};

#endif // SKRSEARCHTAGLISTPROXYMODEL_H

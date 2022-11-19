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
#ifndef SKRTAGHUB_H
#define SKRTAGHUB_H

#include <QObject>
#include "skrresult.h"
#include "skr.h"
#include "skribisto_data_global.h"
#include "treeitemaddress.h"

class EXPORT SKRTagHub : public QObject {
    Q_OBJECT

public:

    explicit SKRTagHub(QObject *parent);
    Q_INVOKABLE QVariantMap saveId(int projectId, int tagId) const;
    Q_INVOKABLE SKRResult restoreId(int projectId, int tagId, const QVariantMap &values);

    Q_INVOKABLE QList<int>getAllTagIds(int projectId) const;
    Q_INVOKABLE SKRResult addTag(int            projectId,
                                 const QString& tagName);
    Q_INVOKABLE SKRResult addTag(int            projectId);
    Q_INVOKABLE SKRResult removeTag(int projectId,
                                    int tagId);
    Q_INVOKABLE int       getTagIdWithName(int            projectId,
                                           const QString& tagName);
    Q_INVOKABLE QString   getTagName(int projectId,
                                     int tagId) const;
    Q_INVOKABLE SKRResult setTagName(int            projectId,
                                     int            tagId,
                                     const QString& tagName);
    Q_INVOKABLE SKRResult setTagId(int            projectId,
                                     int            tagId,
                                     int            newTagId);
    Q_INVOKABLE bool      doesTagNameAlreadyExist(int            projectId,
                                                  const QString& tagName);
    Q_INVOKABLE QString   getTagColor(int projectId,
                                      int tagId) const;
    Q_INVOKABLE SKRResult setTagColor(int            projectId,
                                      int            tagId,
                                      const QString& color);
    Q_INVOKABLE QString   getTagTextColor(int projectId,
                                          int tagId) const;
    Q_INVOKABLE SKRResult setTagTextColor(int            projectId,
                                          int            tagId,
                                          const QString& color);
    Q_INVOKABLE SKRResult setUpdateDate(int              projectId,
                                        int              tagId,
                                        const QDateTime& newDate);
    Q_INVOKABLE QDateTime getUpdateDate(int projectId,
                                        int tagId) const;


    Q_INVOKABLE SKRResult setCreationDate(int              projectId,
                                          int              tagId,
                                          const QDateTime& newDate);
    Q_INVOKABLE QDateTime getCreationDate(int projectId,
                                          int tagId) const;

    SKRResult             set(int             projectId,
                              int             tagId,
                              const QString & fieldName,
                              const QVariant& value,
                              bool            setCurrentDateBool = true,
                              bool            commit             = true);
    QVariant              get(int            projectId,
                              int            tagId,
                              const QString& fieldName) const;
    Q_INVOKABLE int       getLastAddedId();
    Q_INVOKABLE int       getTopTagId(int projectId) const;

    // relationship :
    Q_INVOKABLE QList<TreeItemAddress>getItemIdsFromTag(int projectId,
                                            int tagId) const;
    Q_INVOKABLE QList<int>getTagsFromItemId(const TreeItemAddress &treeItemAddress) const;
    Q_INVOKABLE SKRResult setTagRelationship(const TreeItemAddress &treeItemAddress,
                                             int tagId);
    Q_INVOKABLE SKRResult removeTagRelationship(const TreeItemAddress &treeItemAddress,
                                                int tagId);

    Q_INVOKABLE SKRResult setTagRandomColors(int projectId,
                                             int tagId);

    Q_INVOKABLE QPair<QString, QString> getTagRandomColors();
    Q_INVOKABLE QMap<QString, QString> getTagPresetColors();

signals:

    void errorSent(const SKRResult& result) const;
    void projectModified(int projectId); // for save
    void tagAdded(int projectId,
                  int newTagId);
    void tagRemoved(int projectId,
                    int tagId);
    void tagChanged(int projectId,
                    int tagId);


    void nameChanged(int            projectId,
                     int            tagId,
                     const QString& newName);

    void tagIdChanged(int projectId,
                      int tagId,
                      int newTagId);
    void colorChanged(int            projectId,
                      int            tagId,
                      const QString& newColor);
    void textColorChanged(int            projectId,
                          int            tagId,
                          const QString& newColor);
    void creationDateChanged(int              projectId,
                             int              tagId,
                             const QDateTime& newDate);
    void updateDateChanged(int              projectId,
                           int              tagId,
                           const QDateTime& newDate);


    void tagRelationshipChanged(const TreeItemAddress &treeItemAddress,
                                int tagId);
    void tagRelationshipRemoved(const TreeItemAddress &treeItemAddress,
                                int tagId);
    void tagRelationshipAdded(const TreeItemAddress &treeItemAddress,
                              int tagId);

private:
    void commit(int projectId);

    int m_last_added_id;
};

#endif // SKRTAGHUB_H

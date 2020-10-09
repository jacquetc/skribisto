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
#include "plmerror.h"
#include "skribisto_data_global.h"

class EXPORT SKRTagHub : public QObject {
    Q_OBJECT

public:

    enum ItemType {
        Sheet,
        Note
    };
    Q_ENUM(ItemType)


    explicit SKRTagHub(QObject *parent);

    Q_INVOKABLE QList<int>getAllTagIds(int projectId) const;
    Q_INVOKABLE PLMError  addTag(int            projectId,
                                 const QString& tagName);
    Q_INVOKABLE PLMError  removeTag(int projectId,
                                    int tagId);
    Q_INVOKABLE int       getTagIdWithName(int            projectId,
                                           const QString& tagName);
    Q_INVOKABLE QString   getTagName(int projectId,
                                     int tagId) const;
    Q_INVOKABLE PLMError  setTagName(int            projectId,
                                     int            tagId,
                                     const QString& tagName);
    Q_INVOKABLE bool      doesTagNameAlreadyExist(int            projectId,
                                                  const QString& tagName);
    Q_INVOKABLE QString   getTagColor(int projectId,
                                      int tagId) const;
    Q_INVOKABLE PLMError  setTagColor(int            projectId,
                                      int            tagId,
                                      const QString& color);
    Q_INVOKABLE PLMError  setUpdateDate(int              projectId,
                                        int              paperId,
                                        const QDateTime& newDate);
    Q_INVOKABLE QDateTime getUpdateDate(int projectId,
                                        int tagId) const;


    Q_INVOKABLE PLMError  setCreationDate(int              projectId,
                                          int              paperId,
                                          const QDateTime& newDate);
    Q_INVOKABLE QDateTime getCreationDate(int projectId,
                                          int tagId) const;

    PLMError              set(int             projectId,
                              int             tagId,
                              const QString & fieldName,
                              const QVariant& value,
                              bool            setCurrentDateBool = true);
    QVariant              get(int            projectId,
                              int            tagId,
                              const QString& fieldName) const;
    Q_INVOKABLE int       getLastAddedId();
    Q_INVOKABLE int       getTopPaperId(int projectId) const;

    // relationship :
    Q_INVOKABLE QList<int>getItemIdsFromTag(int  projectId,
                                            int  tagId,
                                            bool haveSeparator = false) const;
    Q_INVOKABLE QList<int>getTagsFromItemId(int                 projectId,
                                            SKRTagHub::ItemType itemType,
                                            int                 itemId) const;
    Q_INVOKABLE PLMError  setTagRelationship(int                 projectId,
                                             SKRTagHub::ItemType itemType,
                                             int                 itemId,
                                             int                 tagId);
    Q_INVOKABLE PLMError removeTagRelationship(int                 projectId,
                                               SKRTagHub::ItemType itemType,
                                               int                 itemId,
                                               int                 tagId);

signals:

    void errorSent(const PLMError& error) const;
    void projectModified(int projectId); // for save
    void tagAdded(int projectId,
                  int newTagId);
    void tagRemoved(int projectId,
                    int tagId);
    void tagChanged(int projectId,
                    int tagId);


    Q_INVOKABLE void nameChanged(int            projectId,
                                 int            tagId,
                                 const QString& newName);
    void             colorChanged(int            projectId,
                                  int            tagId,
                                  const QString& newColor);
    void             creationDateChanged(int              projectId,
                                         int              paperId,
                                         const QDateTime& newDate);
    void             updateDateChanged(int              projectId,
                                       int              tagId,
                                       const QDateTime& newDate);


    void tagRelationshipChanged(int                 projectId,
                                SKRTagHub::ItemType itemType,
                                int                 itemId,
                                int                 tagId);
    void tagRelationshipRemoved(int                 projectId,
                                SKRTagHub::ItemType itemType,
                                int                 itemId,
                                int                 tagId);
    void tagRelationshipAdded(int                 projectId,
                              SKRTagHub::ItemType itemType,
                              int                 itemId,
                              int                 tagId);

private:

    int m_last_added_id;
};

#endif // SKRTAGHUB_H

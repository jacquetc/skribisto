/***************************************************************************
*   Copyright (C) 2017 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmpropertyhub.h
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
#ifndef PLMPROPERTYHUB_H
#define PLMPROPERTYHUB_H

#include <QObject>
#include <QString>
#include <QHash>
#include <QList>
#include <QVariant>
#include <QDateTime>

#include "skrresult.h"
#include "skribisto_data_global.h"


class EXPORT PLMPropertyHub : public QObject {
    Q_OBJECT

public:

    explicit PLMPropertyHub(QObject       *parent,
                            const QString& tableName,
                            const QString& paperCodeFieldName);

    QList<int>           getAllIdsWithPaperCode(int projectId,
                                                int paperCode) const;
    Q_INVOKABLE SKRResult setProperty(int            projectId,
                                     int            paperCode,
                                     const QString& name,
                                     const QString& value, bool triggerProjectModifiedSignal = true);
    int      getLastAddedId();
    SKRResult addProperty(int projectId,
                         int paperCode,
                         int imposedPropertyId = -1);
    SKRResult removeProperty(int projectId,
                            int propertyId);
    SKRResult             setId(int projectId,
                               int propertyId,
                               int newId);
    SKRResult             setValue(int            projectId,
                                  int            propertyId,
                                  const QString& value);
    Q_INVOKABLE SKRResult setName(int            projectId,
                                 int            propertyId,
                                 const QString& name);
    QString              getName(int projectId,
                                 int propertyId);
    SKRResult             setPaperCode(int projectId,
                                      int propertyId,
                                      int paperCode);
    int                  getPaperCode(int projectId,
                                      int propertyId);
    SKRResult             setCreationDate(int              projectId,
                                         int              propertyId,
                                         const QDateTime& date);
    QDateTime            getCreationDate(int projectId,
                                         int propertyId) const;
    SKRResult             setModificationDate(int              projectId,
                                             int              propertyId,
                                             const QDateTime& date);
    QDateTime            getModificationDate(int projectId,
                                             int propertyId) const;
    SKRResult             setIsSystem(int  projectId,
                                     int  propertyId,
                                     bool isSystem);
    bool                 getIsSystem(int projectId,
                                     int propertyId) const;
    bool                 propertyExists(int            projectId,
                                        int            paperCode,
                                        const QString& name);
    int                  findPropertyId(int            projectId,
                                        int            paperCode,
                                        const QString& name);
    QString              getProperty(int            projectId,
                                     int            paperCode,
                                     const QString& name) const;
    QString              getProperty(int            projectId,
                                     int            paperCode,
                                     const QString& name,
                                     const QString& defaultValue) const;
    QString            getPropertyById(int projectId,
                                       int propertyId) const;
    QHash<int, QString>getAllNames(int projectId) const;
    QHash<int, QString>getAllValues(int projectId) const;
    QHash<int, bool>   getAllIsSystems(int projectId) const;
    QHash<int, int>    getAllPaperCodes(int projectId) const;
    QList<int>         getAllIds(int projectId) const;

signals:

    void errorSent(const SKRResult& result) const;
    void projectModified(int projectId);
    void propertyChanged(int            projectId,
                         int            propertyId,
                         int            paperCode,
                         const QString& name,
                         const QString& value);
    void idChanged(int projectId,
                   int propertyId,
                   int newId);
    void propertyAdded(int projectId,
                       int propertyId);
    void propertyRemoved(int projectId,
                         int propertyId);

public slots:

private:

private:

    QString m_tableName, m_paperCodeFieldName;
    int m_last_added_id;
};

#endif // PLMPROPERTYHUB_H

/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                   *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmprojecthub.h                                 *
 *  This file is part of Plume Creator.                                    *
 *                                                                         *
 *  Plume Creator is free software: you can redistribute it and/or modify  *
 *  it under the terms of the GNU General Public License as published by   *
 *  the Free Software Foundation, either version 3 of the License, or      *
 *  (at your option) any later version.                                    *
 *                                                                         *
 *  Plume Creator is distributed in the hope that it will be useful,       *
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
 *  GNU General Public License for more details.                           *
 *                                                                         *
 *  You should have received a copy of the GNU General Public License      *
 *  along with Plume Creator.  If not, see <http://www.gnu.org/licenses/>. *
 ***************************************************************************/
#ifndef PLMPROJECTHUB_H
#define PLMPROJECTHUB_H

#include <QObject>
#include <QString>
#include "plmerror.h"
#include "plume_creator_data_global.h"


class EXPORT PLMProjectHub : public QObject
{
    Q_OBJECT
public:
    explicit PLMProjectHub(QObject *parent);
    Q_INVOKABLE PLMError loadProject(const QString &path);
    PLMError saveProject(int projectId);
    PLMError saveProjectAs(int projectId, const QString &type, const QString &path);
    PLMError closeProject(int projectId);
    PLMError closeAllProjects();
    QList<int> getProjectIdList();
    QString getPath(int projectId) const;
    PLMError setPath(int projectId, const QString &newPath);
    int getLastLoaded() const;
    PLMError getError();



signals:
    void errorSent(const PLMError &error) const;
    void projectLoaded(int projectId);
    void projectClosed(int projectId);
    void allProjectsClosed();
    void projectTypeChanged(int projectId, const QString &newType);
    void projectPathChanged(int projectId, const QString &newPath);
    void projectSaved(int projectId);
public slots:

private slots:
    void setError(const PLMError &error);

private:
    PLMError m_error;
};

#endif // PLMPROJECTHUB_H

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


class EXPORT PLMProjectHub : public QObject {
    Q_OBJECT

public:

    explicit PLMProjectHub(QObject *parent);
    Q_INVOKABLE PLMError loadProject(const QString& path);
    PLMError             saveProject(int projectId);
    PLMError             saveProjectAs(int            projectId,
                                       const QString& type,
                                       const QString& path);
    PLMError             closeProject(int projectId);
    PLMError             closeAllProjects();
    QList<int>           getProjectIdList();
    QString              getPath(int projectId) const;
    PLMError             setPath(int            projectId,
                                 const QString& newPath);
    int                  getLastLoaded() const;
    PLMError             getError();
    bool                 isThereAnyOpenedProject();

    int                  getDefaultProject();
    void                 setDefaultProject(int defaultProject);


    QList<int>           projectsNotSaved();
    QList<int>           projectsNotYetSavedOnce();

    void                 setProjectNotSavedAnymore(int projectId);

    QString              getProjectName(int projectId) const;
    PLMError             setProjectName(int            projectId,
                                        const QString& projectName);

    PLMError             set(int             projectId,
                             const QString & fieldName,
                             const QVariant& value,
                             bool            setCurrentDateBool = true);
    QVariant get(int            projectId,
                 const QString& fieldName) const;

signals:

    void             errorSent(const PLMError& error) const;
    Q_INVOKABLE void projectLoaded(int projectId);
    void             projectClosed(int projectId);
    void             allProjectsClosed();
    void             projectTypeChanged(int            projectId,
                                        const QString& newType);
    void             projectPathChanged(int            projectId,
                                        const QString& newPath);
    void             projectNameChanged(int            projectId,
                                        const QString& newProjectName);
    void             projectSaved(int projectId);
    void             projectNotSavedAnymore(int projectId);

public slots:

private slots:

    void setError(const PLMError& error);

private:

    PLMError m_error;
    QList<int>m_projectsNotYetSavedOnceList, m_projectsNotSavedList;
    QString m_tableName;
    int m_defaultProject;
};

#endif // PLMPROJECTHUB_H

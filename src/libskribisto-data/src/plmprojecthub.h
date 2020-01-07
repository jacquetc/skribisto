/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                   *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmprojecthub.h                                 *
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
#ifndef PLMPROJECTHUB_H
#define PLMPROJECTHUB_H

#include <QObject>
#include <QString>
#include "plmerror.h"
#include "skribisto_data_global.h"


class EXPORT PLMProjectHub : public QObject {
    Q_OBJECT

public:

    explicit PLMProjectHub(QObject *parent);
    Q_INVOKABLE PLMError loadProject(const QString& path);
    Q_INVOKABLE PLMError             saveProject(int projectId);
    Q_INVOKABLE PLMError             saveProjectAs(int            projectId,
                                       const QString& type,
                                       const QString& path);
    Q_INVOKABLE PLMError             closeProject(int projectId);
    Q_INVOKABLE PLMError             closeAllProjects();
    QList<int>           getProjectIdList();
    Q_INVOKABLE int      getProjectCount();
    Q_INVOKABLE QString              getPath(int projectId) const;
    PLMError             setPath(int            projectId,
                                 const QString& newPath);
    Q_INVOKABLE int                  getLastLoaded() const;
    PLMError             getError();
    Q_INVOKABLE bool                 isThereAnyOpenedProject();

    Q_INVOKABLE int                  getDefaultProject();
    void                 setDefaultProject(int defaultProject);


    QList<int>           projectsNotSaved();
    QList<int>           projectsNotYetSavedOnce();

    void                 setProjectNotSavedAnymore(int projectId);

    Q_INVOKABLE QString              getProjectName(int projectId) const;
    Q_INVOKABLE PLMError             setProjectName(int            projectId,
                                        const QString& projectName);

    QString              getProjectUniqueId(int projectId) const;

    PLMError             set(int             projectId,
                             const QString & fieldName,
                             const QVariant& value,
                             bool            setCurrentDateBool = true);
    QVariant get(int            projectId,
                 const QString& fieldName) const;

signals:

    void             errorSent(const PLMError& error) const;
    Q_INVOKABLE void projectLoaded(int projectId);
    ///
    /// \brief projectToBeClosed
    /// \param projectId
    /// To be used with a direct connection
    Q_INVOKABLE void projectToBeClosed(int projectId);
    Q_INVOKABLE void projectClosed(int projectId);
    Q_INVOKABLE void             allProjectsClosed();
    Q_INVOKABLE void             projectTypeChanged(int            projectId,
                                        const QString& newType);
    Q_INVOKABLE void             projectPathChanged(int            projectId,
                                        const QString& newPath);
    Q_INVOKABLE void             projectNameChanged(int            projectId,
                                        const QString& newProjectName);
    Q_INVOKABLE void             projectSaved(int projectId);
    Q_INVOKABLE void             projectNotSavedAnymore(int projectId);

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

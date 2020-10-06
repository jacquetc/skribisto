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
    Q_PROPERTY(
        bool isThereAnyLoadedProject READ isThereAnyLoadedProject NOTIFY isThereAnyLoadedProjectChanged)

public:

    explicit PLMProjectHub(QObject *parent);
    Q_INVOKABLE PLMError  loadProject(const QUrl& urlFilePath);
    Q_INVOKABLE PLMError  createNewEmptyProject(const QUrl& path);
    Q_INVOKABLE PLMError  saveProject(int projectId);
    Q_INVOKABLE PLMError  saveProjectAs(int            projectId,
                                        const QString& type,
                                        const QUrl   & path);
    Q_INVOKABLE PLMError  saveAProjectCopy(int            projectId,
                                           const QString& type,
                                           const QUrl   & path);
    Q_INVOKABLE PLMError  backupAProject(int            projectId,
                                         const QString& type,
                                         const QUrl   & folderPath);
    Q_INVOKABLE bool      doesBackupOfTheDayExistAtPath(int         projectId,
                                                        const QUrl& folderPath);
    Q_INVOKABLE PLMError  closeProject(int projectId);
    Q_INVOKABLE PLMError  closeAllProjects();
    Q_INVOKABLE QList<int>getProjectIdList();
    Q_INVOKABLE int       getProjectCount();
    Q_INVOKABLE QUrl      getPath(int projectId) const;
    PLMError              setPath(int         projectId,
                                  const QUrl& newUrlPath);
    Q_INVOKABLE int       getLastLoaded() const;
    PLMError              getError();
    Q_INVOKABLE bool      isThereAnyLoadedProject();
    Q_INVOKABLE bool      isURLAlreadyLoaded(const QUrl& newUrlPath);

    Q_INVOKABLE int       getActiveProject();
    Q_INVOKABLE void      setActiveProject(int activeProject);
    Q_INVOKABLE bool      isThisProjectActive(int projectId);


    Q_INVOKABLE QList<int>projectsNotSaved();
    Q_INVOKABLE bool      isProjectSaved(int projectId);
    Q_INVOKABLE QList<int>projectsNotModifiedOnce();
    Q_INVOKABLE bool      isProjectNotModifiedOnce(int projectId);


    Q_INVOKABLE QString   getProjectName(int projectId) const;
    Q_INVOKABLE PLMError  setProjectName(int            projectId,
                                         const QString& projectName);

    QString               getProjectUniqueId(int projectId) const;
    Q_INVOKABLE bool      isThisProjectABackup(int projectId);

    QString getProjectType(int projectId) const;

    PLMError              set(int             projectId,
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
    Q_INVOKABLE void allProjectsClosed();
    Q_INVOKABLE void isThereAnyLoadedProjectChanged(bool value);
    Q_INVOKABLE void projectTypeChanged(int            projectId,
                                        const QString& newType);
    Q_INVOKABLE void activeProjectChanged(int projectId);
    Q_INVOKABLE void projectPathChanged(int         projectId,
                                        const QUrl& newUrlPath);
    Q_INVOKABLE void projectNameChanged(int            projectId,
                                        const QString& newProjectName);
    Q_INVOKABLE void projectSaved(int projectId);
    Q_INVOKABLE void projectNotSavedAnymore(int projectId);
    Q_INVOKABLE void projectCountChanged(int count);

    Q_INVOKABLE void projectIsBackupChanged(int  projectId,
                                            bool isThisABackup);

public slots:

    void setProjectNotSavedAnymore(int projectId);

private slots:

    void setError(const PLMError& error);

private:

    PLMError m_error;
    QList<int>m_projectsNotModifiedOnceList, m_projectsNotSavedList;
    QString m_tableName;
    int m_activeProject;
};

#endif // PLMPROJECTHUB_H

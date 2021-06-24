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
#include "skrresult.h"
#include "skribisto_data_global.h"


class EXPORT PLMProjectHub : public QObject {
    Q_OBJECT
    Q_PROPERTY(
        bool isThereAnyLoadedProject READ isThereAnyLoadedProject NOTIFY isThereAnyLoadedProjectChanged)
    Q_PROPERTY(
        int activeProjectId READ getActiveProject WRITE setActiveProject NOTIFY activeProjectChanged)

public:

    explicit PLMProjectHub(QObject *parent);
    Q_INVOKABLE SKRResult loadProject(const QUrl& urlFilePath,
                                      bool        hidden = false);
    Q_INVOKABLE SKRResult createNewEmptyProject(const QUrl& path,
                                                bool        hidden = false);
    SKRResult             createSilentlyNewSpecificEmptyProject(const QUrl   & path,
                                                                const QString& sqlFile);
    Q_INVOKABLE SKRResult saveProject(int projectId);
    Q_INVOKABLE SKRResult saveProjectAs(int            projectId,
                                        const QString& type,
                                        const QUrl   & path);
    Q_INVOKABLE SKRResult saveAProjectCopy(int            projectId,
                                           const QString& type,
                                           const QUrl   & path);
    Q_INVOKABLE SKRResult backupAProject(int            projectId,
                                         const QString& type,
                                         const QUrl   & folderPath);
    Q_INVOKABLE bool      doesBackupOfTheDayExistAtPath(int         projectId,
                                                        const QUrl& folderPath);
    Q_INVOKABLE SKRResult closeProject(int projectId);
    Q_INVOKABLE SKRResult closeAllProjects();
    Q_INVOKABLE bool      isProjectToBeClosed(int projectId) const;
    Q_INVOKABLE QList<int>getProjectIdList();
    Q_INVOKABLE int       getProjectCount();
    Q_INVOKABLE QUrl      getPath(int projectId) const;
    SKRResult             setPath(int         projectId,
                                  const QUrl& newUrlPath);
    Q_INVOKABLE int       getLastLoaded() const;
    SKRResult             getError();
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
    Q_INVOKABLE SKRResult setProjectName(int            projectId,
                                         const QString& projectName);

    Q_INVOKABLE QString   getLangCode(int projectId) const;
    Q_INVOKABLE SKRResult setLangCode(int            projectId,
                                      const QString& langCode);

    QString               getProjectUniqueId(int projectId) const;
    Q_INVOKABLE bool      isThisProjectABackup(int projectId);

    QString               getProjectType(int projectId) const;

    SKRResult             set(int             projectId,
                              const QString & fieldName,
                              const QVariant& value,
                              bool            setCurrentDateBool = true);
    QVariant get(int            projectId,
                 const QString& fieldName) const;

signals:

    void errorSent(const SKRResult& result) const;

    ///
    /// \brief projectToBeLoaded
    /// To be used with a direct connection
    void projectToBeLoaded();
    void projectLoaded(int projectId);

    ///
    /// \brief projectToBeClosed
    /// \param projectId
    /// To be used with a direct connection
    void projectToBeClosed(int projectId);
    void projectClosed(int projectId);
    void allProjectsClosed();
    void isThereAnyLoadedProjectChanged(bool value);
    void projectTypeChanged(int            projectId,
                            const QString& newType);
    void activeProjectChanged(int projectId);
    void projectPathChanged(int         projectId,
                            const QUrl& newUrlPath);
    void projectNameChanged(int            projectId,
                            const QString& newProjectName);
    void langCodeChanged(int            projectId,
                         const QString& newLang);
    void projectSaved(int projectId);
    void projectNotSavedAnymore(int projectId);
    void projectCountChanged(int count);

    void projectIsBackupChanged(int  projectId,
                                bool isThisABackup);

public slots:

    void setProjectNotSavedAnymore(int projectId);

private slots:

    void setError(const SKRResult& result);

private:

    SKRResult m_error;
    QList<int>m_projectsNotModifiedOnceList, m_projectsNotSavedList;
    QString m_tableName;
    int m_activeProject;
    int m_isProjectToBeClosed;
};

#endif // PLMPROJECTHUB_H

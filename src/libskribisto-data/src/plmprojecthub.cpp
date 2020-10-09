#include "plmprojecthub.h"
#include <QDateTime>
#include <QDebug>
#include <QDir>
#include <QFileInfo>
#include <QRegularExpression>
#include <QUrl>
#include <QVariant>
#include "tasks/plmprojectmanager.h"
#include "tasks/plmsqlqueries.h"

PLMProjectHub::PLMProjectHub(QObject *parent) : QObject(parent),
    m_tableName("tbl_project")
{
    // connection for 'getxxx' functions to have a way to get errors.
    connect(this,
            &PLMProjectHub::errorSent,
            this,
            &PLMProjectHub::setError,
            Qt::DirectConnection);
}

PLMError PLMProjectHub::loadProject(const QUrl& urlFilePath)
{
    // qDebug() << "loading project";
    int projectId  = -1;
    PLMError error = plmProjectManager->loadProject(urlFilePath, projectId);

    // qDebug() << "projectId : " << QString::number(projectId);
    IFOK(error) {
        this->setActiveProject(projectId);
        m_projectsNotModifiedOnceList.append(projectId);
        emit projectLoaded(projectId);
        emit projectCountChanged(this->getProjectCount());
        emit isThereAnyLoadedProjectChanged(true);

        this->setActiveProject(projectId);
    }


    return error;
}

PLMError PLMProjectHub::createNewEmptyProject(const QUrl& path)
{
    int projectId  = -1;
    PLMError error = plmProjectManager->createNewEmptyDatabase(projectId);


    IFOK(error) {
        this->setActiveProject(projectId);
        m_projectsNotModifiedOnceList.append(projectId);
        emit projectLoaded(projectId);
        emit projectCountChanged(this->getProjectCount());
        emit isThereAnyLoadedProjectChanged(true);

        this->setProjectNotSavedAnymore(projectId);
        this->setActiveProject(projectId);

    }
    IFOK(error) {
        if(path.isValid()){
            error = this->saveProjectAs(projectId, this->getProjectType(projectId), path);
        }
    }

    return error;
}

PLMError PLMProjectHub::saveProject(int projectId)
{
    PLMError error;

    error = plmProjectManager->saveProject(projectId);
    IFOK(error) {
        m_projectsNotModifiedOnceList.removeAll(projectId);
        m_projectsNotSavedList.removeAll(projectId);
        emit projectSaved(projectId);
    }
    return error;
}

///
/// \brief PLMProjectHub::setProjectNotSavedAnymore
/// \param projectId
/// called every modification
void PLMProjectHub::setProjectNotSavedAnymore(int projectId)
{
    if (!m_projectsNotSavedList.contains(projectId)) {
        m_projectsNotSavedList.append(projectId);
        m_projectsNotModifiedOnceList.removeAll(projectId);
        emit projectNotSavedAnymore(projectId);
    }
}

QList<int>PLMProjectHub::projectsNotModifiedOnce() {
    return m_projectsNotModifiedOnceList;
}

bool PLMProjectHub::isProjectNotModifiedOnce(int projectId)
{
    return m_projectsNotModifiedOnceList.contains(projectId);
}

bool PLMProjectHub::isProjectSaved(int projectId)
{
    return !m_projectsNotSavedList.contains(projectId);
}

QList<int>PLMProjectHub::projectsNotSaved() {
    return m_projectsNotSavedList;
}

PLMError PLMProjectHub::saveProjectAs(int            projectId,
                                      const QString& type,
                                      const QUrl   & path)
{
    PLMError error;

    // determine if this is a backup project
    bool isThisABackup = isThisProjectABackup(projectId);


    // save
    error = plmProjectManager->saveProjectAs(projectId, type, path);
    IFOK(error) {
        m_projectsNotModifiedOnceList.removeAll(projectId);
        m_projectsNotSavedList.removeAll(projectId);

        if (isThisABackup) {
            emit projectIsBackupChanged(projectId, false);
        }

        emit projectSaved(projectId);
    }
    return error;
}

// ----------------------------------------------------------------------------

PLMError PLMProjectHub::saveAProjectCopy(int            projectId,
                                         const QString& type,
                                         const QUrl   & path)
{
    PLMError error;

    // firstly, save the project
    error = this->saveProjectAs(projectId, type, path);

    // then create a copy
    IFOK(error) {
        error = plmProjectManager->saveProjectAs(projectId, type, path, true);
    }
    return error;
}

// ----------------------------------------------------------------------------

PLMError PLMProjectHub::backupAProject(int            projectId,
                                       const QString& type,
                                       const QUrl   & folderPath) {
    PLMError error;

    QUrl projectPath = this->getPath(projectId);

    if (projectPath.isEmpty()) {
        error.addData(projectId);
        error.setSuccess(false);
        error.setErrorCode("E_PROJECT_no_path");
    }

    if (projectPath.scheme() == "qrc") {
        error.addData(projectId);
        error.setSuccess(false);
        error.setErrorCode("E_PROJECT_qrc_projects_cant_back_up");
    }


    IFOK(error) {
        // verify backup path
        QFileInfo folderInfo(folderPath.toLocalFile());

        if (!folderInfo.exists()) {
            error.addData(projectId);
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_dont_exist");
        }

        if (!folderInfo.isDir()) {
            error.addData(projectId);
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_not_a_directory");
        }

        if (!folderInfo.isWritable()) {
            error.addData(projectId);
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_not_writable");
        }
    }


    // determine file base
    QFileInfo info(projectPath.toLocalFile());
    QFileInfo backupFolderInfo(folderPath.toLocalFile());
    QString   backupFile = backupFolderInfo.filePath() + "/" + info.completeBaseName();

    // add date and time :
    QDateTime now     = QDateTime::currentDateTime();
    QString   nowText = now.toString("_yyyy-MM-dd-HHmmss");

    backupFile = backupFile + nowText;

    // add suffix :
    backupFile = backupFile + "." + type;

    // firstly, save the project
    IFOKDO(error, this->saveProject(projectId));

    // then create a copy
    IFOK(error) {
        error =
                plmProjectManager->saveProjectAs(projectId,
                                                 type,
                                                 QUrl::fromLocalFile(backupFile),
                                                 true);
    }
    return error;
}

// ----------------------------------------------------------------------------

bool PLMProjectHub::doesBackupOfTheDayExistAtPath(int projectId, const QUrl& folderPath) {
    PLMError error;

    QUrl projectPath = this->getPath(projectId);

    if (projectPath.isEmpty()) {
        error.addData(projectId);
        error.setSuccess(false);
        error.setErrorCode("E_PROJECT_no_path");
    }
    IFOK(error) {
        // verify backup path
        QFileInfo folderInfo(folderPath.toLocalFile());

        if (!folderInfo.exists()) {
            error.addData(projectId);
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_dont_exist");
        }

        if (!folderInfo.isDir()) {
            error.addData(projectId);
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_not_a_directory");
        }

        if (!folderInfo.isWritable()) {
            error.addData(projectId);
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_not_writable");
        }
    }
    IFKO(error) {
        emit errorSent(error);

        return true;
    }


    // determine file base
    QFileInfo info(projectPath.toLocalFile());
    QString   backupFileOfTheDay = info.completeBaseName();

    // add date and time :
    QDateTime now     = QDateTime::currentDateTime();
    QString   nowText = now.toString("_yyyy-MM-dd");

    backupFileOfTheDay = backupFileOfTheDay + nowText + "*";

    // find file begining with backupFileOfTheDay

    QDir dir(folderPath.toLocalFile());
    QStringList nameFilters;

    nameFilters << backupFileOfTheDay;
    QStringList entries = dir.entryList(nameFilters);

    if (entries.isEmpty()) {
        return false;
    }
    return true;
}

// ----------------------------------------------------------------------------

PLMError PLMProjectHub::closeProject(int projectId)
{
    PLMError error;
    emit     projectToBeClosed(projectId);

    error = plmProjectManager->closeProject(projectId);
    IFOK(error) {
        emit projectClosed(projectId);
        emit projectCountChanged(this->getProjectCount());

        if (isThereAnyLoadedProject()) {
            emit isThereAnyLoadedProjectChanged(true);
        }
        else {
            emit isThereAnyLoadedProjectChanged(false);
        }
        m_projectsNotModifiedOnceList.removeAll(projectId);
        m_projectsNotSavedList.removeAll(projectId);
    }
    return error;
}

// ----------------------------------------------------------------------------

PLMError PLMProjectHub::closeAllProjects()
{
    PLMError error;

    QList<int> idList = plmProjectManager->projectIdList();

    for (int id : idList) {
        IFOKDO(error, closeProject(id));
        IFKO(error) {
            break;
        }
    }

    IFOK(error) {
        emit allProjectsClosed();
    }
    return error;
}
// ----------------------------------------------------------------------------


QList<int>PLMProjectHub::getProjectIdList()
{
    return plmProjectManager->projectIdList();
}

// ----------------------------------------------------------------------------

int PLMProjectHub::getProjectCount()
{
    return plmProjectManager->projectIdList().count();
}

// ----------------------------------------------------------------------------

QUrl PLMProjectHub::getPath(int projectId) const
{
    PLMError error;
    QUrl     result;
    PLMProject *project = plmProjectManager->project(projectId);

    if (!project) {
        error.setSuccess(false);
    }

    IFOK(error) {
        result = project->getPath();
    }
    IFKO(error) {
        // no project
        emit errorSent(error);
    }
    return result;
}

// ----------------------------------------------------------------------------

PLMError PLMProjectHub::setPath(int projectId, const QUrl& newUrlPath)
{
    PLMError error;
    PLMProject *project = plmProjectManager->project(projectId);

    if (!project) {
        error.setSuccess(false);
    }

    IFOKDO(error, project->setPath(newUrlPath))
            IFOK(error) {
        emit projectPathChanged(projectId, newUrlPath);
    }
    return error;
}
// ----------------------------------------------------------------------------

bool PLMProjectHub::isURLAlreadyLoaded(const QUrl& newUrlPath){
    QList<int> list = plmProjectManager->projectIdList();

    for(int id : list){
        if(newUrlPath == this->getPath(id)){
            return true;
        }
    }
    return false;

}


// ----------------------------------------------------------------------------
int PLMProjectHub::getLastLoaded() const
{
    QList<int> list = plmProjectManager->projectIdList();
    int lastId      = -1;

    if (!list.isEmpty()) {
        lastId = list.last();
    }

    return lastId;
}

// ----------------------------------------------------------------------------

PLMError PLMProjectHub::getError()
{
    return m_error;
}

bool PLMProjectHub::isThereAnyLoadedProject()
{
    QList<int> list = plmProjectManager->projectIdList();

    if (list.isEmpty()) {
        return false;
    }
    return true;
}

// ----------------------------------------------------------------------------

bool PLMProjectHub::isThisProjectABackup(int projectId)
{
    if (this->getPath(projectId).isEmpty()) {
        return false;
    }

    if (this->getPath(projectId).toLocalFile().contains(QRegularExpression(
                                                            "_\\d\\d\\d\\d-\\d\\d-\\d\\d-\\d\\d\\d\\d\\d\\d")))
    {
        return true;
    }
    return false;
}

// ----------------------------------------------------------------------------

QString PLMProjectHub::getProjectType(int projectId) const
{
    PLMError error;
    QString result;
    PLMProject *project = plmProjectManager->project(projectId);

    if (!project) {
        error.setSuccess(false);
    }

    IFOK(error) {
        result = project->getType();
    }
    IFKO(error) {
        // no project
        emit errorSent(error);
    }
    return result;
}

// ----------------------------------------------------------------------------

void PLMProjectHub::setActiveProject(int activeProject)
{
    m_activeProject = activeProject;

    emit activeProjectChanged(m_activeProject);
}

// ----------------------------------------------------------------------------

bool PLMProjectHub::isThisProjectActive(int projectId)
{
    return this->getActiveProject() == projectId;
}

// ----------------------------------------------------------------------------

int PLMProjectHub::getActiveProject()
{
    if (this->getProjectIdList().isEmpty()) {
        m_activeProject = -2;
    }
    else if (!this->getProjectIdList().contains(m_activeProject)) {
        m_activeProject = this->getProjectIdList().first();
    }

    if (this->getProjectIdList().count() == 1) {
        m_activeProject = this->getProjectIdList().first();
    }
    return m_activeProject;
}

// ----------------------------------------------------------------------------

void PLMProjectHub::setError(const PLMError& error)
{
    m_error = error;
}

// ----------------------------------------------------------------------------

QString PLMProjectHub::getProjectName(int projectId) const {
    return get(projectId, "t_project_name").toString();
}

PLMError PLMProjectHub::setProjectName(int projectId, const QString& projectName) {
    PLMError error = this->set(projectId, "t_project_name", projectName, true);

    IFOK(error) {
        emit projectNameChanged(projectId, projectName);

        this->projectNotSavedAnymore(projectId);
    }
    IFKO(error) {
        emit errorSent(error);

        this->setProjectNotSavedAnymore(projectId);
    }
    return error;
}

// ----------------------------------------------------------------------------

QString PLMProjectHub::getProjectUniqueId(int projectId) const
{
    return get(projectId, "t_project_unique_identifier").toString();
}

// ----------------------------------------------------------------------------

PLMError PLMProjectHub::set(int             projectId,
                            const QString & fieldName,
                            const QVariant& value,
                            bool            setCurrentDateBool)
{
    PLMError error;
    PLMSqlQueries queries(projectId, m_tableName);
    int id = 1;

    queries.beginTransaction();
    error = queries.set(id, fieldName, value);

    if (setCurrentDateBool) {
        IFOKDO(error, queries.setCurrentDate(id, "dt_updated"));
    }

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return error;
}

QVariant PLMProjectHub::get(int projectId, const QString& fieldName) const
{
    PLMError error;
    QVariant var;
    QVariant result;
    int id = 1;
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.get(id, fieldName, var);
    IFOK(error) {
        result = var;
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

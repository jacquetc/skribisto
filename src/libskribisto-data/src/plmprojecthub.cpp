#include "plmprojecthub.h"
#include <QDateTime>
#include <QDebug>
#include <QDir>
#include <QFileInfo>
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

PLMError PLMProjectHub::loadProject(const QString& path)
{
    // qDebug() << "loading project";
    int projectId  = -1;
    PLMError error = plmProjectManager->loadProject(path, projectId);

    // qDebug() << "projectId : " << QString::number(projectId);
    IFOK(error) {
        m_projectsNotModifiedOnceList.append(projectId);
        emit projectLoaded(projectId);
        emit projectCountChanged(this->getProjectCount());
        emit isThereAnyLoadedProjectChanged(true);
        this->setDefaultProject(projectId);
    }


    return error;
}

PLMError PLMProjectHub::createNewEmptyProject(const QString &path)
{

    int projectId  = -1;
    PLMError error = plmProjectManager->createNewEmptyDatabase(projectId);


    IFOK(error) {
        m_projectsNotModifiedOnceList.append(projectId);
        emit projectLoaded(projectId);
        emit projectCountChanged(this->getProjectCount());
        emit isThereAnyLoadedProjectChanged(true);
        this->setProjectNotSavedAnymore(projectId);
        this->setDefaultProject(projectId);
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
    if(!m_projectsNotSavedList.contains(projectId)){
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
                                      const QString& path)
{
    PLMError error;

    error = plmProjectManager->saveProjectAs(projectId, type, path);
    IFOK(error) {
        m_projectsNotModifiedOnceList.removeAll(projectId);
        m_projectsNotSavedList.removeAll(projectId);
        emit projectSaved(projectId);
    }
    return error;
}

PLMError PLMProjectHub::saveAProjectCopy(int projectId, const QString &type, const QString &path)
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


PLMError PLMProjectHub::backupAProject(int projectId, const QString &type, const QString &folderPath){


    PLMError error;

    QString projectPath = this->getPath(projectId);
    if (projectPath == ""){
        error.addData(projectId);
        error.setSuccess(false);
        error.setErrorCode("E_PROJECT_no_path");
    }
    IFOK(error){
        // verify backup path
        QFileInfo folderInfo(folderPath);
        if(!folderInfo.exists()){
            error.addData(projectId);
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_dont_exist");

        }
        if(!folderInfo.isDir()){
            error.addData(projectId);
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_not_a_directory");

        }
        if(!folderInfo.isWritable()){
            error.addData(projectId);
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_not_writable");

        }
    }


    // determine file base
    QFileInfo info(projectPath);
    QString  backupFile = info.canonicalPath() + "/" + info.completeBaseName();

    // add date and time :
    QDateTime now = QDateTime::currentDateTime();
    QString nowText = now.toString("_yyyy-MM-dd-HHmmss");
    backupFile = backupFile + nowText;

    //add suffix :
    backupFile = backupFile + "." + type;

    // firstly, save the project
    IFOKDO(error, this->saveProjectAs(projectId, type, backupFile));
    // then create a copy
    IFOK(error) {
        error = plmProjectManager->saveProjectAs(projectId, type, backupFile, true);
    }
    return error;

}


bool PLMProjectHub::doesBackupOfTheDayExistAtPath(int projectId, const QString &folderPath){

    PLMError error;

    QString projectPath = this->getPath(projectId);
    if (projectPath == ""){
        error.addData(projectId);
        error.setSuccess(false);
        error.setErrorCode("E_PROJECT_no_path");
    }
    IFOK(error){
        // verify backup path
        QFileInfo folderInfo(folderPath);
        if(!folderInfo.exists()){
            error.addData(projectId);
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_dont_exist");

        }
        if(!folderInfo.isDir()){
            error.addData(projectId);
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_not_a_directory");

        }
        if(!folderInfo.isWritable()){
            error.addData(projectId);
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_not_writable");

        }
    }
    IFKO(error){

        emit errorSent(error);
        return true;
    }



    // determine file base
    QFileInfo info(projectPath);
    QString  backupFileOfTheDay = info.completeBaseName();

    // add date and time :
    QDateTime now = QDateTime::currentDateTime();
    QString nowText = now.toString("_yyyy-MM-dd");
    backupFileOfTheDay = backupFileOfTheDay + nowText + "*";

    //find file begining with backupFileOfTheDay

    QDir dir(folderPath);
    QStringList nameFilters;
    nameFilters << backupFileOfTheDay;
    QStringList entries = dir.entryList(nameFilters);

    if(entries.isEmpty()){
        return false;
    }
    return true;

}


PLMError PLMProjectHub::closeProject(int projectId)
{
    PLMError error;
    emit projectToBeClosed(projectId);
    error = plmProjectManager->closeProject(projectId);
    IFOK(error) {
        emit projectClosed(projectId);
        emit projectCountChanged(this->getProjectCount());
        if (isThereAnyLoadedProject()){
            emit isThereAnyLoadedProjectChanged(true);
        }
        else{
            emit isThereAnyLoadedProjectChanged(false);
        }
        m_projectsNotModifiedOnceList.removeAll(projectId);
        m_projectsNotSavedList.removeAll(projectId);
    }
    return error;
}

PLMError PLMProjectHub::closeAllProjects()
{
    PLMError error;

    QList<int> idList = plmProjectManager->projectIdList();

    for(int id : idList) {
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

QList<int>PLMProjectHub::getProjectIdList()
{
    return plmProjectManager->projectIdList();
}

int PLMProjectHub::getProjectCount()
{
    return plmProjectManager->projectIdList().count();
}

QString PLMProjectHub::getPath(int projectId) const
{
    PLMError error;
    QString  result("");
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

PLMError PLMProjectHub::setPath(int projectId, const QString& newPath)
{
    PLMError error;
    PLMProject *project = plmProjectManager->project(projectId);

    if (!project) {
        error.setSuccess(false);
    }

    IFOKDO(error, project->setPath(newPath))
            IFOK(error) {
        emit projectPathChanged(projectId, newPath);
    }
    return error;
}

int PLMProjectHub::getLastLoaded() const
{
    QList<int> list = plmProjectManager->projectIdList();
    int lastId      = -1;

    if (!list.isEmpty()) {
        lastId = list.last();
    }

    return lastId;
}

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

void PLMProjectHub::setDefaultProject(int defaultProject)
{
    m_defaultProject = defaultProject;

    emit defaultProjectChanged(m_defaultProject);
}

int PLMProjectHub::getDefaultProject()
{

    if (this->getProjectIdList().isEmpty()) {
        m_defaultProject = -2;
    }
    else if (!this->getProjectIdList().contains(m_defaultProject)) {
        m_defaultProject = this->getProjectIdList().first();
    }

    if (this->getProjectIdList().count() == 1) {
        m_defaultProject = this->getProjectIdList().first();
    }
    return m_defaultProject;
}

void PLMProjectHub::setError(const PLMError& error)
{
    m_error = error;
}

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

QString PLMProjectHub::getProjectUniqueId(int projectId) const
{
    return get(projectId, "t_project_unique_identifier").toString();
}

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

#include "plmprojecthub.h"
#include <QDebug>
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
        m_projectsNotYetSavedOnceList.append(projectId);
        m_projectsNotSavedList.append(projectId);
        emit projectLoaded(projectId);
    }


    return error;
}

PLMError PLMProjectHub::saveProject(int projectId)
{
    PLMError error;

    error = plmProjectManager->saveProject(projectId);
    IFOK(error) {
        m_projectsNotYetSavedOnceList.removeAll(projectId);
        m_projectsNotSavedList.removeAll(projectId);
        emit projectSaved(projectId);
    }
    return error;
}

void PLMProjectHub::setProjectNotSavedAnymore(int projectId)
{
    m_projectsNotSavedList.append(projectId);
    emit projectNotSavedAnymore(projectId);
}

QList<int>PLMProjectHub::projectsNotYetSavedOnce() {
    return m_projectsNotYetSavedOnceList;
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
        m_projectsNotYetSavedOnceList.removeAll(projectId);
        m_projectsNotSavedList.removeAll(projectId);
        emit projectSaved(projectId);
    }
    return error;
}

PLMError PLMProjectHub::closeProject(int projectId)
{
    PLMError error;

    error = plmProjectManager->closeProject(projectId);
    IFOK(error) {
        emit projectClosed(projectId);
    }
    return error;
}

PLMError PLMProjectHub::closeAllProjects()
{
    PLMError error;

    QList<int> idList = plmProjectManager->projectIdList();

    foreach(int id, idList) {
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

QString PLMProjectHub::getPath(int projectId) const
{
    PLMError error;
    QString  result("");
    PLMProject *project = plmProjectManager->project(projectId);

    if (!project) {
        error.setSuccess(false);
    }

    IFKO(error) {
        // no project
    }
    IFOK(error) {
        result = project->getPath();
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

bool PLMProjectHub::isThereAnyOpenedProject()
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
}

int PLMProjectHub::getDefaultProject()
{
    if (!this->getProjectIdList().contains(m_defaultProject)) {
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

    return error;
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

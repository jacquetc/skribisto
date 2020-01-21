#include "plmprojectmanager.h"
#include "sql/plmexporter.h"

#include <QCoreApplication>
#include <QDebug>

PLMProjectManager::PLMProjectManager(QObject *parent) : QObject(parent)
{
    m_projectIdIncrement = 0;
    m_instance           = this;
}

// -----------------------------------------------------------------------------

PLMError PLMProjectManager::createNewEmptyDatabase(int& projectId)
{
    return loadProject("", projectId);
}

// -----------------------------------------------------------------------------

PLMProjectManager *PLMProjectManager::m_instance = 0;

// -----------------------------------------------------------------------------


PLMError PLMProjectManager::loadProject(const QString& fileName, int& projectId)
{
    PLMError error;

    m_projectIdIncrement += 1;
    projectId             = m_projectIdIncrement;
    PLMProject *project = new PLMProject(this, projectId, fileName);

    // if error :
    if (project->id() == -1) {
        // emit plmTaskError->errorSent("E_PROJECT_PROJECTNOTLOADED",
        // Q_FUNC_INFO, "");
        project->deleteLater();
        projectId = -1;
        error.setSuccess(false);
        return error;
    }

    m_projectForIntHash.insert(projectId, project);
    return error;
}

// -----------------------------------------------------------------------------

PLMError PLMProjectManager::saveProject(int projectId)
{
    PLMProject *project = m_projectForIntHash.value(projectId, 0);

    return saveProjectAs(projectId, project->getType(), project->getPath());
}

// -----------------------------------------------------------------------------

PLMError PLMProjectManager::saveProjectAs(int            projectId,
                                          const QString& type,
                                          const QString& path)
{
    PLMError error;
    PLMProject *project = m_projectForIntHash.value(projectId, 0);

    if (!project) {
        // emit plmTaskError->errorSent("E_PROJECT_PROJECTMISSING", Q_FUNC_INFO,
        // "No project with the id " + QString::number(projectId));
        error.setSuccess(false);
        return error;
    }

    if (project->getPath() == "") {
        // emit plmTaskError->errorSent("E_PROJECT_NOPATH", Q_FUNC_INFO, "No
        // project path set");
        error.setSuccess(false);
        return error;
    }

    PLMExporter exporter(this);
    IFOKDO(error, exporter.exportSQLiteDbTo(project, type, path));
    IFOK(error) {
        project->setPath(path);
    }
    return error;
}

PLMProject * PLMProjectManager::project(int projectId)
{
    //    if(!m_projectForIntHash.contains(projectId)){
    //    }
    // qDebug()   <<  "project n°" << projectId;
    PLMProject *project = m_projectForIntHash.value(projectId, 0);

    if (!project) {
        // emit plmTaskError->errorSent("E_PROJECTMISSING", Q_FUNC_INFO, "No
        // project with the id " + QString::number(projectId));

        qDebug() << "project not found : " << projectId;
    }

    return project;
}

// -----------------------------------------------------------------------------

QList<int>PLMProjectManager::projectIdList()
{
    return m_projectForIntHash.keys();
}

// -----------------------------------------------------------------------------

PLMError PLMProjectManager::closeProject(int projectId)
{
    PLMError error;
    PLMProject *project = m_projectForIntHash.value(projectId, 0);

    if (!project) {
        error.setSuccess(false);
        return error;
    }

    m_projectForIntHash.remove(projectId);

    // the project deletion is done outside PLMProject() so the QSqlDatabase is
    // out of scope
    {
    delete project;
    QSqlDatabase::removeDatabase(QString::number(projectId));
    }
    error.setSuccess(true);
    return error;
}

// -----------------------------------------------------------------------------

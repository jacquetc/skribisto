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

SKRResult PLMProjectManager::createNewEmptyDatabase(int& projectId)
{
    return loadProject(QUrl(), projectId);
}

// -----------------------------------------------------------------------------

PLMProjectManager *PLMProjectManager::m_instance = 0;

// -----------------------------------------------------------------------------


SKRResult PLMProjectManager::loadProject(const QUrl& fileName, int& projectId)
{
    SKRResult result(this);

    m_projectIdIncrement += 1;
    projectId             = m_projectIdIncrement;
    PLMProject *project = new PLMProject(this, projectId, fileName, &result);

    // if result :
    if (project->id() == -1) {
        // emit plmTaskError->errorSent("C_PROJECT_PROJECTNOTLOADED",
        // Q_FUNC_INFO, "");
        project->deleteLater();
        projectId = -1;
        result = SKRResult(SKRResult::Critical, this, "project_not_loaded");
        return result;
    }

    IFOK(result){
        m_projectForIntMap.insert(projectId, project);
    }

    return result;
}

// -----------------------------------------------------------------------------

SKRResult PLMProjectManager::saveProject(int projectId)
{
    PLMProject *project = m_projectForIntMap.value(projectId, 0);

    return saveProjectAs(projectId, project->getType(), project->getPath());
}

// -----------------------------------------------------------------------------

SKRResult PLMProjectManager::saveProjectAs(int projectId,
                                          const QString& type,
                                          const QUrl& path, bool isCopy)
{
    SKRResult result(this);
    PLMProject *project = m_projectForIntMap.value(projectId, 0);

    if (!project) {
        // emit plmTaskError->errorSent("C_PROJECT_PROJECTMISSING", Q_FUNC_INFO,
        // "No project with the id " + QString::number(projectId));
        result = SKRResult(SKRResult::Critical, this, "project_missing");
        result.addData("projectId", projectId);
        return result;
    }

    if (path.isEmpty()) {
        // emit plmTaskError->errorSent("C_PROJECT_NOPATH", Q_FUNC_INFO, "No
        // project path set");
        result = SKRResult(SKRResult::Critical, this, "no_path");
        result.addData("projectId", projectId);
        return result;
    }

    PLMExporter exporter(this);

    IFOKDO(result, exporter.exportSQLiteDbTo(project, type, path));
    IFOK(result) {
        // if it's a true save and not a copy :
        if (!isCopy) {
            project->setPath(path);
        }
    }
    return result;
}

PLMProject * PLMProjectManager::project(int projectId)
{
    //    if(!m_projectForIntHash.contains(projectId)){
    //    }
    // qDebug()   <<  "project n°" << projectId;
    PLMProject *project = m_projectForIntMap.value(projectId, 0);

    if (!project) {
        // emit plmTaskError->errorSent("C_PROJECTMISSING", Q_FUNC_INFO, "No
        // project with the id " + QString::number(projectId));

        qDebug() << "project not found : " << projectId;
    }

    return project;
}

// -----------------------------------------------------------------------------

QList<int>PLMProjectManager::projectIdList()
{
    return m_projectForIntMap.keys();
}

// -----------------------------------------------------------------------------

SKRResult PLMProjectManager::closeProject(int projectId)
{
    SKRResult result(this);
    PLMProject *project = m_projectForIntMap.value(projectId, 0);

    if (!project) {
        result = SKRResult(SKRResult::Critical, this, "project_not_found");
        return result;
    }

    m_projectForIntMap.remove(projectId);

    // the project deletion is done outside PLMProject() so the QSqlDatabase is
    // out of scope
    {
        delete project;
        QSqlDatabase::removeDatabase(QString::number(projectId));
    }
    return result;
}

// -----------------------------------------------------------------------------

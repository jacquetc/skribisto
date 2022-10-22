#include "plmprojectmanager.h"

#include <QCoreApplication>
#include <QDebug>

PLMProjectManager::PLMProjectManager(QObject *parent) : QObject(parent)
{
    m_projectIdIncrement = 0;
    m_instance           = this;
}


// -----------------------------------------------------------------------------

PLMProjectManager *PLMProjectManager::m_instance = 0;

// -----------------------------------------------------------------------------


SKRResult PLMProjectManager::loadProject(PLMProject *project)
{
    SKRResult result(this);

    m_projectIdIncrement += 1;
    project->setId(m_projectIdIncrement);
    m_projectForIntMap.insert(project->id(), project);
    return result;
}


PLMProject * PLMProjectManager::project(int projectId)
{
    //    if(!m_projectForIntHash.contains(projectId)){
    //    }
    // qDebug()   <<  "project n°" << projectId;
    PLMProject *project = m_projectForIntMap.value(projectId, nullptr);

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
    SKRResult   result(this);
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

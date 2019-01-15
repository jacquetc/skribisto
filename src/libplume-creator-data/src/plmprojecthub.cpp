#include "plmprojecthub.h"
#include <QDebug>
#include <QVariant>
#include "tasks/plmprojectmanager.h"

PLMProjectHub::PLMProjectHub(QObject *parent) : QObject(parent)
{
    // connection for 'getxxx' functions to have a way to get errors.
    connect(this, &PLMProjectHub::errorSent, this, &PLMProjectHub::setError, Qt::DirectConnection);
}


PLMError PLMProjectHub::loadProject(const QString &path)
{
    //qDebug() << "loading project";
    int projectId = -1;
    PLMError error = plmProjectManager->loadProject(path, projectId);
    //qDebug() << "projectId : " << QString::number(projectId);
    IFOK(error) {
        emit projectLoaded(projectId);
    }
    return error;
}

PLMError PLMProjectHub::saveProject(int projectId)
{
    PLMError error;
    error = plmProjectManager->saveProject(projectId);
    IFOK(error) {
        emit projectSaved(projectId);
    }
    return error;
}

PLMError PLMProjectHub::saveProjectAs(int projectId, const QString &type, const QString &path)
{
    PLMError error;
    error = plmProjectManager->saveProjectAs(projectId, type, path);
    IFOK(error) {
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

    foreach (int id, idList) {
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

QList<int> PLMProjectHub::getProjectIdList()
{
    return plmProjectManager->projectIdList();
}

QString PLMProjectHub::getPath(int projectId) const
{
    PLMError error;
    QString result("");
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

PLMError PLMProjectHub::setPath(int projectId, const QString &newPath)
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
    int lastId = -1;

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

void PLMProjectHub::setError(const PLMError &error)
{
    m_error = error;
}

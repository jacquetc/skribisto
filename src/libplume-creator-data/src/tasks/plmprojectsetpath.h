#ifndef PLMPROJECTSETPATH_H
#define PLMPROJECTSETPATH_H
#include "plmtask.h"
#include "plmprojectmanager.h"
#include "plmdata.h"

#include <QString>



class PLMProjectSetPath : public PLMTask
{

public:
    PLMProjectSetPath(int projectId, const QString &newPath) {
        setType(Setter);
        m_projectId = projectId;
        m_newPath = newPath;

    }

    void doTask(bool *ok){
        emit taskStarted();

        PLMProject *project = plmProjectManager->project(m_projectId);
        if(!project){ *ok = false; return; }
        project->setPath(m_newPath);
        emit plmdata->projectHub()->projectPathChanged(m_projectId, m_newPath);

        *ok = true;
        emit taskFinished();
    }

private:
    int m_projectId;
    QString m_newPath;

};
#endif // PLMPROJECTSETPATH_H

#ifndef PLMPROJECTGETPATH_H
#define PLMPROJECTGETPATH_H
#include "plmtask.h"
#include "plmprojectmanager.h"

#include <QString>



class PLMProjectGetPath : public PLMTask
{

public:
    PLMProjectGetPath(int projectId) {
        setType(Getter);
        m_projectId = projectId;

    }

    void doTask(bool *ok){
        emit taskStarted();

        PLMProject *project = plmProjectManager->project(m_projectId);
        if(!project){ *ok = false; return; }
        QString path = project->getPath();
        setReturnValue(path);

        *ok = true;
        emit taskFinished();
    }

private:
    int m_projectId;

};
#endif // PLMPROJECTGETPATH_H

#ifndef PLMPROJECTSAVEPROJECT_H
#define PLMPROJECTSAVEPROJECT_H
#include "plmtask.h"
#include "plmprojectmanager.h"
#include "plmdata.h"

#include <QString>



class PLMProjectSaveProject : public PLMTask
{

public:
    PLMProjectSaveProject(int projectId) {
        setType(Setter);
        m_projectId = projectId;

    }

    void doTask(bool *ok){
        emit taskStarted();

        bool success = plmProjectManager->saveProject(m_projectId);
        if(!success){ *ok = false; return; }

        emit plmdata->projectHub()->projectSaved(m_projectId);

        *ok = true;
        emit taskFinished();
    }

private:
    int m_projectId;

};
#endif // PLMPROJECTSAVEPROJECT_H

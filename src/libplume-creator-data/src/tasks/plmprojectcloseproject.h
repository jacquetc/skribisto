#ifndef PLMPROJECTCLOSEPROJECT_H
#define PLMPROJECTCLOSEPROJECT_H

#include <QString>
#include <QVariant>

#include "plmtask.h"
#include "plmdata.h"
#include "plmprojectmanager.h"

class PLMProjectCloseProject : public PLMTask
{

public:
    PLMProjectCloseProject(int projectId) {
        setType(Setter);
        m_projectId = projectId;
    }

    void doTask(bool *ok){
        emit taskStarted();

        *ok = plmProjectManager->closeProject(m_projectId);

        emit plmdata->projectHub()->projectClosed(m_projectId);
        QList<int> idList = plmProjectManager->projectIdList();
        if(idList.isEmpty())
            emit plmdata->projectHub()->allProjectsClosed();

        emit taskFinished();
    }

private:
    int m_projectId;

};
#endif // PLMPROJECTCLOSEPROJECT_H

#ifndef PLMPROJECTSAVEPROJECTAS_H
#define PLMPROJECTSAVEPROJECTAS_H
#include "plmtask.h"
#include "plmprojectmanager.h"
#include "plmdata.h"

#include <QString>



class PLMProjectSaveProjectAs : public PLMTask
{

public:
    PLMProjectSaveProjectAs(int projectId, const QString &type, const QString &newPath) {
        setType(Setter);
        m_projectId = projectId;
        m_newPath = newPath;
        m_type = type;

    }

    void doTask(bool *ok){
        emit taskStarted();

        bool success = plmProjectManager->saveProjectAs(m_projectId, m_type, m_newPath);
        if(!success){ *ok = false; return; }

        emit plmdata->projectHub()->projectSaved(m_projectId);
        emit plmdata->projectHub()->projectTypeChanged(m_projectId, m_type);
        emit plmdata->projectHub()->projectPathChanged(m_projectId, m_newPath);

        *ok = true;
        emit taskFinished();
    }

private:
    int m_projectId;
    QString m_type, m_newPath;

};
#endif // PLMPROJECTSAVEPROJECTAS_H

#ifndef PLMPROJECTLOADPROJECT_H
#define PLMPROJECTLOADPROJECT_H

#include <QString>
#include <QVariant>
#include <QDebug>

#include "plmtask.h"
#include "plmdata.h"
#include "plmprojectmanager.h"
class PLMProjectLoadProject : public PLMTask
{
Q_OBJECT
public:
    PLMProjectLoadProject(const QString &path) {
        setType(Setter);
        m_path = path;
//                this->connect(this, SIGNAL(projectLoaded(int)), plmdata->projectHub(),
//                        SIGNAL(projectLoaded(int)), Qt::QueuedConnection);
    }

    void doTask(bool *ok){
        emit taskStarted();


        emit taskFinished();
    }

private:
    QString m_path;

signals:
    void projectLoaded(int projectId);

};
#endif // PLMPROJECTLOADPROJECT_H



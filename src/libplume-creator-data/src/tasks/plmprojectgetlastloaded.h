#ifndef PLMPROJECTGETLASTLOADED_H
#define PLMPROJECTGETLASTLOADED_H
#include "plmtask.h"
#include "plmprojectmanager.h"

#include <QList>
#include <QVariant>



class PLMProjectGetLastLoaded : public PLMTask
{

public:
    PLMProjectGetLastLoaded() {
        setType(Getter);

    }

    void doTask(bool *ok){
        emit taskStarted();

        QList<int> list = plmProjectManager->projectIdList();
        int lastId = -1;
        if(!list.isEmpty()){
            lastId = list.last();
        }
        setReturnValue(QVariant(lastId));

        *ok = true;
        emit taskFinished();
    }

private:
};
#endif // PLMPROJECTGETLASTLOADED_H

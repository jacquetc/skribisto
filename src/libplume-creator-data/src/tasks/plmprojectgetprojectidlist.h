#ifndef PLMPROJECTGETPROJECTIDLIST_H
#define PLMPROJECTGETPROJECTIDLIST_H
#include "plmtask.h"
#include "plmdata.h"
#include "plmprojectmanager.h"

#include <QList>
#include <QString>
#include <QVariant>



class PLMProjectGetProjectIdList : public PLMTask
{

public:
    PLMProjectGetProjectIdList() {
        setType(Getter);

    }

    void doTask(bool *ok){
        emit taskStarted();

        QList<int> list = plmProjectManager->projectIdList();
        QList<QVariant> newList;
        foreach (int var, list) {
            newList.append(var);
        }
        setReturnValueList(newList);

        *ok = true;
        emit taskFinished();
    }

private:

};


#endif // PLMPROJECTGETPROJECTIDLIST_H

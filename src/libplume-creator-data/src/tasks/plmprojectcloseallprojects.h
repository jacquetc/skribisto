/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                   *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmprojectcloseallprojects.h                                 *
 *  This file is part of Plume Creator.                                    *
 *                                                                         *
 *  Plume Creator is free software: you can redistribute it and/or modify  *
 *  it under the terms of the GNU General Public License as published by   *
 *  the Free Software Foundation, either version 3 of the License, or      *
 *  (at your option) any later version.                                    *
 *                                                                         *
 *  Plume Creator is distributed in the hope that it will be useful,       *
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
 *  GNU General Public License for more details.                           *
 *                                                                         *
 *  You should have received a copy of the GNU General Public License      *
 *  along with Plume Creator.  If not, see <http://www.gnu.org/licenses/>. *
 ***************************************************************************/
#ifndef PLMPROJECTCLOSEALLPROJECTS_H
#define PLMPROJECTCLOSEALLPROJECTS_H

#include <QString>
#include <QVariant>

#include "plmtask.h"
#include "plmdata.h"
#include "plmprojectmanager.h"

class PLMProjectCloseAllProjects : public PLMTask
{

public:
    PLMProjectCloseAllProjects() {
        setType(Setter);
    }

    void doTask(bool *ok){
        *ok = true;
        emit taskStarted();

        QList<int> idList = plmProjectManager->projectIdList();
        foreach (int id, idList) {
            *ok = plmProjectManager->closeProject(id);
            emit plmdata->projectHub()->projectClosed(id);
        }
        emit plmdata->projectHub()->allProjectsClosed();

        emit taskFinished();
    }

private:

};
#endif // PLMPROJECTCLOSEALLPROJECTS_H

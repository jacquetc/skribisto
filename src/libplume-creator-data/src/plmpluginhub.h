/***************************************************************************
*   Copyright (C) 2018 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmpluginhub.h                                                   *
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
#ifndef PLMPLUGINHUB_H
#define PLMPLUGINHUB_H

#include <QObject>
#include <QVariant>
#include "plume_creator_data_global.h"

struct PLMCommand
{
    PLMCommand() {
        origin    = QString();
        cmd       = QString();
        projectId = -1;
        paperId   = -1;
        arg1      = QVariant();
        arg2      = QVariant();
        arg3      = QVariant();
    }

    PLMCommand(const QString & _origin,
               const QString & _cmd,
               int             _projectId = -1,
               int             _paperId   = -1,
               const QVariant& _arg1      = QVariant(),
               const QVariant& _arg2      = QVariant(),
               const QVariant& _arg3      = QVariant()) {
        origin    = _origin;
        cmd       = _cmd;
        projectId = _projectId;
        paperId   = _paperId;
        arg1      = _arg1;
        arg2      = _arg2;
        arg3      = _arg3;
    }

    PLMCommand(const PLMCommand& otherCommand) {
        origin    = otherCommand.origin;
        cmd       = otherCommand.cmd;
        projectId = otherCommand.projectId;
        paperId   = otherCommand.paperId;
        arg1      = otherCommand.arg1;
        arg2      = otherCommand.arg2;
        arg3      = otherCommand.arg3;
    }

    ~PLMCommand() {}

    bool operator!() const {
        return origin.isNull() || cmd.isNull();
    }

    operator bool() const {
        return !origin.isNull() && !cmd.isNull();
    }
    PLMCommand& operator=(const PLMCommand& otherCommand) {
        origin    = otherCommand.origin;
        cmd       = otherCommand.cmd;
        projectId = otherCommand.projectId;
        paperId   = otherCommand.paperId;
        arg1      = otherCommand.arg1;
        arg2      = otherCommand.arg2;
        arg3      = otherCommand.arg3;
        return *this;
    }

    bool operator==(const PLMCommand& otherCommand) const {
        if (origin != otherCommand.origin) return false;

        if (cmd != otherCommand.cmd) return false;

        if (paperId != otherCommand.paperId) return false;

        if (projectId != otherCommand.projectId) return false;

        if (arg1 != otherCommand.arg1) return false;

        if (arg2 != otherCommand.arg2) return false;

        if (arg3 != otherCommand.arg3) return false;

        return true;
    }

    QString  origin;
    QString  cmd;
    int      paperId;
    int      projectId;
    QVariant arg1;
    QVariant arg2;
    QVariant arg3;
};
Q_DECLARE_METATYPE(PLMCommand)

// --------------------------------------------------

class EXPORT PLMPluginHub : public QObject {
    Q_OBJECT

public:

    explicit PLMPluginHub(QObject *parent = nullptr);

    void reloadPlugins();

signals:

    void commandSent(const PLMCommand& command);

public slots:
};

#endif // PLMPLUGINHUB_H

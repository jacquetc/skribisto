/***************************************************************************
*   Copyright (C) 2018 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmpluginhub.h                                                   *
*  This file is part of Skribisto.                                    *
*                                                                         *
*  Skribisto is free software: you can redistribute it and/or modify  *
*  it under the terms of the GNU General Public License as published by   *
*  the Free Software Foundation, either version 3 of the License, or      *
*  (at your option) any later version.                                    *
*                                                                         *
*  Skribisto is distributed in the hope that it will be useful,       *
*  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
*  GNU General Public License for more details.                           *
*                                                                         *
*  You should have received a copy of the GNU General Public License      *
*  along with Skribisto.  If not, see <http://www.gnu.org/licenses/>. *
***************************************************************************/
#ifndef SKRPLUGINHUB_H
#define SKRPLUGINHUB_H

#include <QObject>
#include <QVariant>
#include "skribisto_data_global.h"
#include "skrresult.h"
#include "skrpluginloader.h"

struct SKRCommand
{
    SKRCommand() {
        origin    = QString();
        cmd       = QString();
        projectId = -1;
        paperId   = -1;
        arg1      = QVariant();
        arg2      = QVariant();
        arg3      = QVariant();
    }

    SKRCommand(const QString & _origin,
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

    SKRCommand(const SKRCommand& otherCommand) {
        origin    = otherCommand.origin;
        cmd       = otherCommand.cmd;
        projectId = otherCommand.projectId;
        paperId   = otherCommand.paperId;
        arg1      = otherCommand.arg1;
        arg2      = otherCommand.arg2;
        arg3      = otherCommand.arg3;
    }

    ~SKRCommand() {}

    bool operator!() const {
        return origin.isNull() || cmd.isNull();
    }

    operator bool() const {
        return !origin.isNull() && !cmd.isNull();
    }
    SKRCommand& operator=(const SKRCommand& otherCommand) {
        origin    = otherCommand.origin;
        cmd       = otherCommand.cmd;
        projectId = otherCommand.projectId;
        paperId   = otherCommand.paperId;
        arg1      = otherCommand.arg1;
        arg2      = otherCommand.arg2;
        arg3      = otherCommand.arg3;
        return *this;
    }

    bool operator==(const SKRCommand& otherCommand) const {
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
Q_DECLARE_METATYPE(SKRCommand)

// --------------------------------------------------

class EXPORT SKRPluginHub : public SKRPluginLoader {
    Q_OBJECT

public:

    explicit SKRPluginHub(QObject *parent = nullptr);

    void      reloadPlugins();
    SKRResult set(int             projectId,
                  int             id,
                  const QString & tableName,
                  const QString & fieldName,
                  const QVariant& value);
    QVariant get(int            projectId,
                 int            id,
                 const QString& tableName,
                 const QString& fieldName) const;

    QList<int>getIds(int            projectId,
                     const QString& tableName) const;
    SKRResult ensureTableExists(int            projectId,
                                const QString& tableName,
                                const QString& sqlString);

signals:

    void errorSent(const SKRResult& result) const;

    void commandSent(const SKRCommand& command);

public slots:
};

#endif // SKRPLUGINHUB_H

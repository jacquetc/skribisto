/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: trashprojecttoolbox.h
*                                                  *
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
#ifndef TRASHPROJECTTOOLBOX_H
#define TRASHPROJECTTOOLBOX_H

#include <QObject>
#include "interfaces/projecttoolboxinterface.h"

class TrashProjectToolbox : public QObject,
                                 public ProjectToolboxInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.TrashProjectToolboxPlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(ProjectToolboxInterface)

public:

    explicit TrashProjectToolbox(QObject *parent = nullptr);
    ~TrashProjectToolbox();
    QString name() const override {
        return "TrashProjectToolbox";
    }

    QString displayedName() const override {
        return tr("Trash Project Toolbox", "plugin name");
    }

    QString use() const override {
        return "Display a toolbox offering access to project trash";
    }

    QString pluginGroup() const override {
        return "ProjectToolbox";
    }

    QString pluginSelectionGroup() const override {
        return "Mandatory";
    }

    Toolbox *getToolbox() const override;

    int weight() const override {
        return 200;
    }

signals:

private:
};

#endif // TRASHPROJECTTOOLBOX_H

/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: navigationprojecttoolbox.h
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
#ifndef NAVIGATIONPROJECTTOOLBOX_H
#define NAVIGATIONPROJECTTOOLBOX_H

#include <QObject>
#include "skrprojecttoolboxinterface.h"

class NavigationProjectToolbox : public QObject,
                                 public SKRProjectToolboxInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.NavigationProjectToolboxPlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(SKRProjectToolboxInterface)

public:

    explicit NavigationProjectToolbox(QObject *parent = nullptr);
    ~NavigationProjectToolbox();
    QString name() const override {
        return "NavigationProjectToolbox";
    }

    QString displayedName() const override {
        return tr("Navigation Project Toolbox", "plugin name");
    }

    QString use() const override {
        return "Display a toolbox offering access to navigation by list";
    }

    QString pluginGroup() const override {
        return "ProjectToolbox";
    }

    QString pluginSelectionGroup() const override {
        return "Mandatory";
    }

    QString qmlUrl() const override {
        return "qrc:///eu.skribisto.skribisto/imports/qml/plugins/skribisto-plugin-navigationProjectToolbox/NavigationProjectToolbox.qml";
    }

    int weight() const override {
        return 100;
    }

signals:

private:
};

#endif // NAVIGATIONPROJECTTOOLBOX_H

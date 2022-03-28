/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: favoritesprojecttoolbox.h
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
#ifndef FAVORITESPROJECTTOOLBOX_H
#define FAVORITESPROJECTTOOLBOX_H

#include <QObject>
#include "skrprojecttoolboxinterface.h"

class FavoritesProjectToolbox : public QObject,
                                public SKRProjectToolboxInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.FavoritesProjectToolboxPlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(SKRProjectToolboxInterface)

public:

    explicit FavoritesProjectToolbox(QObject *parent = nullptr);
    ~FavoritesProjectToolbox();
    QString name() const override {
        return "FavoritesProjectToolbox";
    }

    QString displayedName() const override {
        return tr("Favorites project toolbox", "plugin name");
    }

    QString use() const override {
        return "Display a toolbox offering access to favorite items";
    }

    QString pluginGroup() const override {
        return "ProjectToolbox";
    }

    QString pluginSelectionGroup() const override {
        return "Common";
    }

    QString qmlUrl() const override {
        return "qrc:///eu.skribisto.skribisto/imports/qml/plugins/skribisto-plugin-favoritesProjectToolbox/FavoritesProjectToolbox.qml";
    }

    int weight() const override {
        return 600;
    }

signals:

private:
};

#endif // FAVORITESPROJECTTOOLBOX_H

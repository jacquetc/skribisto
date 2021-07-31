/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: antidotepagetoolbox.h
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
#ifndef ANTIDOTEPAGETOOLBOX_H
#define ANTIDOTEPAGETOOLBOX_H

#include <QObject>
#include "skrpagetoolboxinterface.h"

class AntidotePageToolbox : public QObject,
                            public SKRPageToolboxInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.AntidotePageToolboxPlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(SKRPageToolboxInterface)

public:

    explicit AntidotePageToolbox(QObject *parent = nullptr);
    ~AntidotePageToolbox();
    QString name() const override {
        return "AntidotePageToolbox";
    }

    QString displayedName() const override {
        return tr("Druide Antidote\u2122 Page Toolbox", "plugin name");
    }

    QString use() const override {
        return tr("Display a toolbox offering a quick access to Druide Antidote\u2122 (Linux only)",
                  "plugin description");
    }

    QString pluginGroup() const override {
        return "PageToolbox";
    }

    QString pluginSelectionGroup() const override {
        return "Writing";
    }

    QStringList associatedPageTypes() const override {
        QStringList list;

        list << "TEXT";
        return list;
    }

    QString qmlUrl() const override {
        return "qrc:///qml/plugins/AntidotePageToolbox/AntidotePageToolbox.qml";
    }

    int weight() const override {
        return 550;
    }

signals:

private:
};

#endif // ANTIDOTEPAGETOOLBOX_H

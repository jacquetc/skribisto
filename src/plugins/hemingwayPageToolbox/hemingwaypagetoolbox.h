/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: hemingwaypagetoolbox.h                                                   *
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
#ifndef HEMINGWAYPAGETOOLBOX_H
#define HEMINGWAYPAGETOOLBOX_H

#include <QObject>
#include "skrpagetoolboxinterface.h"

class HemingwayPageToolbox : public QObject,
//                 public SKRCoreInterface,
                 public SKRPageToolboxInterface{
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.HemingwayPageToolboxPlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(/*SKRCoreInterface, */SKRPageToolboxInterface)

public:

    explicit HemingwayPageToolbox(QObject *parent = nullptr);
    ~HemingwayPageToolbox();
    QString name() const override {
        return "HemingwayPageToolbox";
    }

    QString displayedName() const override {
        return tr("Hemingway Page Toolbox");
    }

    QString use() const override {
        return "Display a toolbox for Hemingway writing game";
    }

    QStringList  associatedPageTypes() const override {
        QStringList list;
        list << "TEXT";
        return list;
    }

    QString qmlUrl() const override {
        return "qrc:///qml/plugins/HemingwayPageToolbox/HemingwayPageToolbox.qml";
    }


signals:

private:

};

#endif // HEMINGWAYPAGETOOLBOX_H

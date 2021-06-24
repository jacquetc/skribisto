/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plugin.h
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
#ifndef PLUMECREATORIMPORTER_H
#define PLUMECREATORIMPORTER_H

#include <QObject>
#include "skrimporterinterface.h"

class Plugin : public QObject,
               public SKRImporterInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.PlumeCreatorImporterPlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(SKRImporterInterface)

public:

    explicit Plugin(QObject *parent = nullptr);
    ~Plugin();
    QString name() const override {
        return "PlumeCreatorImporter";
    }

    QString displayedName() const override {
        return tr("Plume Creator importer");
    }

    QString use() const override {
        return tr("Import a project from Plume Creator");
    }

    QString iconSource() const override {
        return "qrc:/pics/skribisto.svg";
    }

    QString buttonText() const override {
        return tr("Import a project from Plume Creator ");
    }

    QString qmlUrl() const override {
        return "qrc:///qml/plugins/PlumeCreatorImporter/PlumeCreatorImporter.qml";
    }

    int weight() const override {
        return 500;
    }

signals:

private:
};

#endif // PLUGIN_H

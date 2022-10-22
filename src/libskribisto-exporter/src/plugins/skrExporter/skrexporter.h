/***************************************************************************
 *   Copyright (C) 2022 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skrexporter.h                                                   *
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
#ifndef SKREXPORTER_H
#define SKREXPORTER_H

#include "interfaces/exporterinterface.h"
#include <QObject>

class SkrExporter : public QObject, public ExporterInterface {
  Q_OBJECT
  Q_PLUGIN_METADATA(IID "eu.skribisto.SkrExporterPlugin/1.0" FILE
                        "plugin_info.json")
  Q_INTERFACES(ExporterInterface)

public:
  explicit SkrExporter(QObject *parent = nullptr);
  ~SkrExporter();

signals:

private:


    // ExporterInterface interface
public:
    QString extension() const override
    {
        return "skrib";
    }
    QString extensionHumanName() const override
    {
        return tr("Skribisto project");
    }
    QString extensionShortName() const override
    {
        return "skrib";
    }
    SKRResult run(int projectId, const QUrl &url, const QVariantMap &parameters, QList<int> treeItemIds) const override;

    // SKRInterfaceSettings interface
public:
    QString name() const override
    {
        return "SkrExporter";
    }
    QString displayedName() const override
    {
        return tr(".skr exporter");
    }
    QString use() const override
    {
        return "Export to .skr";
    }
    QString pluginGroup() const override
    {
        return "Exporter";
    }
    QString pluginSelectionGroup() const override
    {
        return "Mandatory";
    }
};

#endif // SKREXPORTER_H

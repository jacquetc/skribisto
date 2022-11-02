/***************************************************************************
 *   Copyright (C) 2022 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skribexporter.h                                                   *
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
#ifndef SKRIBEXPORTER_H
#define SKRIBEXPORTER_H

#include "interfaces/exporterinterface.h"
#include <QObject>

class SkribExporter : public QObject, public ExporterInterface {
  Q_OBJECT
  Q_PLUGIN_METADATA(IID "eu.skribisto.SkribExporterPlugin/1.0" FILE
                        "plugin_info.json")
  Q_INTERFACES(ExporterInterface)

public:
  explicit SkribExporter(QObject *parent = nullptr);
  ~SkribExporter();

signals:

private:


    // ExporterInterface interface
public:
    int weight() const override
    {
        return 200;
    }
    QStringList extensions() const override
    {
        return  QStringList() << "skrib";
    }
    QStringList extensionHumanNames() const override
    {
        return  QStringList() << tr("Skribisto project");
    }
    QStringList extensionShortNames() const override
    {
        return QStringList() << "skrib";
    }
    bool canSave() override
    {
        return true;
    }
    SKRResult run(int projectId, const QUrl &url, const QString &extension, const QVariantMap &parameters, QList<int> treeItemIds) const override;

    // SKRInterfaceSettings interface
public:
    QString name() const override
    {
        return "SkribExporter";
    }
    QString displayedName() const override
    {
        return tr(".skrib exporter");
    }
    QString use() const override
    {
        return "Export to .skrib";
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

#endif // SKRIBEXPORTER_H

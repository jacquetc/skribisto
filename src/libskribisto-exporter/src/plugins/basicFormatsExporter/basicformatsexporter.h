/***************************************************************************
 *   Copyright (C) 2022 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: basicformatsexporter.h * This file is part of Skribisto. *
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
#ifndef BASICFORMATSEXPORTER_H
#define BASICFORMATSEXPORTER_H

#include "interfaces/exporterinterface.h"
#include <QObject>

class BasicFormatsExporter : public QObject, public ExporterInterface {
  Q_OBJECT
  Q_PLUGIN_METADATA(IID "eu.skribisto.BasicFormatsExporterPlugin/1.0" FILE
                        "plugin_info.json")
  Q_INTERFACES(ExporterInterface)

public:
  explicit BasicFormatsExporter(QObject *parent = nullptr);
  ~BasicFormatsExporter();

signals:

private:
  // ExporterInterface interface
public:
  int weight() const override { return 200; }
  QStringList extensions() const override {
    return QStringList() << "odt"
                         << "html" << "txt";
  }
  QStringList extensionHumanNames() const override {
    return QStringList() << tr("Open Document Text") << tr("HTML")
                         << tr("Plain text");
  }
  QStringList extensionShortNames() const override {
    return QStringList() << "odt"
                         << "html"
                         << "txt";
  }
  bool canSave() override { return false; }
  SKRResult run(QList<TreeItemAddress> treeItemAddresses, const QUrl &url, const QString &extension,
                const QVariantMap &parameters) const override;

  // SKRInterfaceSettings interface
public:
  QString name() const override { return "BasicFormatsExporter"; }
  QString displayedName() const override { return tr(".skrib exporter"); }
  QString use() const override { return "Export to .skrib"; }
  QString pluginGroup() const override { return "Exporter"; }
  QString pluginSelectionGroup() const override { return "Mandatory"; }
};

#endif // BASICFORMATSEXPORTER_H

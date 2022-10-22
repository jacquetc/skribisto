/***************************************************************************
 *   Copyright (C) 2022 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skrimporter.h                                                   *
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
#ifndef SKRIMPORTER_H
#define SKRIMPORTER_H

#include "interfaces/importerinterface.h"
#include <QObject>

class SkrImporter : public QObject, public ImporterInterface {
  Q_OBJECT
  Q_PLUGIN_METADATA(IID "eu.skribisto.SkrImporterPlugin/1.0" FILE
                        "plugin_info.json")
  Q_INTERFACES(ImporterInterface)

public:
  explicit SkrImporter(QObject *parent = nullptr);
  ~SkrImporter();

signals:

private:


    // SKRInterfaceSettings interface
public:
    QString name() const override
    {
        return "SkrImporter";
    }
    QString displayedName() const override
    {
        return tr(".skr importer");
    }
    QString use() const override
    {
        return "Import from .skr";
    }
    QString pluginGroup() const override
    {
        return "Importer";
    }
    QString pluginSelectionGroup() const override
    {
        return "Mandatory";
    }

    // ImporterInterface interface
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
    QString importProject(const QUrl &url, const QVariantMap &parameters, SKRResult &result) const override;
};

#endif // SKRIMPORTER_H

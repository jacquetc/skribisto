/***************************************************************************
 *   Copyright (C) 2021 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skrformat.h                                                   *
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
#ifndef SKRIBFORMAT_H
#define SKRIBFORMAT_H

#include "interfaces/newprojectformatinterface.h"
#include <QObject>

class SkribFormat : public QObject,
                  public NewProjectFormatInterface {
  Q_OBJECT
  Q_PLUGIN_METADATA(IID "eu.skribisto.SkribFormatPlugin/1.0" FILE
                        "plugin_info.json")
  Q_INTERFACES(NewProjectFormatInterface)

public:
  explicit SkribFormat(QObject *parent = nullptr);
  ~SkribFormat();
  QString name() const override { return "SkribFormat"; }

  QString displayedName() const override {
    return tr(".skrib format for new project", "plugin name");
  }

  QString use() const override { return "Allow the use of .skrib format in new projects"; }

  QString pluginGroup() const override { return "Format"; }

  QString pluginSelectionGroup() const override { return "Mandatory"; }

  // NewProjectFormatInterface interface
public:
  int weight() const override
  {
      return 100;
  }
  QString buttonText() const override
  {
      return ".skrib";
  }
  QString buttonIcon() const override
  {
      return "";
  }
  QString extension() const override
  {
      return "skrib";
  }

  QString formatDetailText() const override;
  QString finalFileName(const QString &path, const QString &fileBaseName) const override;
};

#endif // SKRIBFORMAT_H

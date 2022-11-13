/***************************************************************************
 *   Copyright (C) 2022 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: noveltemplates.h * This file is part of Skribisto. *
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
#ifndef NOVELTEMPLATES_H
#define NOVELTEMPLATES_H

#include "interfaces/newprojecttemplateinterface.h"
#include <QObject>

class NovelTemplates : public QObject, public NewProjectTemplateInterface {
  Q_OBJECT
  Q_PLUGIN_METADATA(IID "eu.skribisto.NovelTemplatesPlugin/1.0" FILE
                        "plugin_info.json")
  Q_INTERFACES(NewProjectTemplateInterface)

public:
  explicit NovelTemplates(QObject *parent = nullptr);
  ~NovelTemplates();

signals:

private:
  // SKRInterfaceSettings interface
public:
  QString name() const override { return "NovelTemplates"; }
  QString displayedName() const override { return tr("Novel templates"); }
  QString use() const override { return "Provide basic novel templates"; }
  QString pluginGroup() const override { return "NewProjectTemplate"; }
  QString pluginSelectionGroup() const override { return "Mandatory"; }

  // NewProjectTemplateInterface interface
public:
  int weight() const override
  {
      return 500;
  }
  QStringList templateNames() const override
  {
      return QStringList() << "light novel" << "novel";
  }
  QStringList templateHumanNames() const override
  {
      return QStringList() << tr("Light novel") << tr("Novel");
  }

  QStringList templateDetailText() const override;
  void applyTemplate(int projectId, const QString &templateName) override;
};

#endif // NOVELTEMPLATES_H

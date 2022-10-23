/***************************************************************************
 *   Copyright (C) 2021 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skrnewprojectformatinterfaace.h
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
#ifndef NEWPROJECTTEMPLATEINTERFACE_H
#define NEWPROJECTTEMPLATEINTERFACE_H

#include "skrcoreinterface.h"
#include <QString>
#include <QVariantMap>

class NewProjectTemplateInterface : public SKRCoreInterface {
public:
  virtual ~NewProjectTemplateInterface() {}

  virtual int weight() const = 0;
  virtual QStringList templateNames() const = 0;
  virtual QStringList templateHumanNames() const = 0;
  virtual QStringList
  templateDetailText() const = 0;
  virtual void applyTemplate(int projectId,
                             const QString &templateName) = 0;

protected:
};

#define NewProjectTemplateInterface_iid                                        \
  "com.skribisto.NewProjectTemplateInterface/1.0"

Q_DECLARE_INTERFACE(NewProjectTemplateInterface,
                    NewProjectTemplateInterface_iid)

#endif // NEWPROJECTTEMPLATEINTERFACE_H

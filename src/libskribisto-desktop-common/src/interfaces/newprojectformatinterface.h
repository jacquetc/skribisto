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
#ifndef NEWPROJECTFORMATINTERFACE_H
#define NEWPROJECTFORMATINTERFACE_H

#include "interfaces/skrcoreinterface.h"
#include <QString>
#include <QVariantMap>

class NewProjectFormatInterface : public SKRCoreInterface {
public:
  virtual ~NewProjectFormatInterface() {}

  virtual int weight() const = 0;
  virtual QString buttonText() const = 0;
  virtual QString buttonIcon() const = 0;
  virtual QString extension() const = 0;
    virtual QString formatDetailText() const = 0;
    virtual QString finalFileName(const QString &path, const QString &fileBaseName) const = 0;

protected:
};

#define NewProjectFormatInterface_iid                                         \
  "com.skribisto.NewProjectFormatInterface/1.0"

Q_DECLARE_INTERFACE(NewProjectFormatInterface, NewProjectFormatInterface_iid)

#endif // NEWPROJECTFORMATINTERFACE_H

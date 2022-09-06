/***************************************************************************
 *   Copyright (C) 2022 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: pagetoolboxinterface.h
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

#ifndef PROJECTTOOLBOXINTERFACE_H
#define PROJECTTOOLBOXINTERFACE_H

#include "skrcoreinterface.h"
#include <QString>
#include <toolbox.h>

class ProjectToolboxInterface : public SKRCoreInterface {
public:
  virtual ~ProjectToolboxInterface() {}

    virtual Toolbox * getToolbox() const = 0;
    virtual int weight() const { return 500; }
};

#define ProjectToolboxInterface_iid "com.skribisto.ProjectToolboxInterface/1.0"

Q_DECLARE_INTERFACE(ProjectToolboxInterface, ProjectToolboxInterface_iid)

#endif // PROJECTTOOLBOXINTERFACE_H

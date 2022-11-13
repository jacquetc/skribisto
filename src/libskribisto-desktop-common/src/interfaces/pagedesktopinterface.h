/***************************************************************************
 *   Copyright (C) 2021 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skrpagedesktopinterface.h
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
#ifndef PAGEDESKTOPINTERFACE_H
#define PAGEDESKTOPINTERFACE_H

#include "interfaces/skrcoreinterface.h"
#include "treeitemcreationparameterswidget.h"
#include "view.h"
#include <QString>
#include <QVariantMap>

class PageDesktopInterface {
public:
  virtual ~PageDesktopInterface() {}

    virtual QString pageType() const       = 0;

  virtual View *getView() const = 0;
  virtual TreeItemCreationParametersWidget *
  pageCreationParametersWidget() const {
    return nullptr;
  }

protected:
};

#define PageDesktopInterface_iid "com.skribisto.PageDesktopInterface/1.0"

Q_DECLARE_INTERFACE(PageDesktopInterface, PageDesktopInterface_iid)

#endif // PAGEDESKTOPINTERFACE_H

/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrpageinterface.h
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
#ifndef PAGEINTERFACE_H
#define PAGEINTERFACE_H

#include <QString>
#include <QVariantMap>
#include "treeitemcreationparameterswidget.h"
#include "view.h"
#include "skrcoreinterface.h"


class PageInterface : public SKRCoreInterface {
public:

    virtual ~PageInterface() {}

    virtual QString pageType() const       = 0;
    virtual int     weight() const         = 0;
    virtual QString visualText() const     = 0;
    virtual QString pageDetailText() const = 0;
    virtual View* getView() const        = 0;
    virtual TreeItemCreationParametersWidget * pageCreationParametersWidget() const {
        return nullptr;
    }
    virtual bool      isConstructible() const = 0;

    virtual QVariantMap propertiesForCreationOfTreeItem(const QVariantMap &customProperties = QVariantMap()) const = 0;

    virtual void      updateCharAndWordCount(int  projectId,
                                             int  treeItemId,
                                             bool sameThread = false) {}

protected:

};

#define PageInterface_iid "com.skribisto.PageInterface/1.0"


Q_DECLARE_INTERFACE(PageInterface, PageInterface_iid)

#endif // PAGEINTERFACE_H

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
#ifndef SKRPAGEINTERFACE_H
#define SKRPAGEINTERFACE_H

#include <QString>
#include <QVariantMap>
#include "skrresult.h"
#include "skrcoreinterface.h"


class SKRPageInterface : public SKRCoreInterface {
public:

    virtual ~SKRPageInterface() {}

    virtual QString pageType() const       = 0;
    virtual QString visualText() const     = 0;
    virtual QString pageDetailText() const = 0;
    virtual QString pageUrl() const        = 0;
    virtual QString pageCreationParametersUrl() const {
        return "";
    }

    void setPageCreationParameters(const QVariantMap& parametersMap) {
        m_creationParametersMap = parametersMap;
    }

    virtual bool      isConstructible() const = 0;
    virtual QString   pageTypeIconUrl() const = 0;

    virtual SKRResult finaliseAfterCreationOfTreeItem(int projectId,
                                                      int treeItemId) = 0;

    virtual void      updateCharAndWordCount(int  projectId,
                                             int  treeItemId,
                                             bool sameThread = false) {}

protected:

    QVariantMap m_creationParametersMap;
};

#define SKRPageInterface_iid "com.skribisto.PageInterface/1.0"


Q_DECLARE_INTERFACE(SKRPageInterface, SKRPageInterface_iid)

#endif // SKRPAGEINTERFACE_H

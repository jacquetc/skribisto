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
#ifndef SKRPROJECTPAGEINTERFACE_H
#define SKRPROJECTPAGEINTERFACE_H

#include <QString>
#include "skrresult.h"
#include "skrpageinterface.h"

class SKRProjectPageInterface : public SKRPageInterface {
public:

    virtual ~SKRProjectPageInterface() {}

    virtual QStringList locations() const = 0;

    virtual QString     iconSource() const = 0;

    virtual QString     showButtonText() const = 0;

    virtual QStringList shortcutSequences() const = 0;

    bool                isConstructible() const override {
        return false;
    }

    QString pageTypeIconUrl(int projectId, int treeItemId) const override {
        Q_UNUSED(projectId)
        Q_UNUSED(treeItemId)

        return "";
    }

    SKRResult finaliseAfterCreationOfTreeItem(int projectId,
                                              int treeItemId) override {
        return SKRResult();
    }

    void updateCharAndWordCount(int  projectId,
                                int  treeItemId,
                                bool sameThread = false) override {}
};

#define SKRProjectPageInterface_iid "com.skribisto.ProjectPageInterface/1.0"


Q_DECLARE_INTERFACE(SKRProjectPageInterface, SKRProjectPageInterface_iid)

#endif // SKRPROJECTPAGEINTERFACE_H

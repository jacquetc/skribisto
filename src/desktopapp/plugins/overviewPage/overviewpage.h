/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: overviewpage.h                                                   *
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
#ifndef OVERVIEWPAGE_H
#define OVERVIEWPAGE_H

#include <QObject>
#include "interfaces/projectpageinterface.h"
#include "interfaces/pagedesktopinterface.h"
#include <view.h>

class OverviewPage : public QObject,
                    public PageDesktopInterface,
                     public ProjectPageInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.OverviewPagePlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(ProjectPageInterface PageDesktopInterface)

public:

    explicit OverviewPage(QObject *parent = nullptr);
    ~OverviewPage();
    QString name() const override {
        return "OverviewPage";
    }

    QString displayedName() const override {
        return tr("Overview page", "plugin name");
    }

    QString use() const override {
        return "Display an overview of the project";
    }

    QString pluginGroup() const override {
        return "ProjectPage";
    }

    QString pluginSelectionGroup() const override {
        return "Common";
    }

    // Page
    QString pageType() const override {
        return "OVERVIEW";
    }

    int weight() const override {
        return 500;
    }

    QString visualText() const override {
        return tr("Overview");
    }

    QString pageDetailText() const override {
        return "";
    }

    View * getView() const override;


    // ---------- project page :

    QString iconSource() const override {
        return "qrc:///icons/backup/object-rows.svg";
    }

    QString showButtonText() const override {
        return tr("Show the overview");
    }

    QStringList shortcutSequences() const override {
        return QStringList() << "F5";
    }

    QStringList locations() const override {
        return QStringList() << "top-toolbar";
    }

signals:

private:
};

#endif // OVERVIEWPAGE_H

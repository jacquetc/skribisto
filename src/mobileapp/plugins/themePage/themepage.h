/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: textpage.h                                                   *
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
#ifndef TEXTPAGE_H
#define TEXTPAGE_H

#include <QObject>
#include "skrpageinterface.h"
#include "skritemexporterinterface.h"
#include "skrwordmeter.h"

class ThemePage : public QObject,
                  public SKRPageInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.ThemePagePlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(SKRPageInterface)

public:

    explicit ThemePage(QObject *parent = nullptr);
    ~ThemePage();
    QString name() const  override {
        return "ThemePage";
    }

    QString displayedName() const override {
        return tr("Theme page", "plugin name");
    }

    QString use() const override {
        return "Display a page for the theme";
    }

    QString pluginGroup() const override {
        return "Page";
    }

    QString pluginSelectionGroup() const override {
        return "Mandatory";
    }

    // Page
    QString pageType() const override {
        return "THEME";
    }

    int weight() const override {
        return 500;
    }

    QString visualText() const override {
        return tr("Themes");
    }

    QString pageDetailText() const override {
        return tr("Choose or modify your themes");
    }

    QString pageUrl() const override {
        return "qrc:///eu.skribisto.skribisto/qml/plugins/eu/skribisto/themePage/ThemePage.qml";
    }

    bool isConstructible() const  override {
        return false;
    }

    QString pageTypeIconUrl(int projectId, int treeItemId) const override {
        Q_UNUSED(projectId)
        Q_UNUSED(treeItemId)
        return "qrc:///icons/backup/color-picker-white.svg";
    }

    SKRResult finaliseAfterCreationOfTreeItem(int projectId,
                                              int treeItemId) override;

    void      updateCharAndWordCount(int  projectId,
                                     int  treeItemId,
                                     bool sameThread = false)  override;

signals:

private:
};

#endif // TEXTPAGE_H

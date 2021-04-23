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
#include "skrcoreinterface.h"
#include "skrexporterinterface.h"
#include "skrwordmeter.h"

class ThemePage : public QObject,
                  public SKRCoreInterface,
                  public SKRPageInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.ThemePagePlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(SKRCoreInterface SKRPageInterface)

public:

    explicit ThemePage(QObject *parent = nullptr);
    ~ThemePage();
    QString name() const {
        return "ThemePage";
    }

    QString displayedName() const {
        return tr("Theme page");
    }

    QString use() const {
        return "Display a page for the theme";
    }

    // Page
    QString pageType() const {
        return "THEME";
    }

    QString visualText() const {
        return tr("Text");
    }

    QString pageDetailText() const {
        return tr("Write here any text, be it a scene or a note");
    }

    QString pageUrl() const {
        return "qrc:///qml/plugins/ThemePage/ThemePage.qml";
    }

    bool isConstructible() const {
        return false;
    }

    QString pageTypeIconUrl() const {
        return "qrc:///icons/backup/color-picker-white.svg";
    }

    SKRResult finaliseAfterCreationOfTreeItem(int projectId,
                                              int treeItemId);

    void      updateCharAndWordCount(int  projectId,
                                     int  treeItemId,
                                     bool sameThread = false)  override;

signals:

private:
};

#endif // TEXTPAGE_H

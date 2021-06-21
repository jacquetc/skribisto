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
#ifndef CARDVIEWPAGE_H
#define CARDVIEWPAGE_H

#include <QObject>
#include "skrprojectpageinterface.h"

class CardViewPage : public QObject,
                     public SKRProjectPageInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.CardViewPagePlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(SKRProjectPageInterface)

public:

    explicit CardViewPage(QObject *parent = nullptr);
    ~CardViewPage();
    QString name() const override {
        return "CardViewPage";
    }

    QString displayedName() const override {
        return tr("Card view page");
    }

    QString use() const override {
        return "Display a grid-like overview";
    }

    // Page
    QString pageType() const override {
        return "CARDVIEW";
    }

    QString visualText() const override {
        return tr("Card View");
    }

    QString pageDetailText() const override {
        return "";
    }

    QString pageUrl() const override {
        return "qrc:///qml/plugins/CardViewPage/CardViewPage.qml";
    }

    // ---------- project page :

    QString iconSource() const override {
        return "qrc:///icons/backup/view-list-icons.svg";
    }

    QString showButtonText() const override {
        return tr("Show the card view");
    }

    QStringList shortcutSequences() const override {
        return QStringList() << "F6";
    }

    int weight() const override {
        return 500;
    }

    QStringList locations() const override {
        return QStringList() << "top-toolbar";
    }

signals:

private:
};

#endif // CARDVIEWPAGE_H

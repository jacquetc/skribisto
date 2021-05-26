/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: writingGamespagetoolbox.h                                                   *
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
#ifndef WRITINGGAMESPAGETOOLBOX_H
#define WRITINGGAMESPAGETOOLBOX_H

#include <QObject>
#include "skrpagetoolboxinterface.h"

class WritingGamesPageToolbox : public QObject,
                 public SKRPageToolboxInterface{
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.WritingGamesPageToolboxPlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(SKRPageToolboxInterface)

public:

    explicit WritingGamesPageToolbox(QObject *parent = nullptr);
    ~WritingGamesPageToolbox();
    QString name() const override {
        return "WritingGamesPageToolbox";
    }

    QString displayedName() const override {
        return tr("Writing Games Page Toolbox");
    }

    QString use() const override {
        return "Display a toolbox offering some writing game";
    }

    QStringList  associatedPageTypes() const override {
        QStringList list;
        list << "TEXT";
        return list;
    }

    QString qmlUrl() const override {
        return "qrc:///qml/plugins/WritingGamesPageToolbox/WritingGamesPageToolbox.qml";
    }


signals:

private:

};

#endif // WRITINGGAMESPAGETOOLBOX_H

/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: SKRTools.h                                                   *
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
#ifndef SKRQMLTOOLS_H
#define SKRQMLTOOLS_H


#include <QAccessibleEvent>
#include <QFileInfo>
#include <QKeySequence>
#include <QObject>
#include <QUrl>


class SKRQMLTools : public QObject {
    Q_OBJECT

public:

    explicit SKRQMLTools(QObject *parent = nullptr) : QObject(parent) {}

    Q_INVOKABLE QString translateURLToLocalFile(const QUrl& url) const {
        return url.toLocalFile();
    }

    Q_INVOKABLE QUrl getURLFromLocalFile(const QString& path) const {
        return QUrl::fromLocalFile(path);
    }

    Q_INVOKABLE QUrl getFolderPathURLFromURL(const QUrl& url) const {
        QFileInfo info(url.toLocalFile());
        QString   folderPath = info.path();

        return QUrl(folderPath);
    }

    Q_INVOKABLE bool isURLSchemeQRC(const QUrl& url) const {
        if (url.scheme() == "qrc") {
            return true;
        }
        return false;
    }

    Q_INVOKABLE QUrl setURLScheme(const QUrl& url, const QString& scheme) const {
        QUrl newUrl(url);

        newUrl.setScheme(scheme);

        return newUrl;
    }

    Q_INVOKABLE bool isURLValid(const QUrl& url) const {
        return url.isValid();
    }

    Q_INVOKABLE QString mnemonic(const QString& text) {
        return QKeySequence::mnemonic(text).toString();
    }

    Q_INVOKABLE QString colorString(const QColor& color) {
        return color.name().toLower();
    }
};

#endif // SKRQMLTOOLS_H

/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: SkrTools.h                                                   *
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

#ifndef SKRTOOLS_H
#define SKRTOOLS_H

#include <QObject>
#include <QString>
#include <QTranslator>
#include <QQmlEngine>
#include <QDateTime>

class SkrTools : public QObject {
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON
    Q_PROPERTY(
        QString currentTranslationLanguageCode READ getLanguageFromSettings WRITE setCurrentTranslationLanguageCode NOTIFY currentTranslationLanguageCodeChanged)

public:

    explicit SkrTools(QObject *parent = nullptr);
    Q_INVOKABLE void           setCurrentTranslationLanguageCode(
        const QString& langCode = "default");

    Q_INVOKABLE void           applyLanguageFromSettings();
    Q_INVOKABLE QString        getLanguageFromSettings() const;


    Q_INVOKABLE QString        skribistoVersion() const;
    Q_INVOKABLE QString        toLocaleDateTimeFormat(const QDateTime& dateTime) const;
    Q_INVOKABLE QString        toLocaleIntString(int number) const;

    Q_INVOKABLE QString        getQtVersion() const;
    Q_INVOKABLE bool           hasPrintSupport() const;
    Q_INVOKABLE QString        defaultFontFamily() const;

    Q_INVOKABLE QVariantMap    findAvailableTranslationsMap() const;

    Q_INVOKABLE static QString cleanUpHtml(const QString& html);

    Q_INVOKABLE bool           createPath(const QString &path);
    Q_INVOKABLE QString        getWritableAddonsPathsListDir() const;

    Q_INVOKABLE QString        getTempPath() const;

    Q_INVOKABLE QStringList    getDictFoldersFromGitHubTree(const QString& treeFile) const;

    Q_INVOKABLE void           removeFile(const QString& fileName);

    Q_INVOKABLE QString        getOnlyLanguageFromLocale(const QString& na_me) const;
    Q_INVOKABLE QString        getNativeCountryNameFromLocale(const QString& name) const;
    Q_INVOKABLE QString        getNativeLanguageNameFromLocale(const QString& name) const;

signals:

    void currentTranslationLanguageCodeChanged(const QString& langCode);

private:

    QString findTranslationDir() const;


    QString m_langCode;
    QTranslator *skribistoTranslator;
    QTranslator *qtTranslator;
    static QString trimStyle(const QString &styleValue) ;
};

#endif // SKRTOOLS_H

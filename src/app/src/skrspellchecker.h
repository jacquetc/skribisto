/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: SpellChecker.h                                                   *
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

#ifndef SKRSPELLCHECKER_H
#define SKRSPELLCHECKER_H

#include <QString>
#include <QObject>
#include <QHash>
#include <QTextStream>
#include <QStringList>


class Hunspell;

class SKRSpellChecker : public QObject {
    Q_OBJECT

    Q_PROPERTY(bool active WRITE activate READ isActive NOTIFY activated)
    Q_PROPERTY(QString langCode WRITE setLangCode READ getLangCode NOTIFY langCodeChanged)
    Q_PROPERTY(
        QStringList userDict WRITE setUserDict READ getUserDict NOTIFY userDictChanged)

public:

    SKRSpellChecker(QObject *parent = nullptr);
    ~SKRSpellChecker();
    Q_INVOKABLE void                         setDict(const QString& dictionaryPath);
    bool                                     isHunspellLaunched() const;

    Q_INVOKABLE bool                         spell(const QString& word);
    Q_INVOKABLE QStringList                  suggest(const QString& word);

    bool                                     isActive();
    Q_INVOKABLE bool                         activate(bool value = true);
    void                                     deactivate();

    Q_INVOKABLE static QStringList           dictsPaths();
    Q_INVOKABLE static QMap<QString, QString>dictAndPathMap();
    Q_INVOKABLE static QStringList           dictList();
    Q_INVOKABLE static QStringList           dictPathList();

    void                                     ignoreWord(const QString& word);
    Q_INVOKABLE void                         addWordToUserDict(const QString& word,
                                                               bool           emitSignal = true);
    Q_INVOKABLE bool                         isInUserDict(const QString& word);
    Q_INVOKABLE void                         removeWordFromUserDict(const QString& word,
                                                                    bool           emitSignal = true);
    Q_INVOKABLE void                         clearUserDict();


    Q_INVOKABLE void                         setLangCode(const QString& newLangCode);
    QString                                  getLangCode() const;

    Q_INVOKABLE void                         setUserDict(const QStringList& userDict);
    QStringList                              getUserDict() const;

signals:

    void activated(bool value);
    void langCodeChanged(QString langCode);
    void userDictChanged(QStringList userDict);

private:

    // fix bug when hunspell gives me latin1 encoded results on several Linux
    // systems :
    QString testHunspellForEncoding();
    void    addWordToDict(const QString& word);
    Hunspell *m_hunspell;
    bool m_isActive, m_hunspellLaunched;
    QStringList m_userDict;
    QString m_langCode;


    QString m_encodingFix, m_dictionaryPath;
};

#endif // SKRSPELLCHECKER_H

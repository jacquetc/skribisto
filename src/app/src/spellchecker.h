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

#ifndef SPELLCHECKER_H
#define SPELLCHECKER_H

#include <QString>
#include <QObject>
#include <QHash>
#include <QTextStream>
#include <QStringList>


class Hunspell;

class SpellChecker : public QObject
{
    Q_OBJECT
public:

    SpellChecker(QObject *parent);
    ~SpellChecker();
    void setDict(const QString &dictionaryPath, const QStringList &userDictionary, const QStringList &attendTree_names);

    bool spell(const QString &word);
    QStringList suggest(const QString &word);
    void ignoreWord(const QString &word);
    void addToUserWordlist(const QString &word);

    bool isActive();
    bool activate();
    void deactivate();

    static QStringList dictsPaths();
    static QHash<QString, QString> dictsList();

    bool isInUserWordlist(QString &word);
    void removeFromUserWordlist(const QString &word);

    // fix bug when hunspell gives me latin1 encoded results on several Linux systems :
    QString testHunspellForEncoding();

signals:
  void userDictSignal(QStringList userDict);

private:
    void put_word(const QString &word);
    Hunspell *_hunspell;
    bool m_isActive, hunspellLaunched;
    QStringList userDict;


    QString encodingFix, m_dictionaryPath;
};

#endif // SPELLCHECKER_H

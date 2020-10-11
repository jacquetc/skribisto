/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: Highlighter.h                                                   *
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

#ifndef SKRHIGHLIGHTER_H
#define SKRHIGHLIGHTER_H

#include <QSyntaxHighlighter>
#include "skrspellchecker.h"

class SKRHighlighter : public QSyntaxHighlighter
{
    Q_OBJECT

    Q_PROPERTY(SKRSpellChecker *spellChecker READ getSpellChecker)

public:
    SKRHighlighter(QTextDocument *parentDoc);
    void setTextToHighlight(QString string);
    void setCaseSensitivity(bool isCaseSensitive);
    SKRSpellChecker *getSpellChecker();

protected:
    void highlightBlock(const QString &text) override;

signals:


public slots:

private:
    void setSpellChecker(SKRSpellChecker *spellChecker);

private:
    QString textToHighLight;
    Qt::CaseSensitivity sensitivity;
    SKRSpellChecker *m_spellChecker;
    bool m_spellCheckerSet;


};

#endif // SKRHIGHLIGHTER_H

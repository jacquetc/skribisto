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

class SKRHighlighter : public QSyntaxHighlighter {
    Q_OBJECT

    Q_PROPERTY(SKRSpellChecker * spellChecker READ getSpellChecker)
    Q_PROPERTY(int projectId READ getProjectId WRITE setProjectId NOTIFY projectIdChanged)
    Q_PROPERTY(bool isForMinimap READ getIsForMinimap WRITE setIsForMinimap NOTIFY isForMinimapChanged)
    Q_PROPERTY(
        QString spellCheckHighlightColor READ spellCheckHighlightColor WRITE setSpellCheckHighlightColor NOTIFY spellCheckHighlightColorChanged)
    Q_PROPERTY(
        QString findHighlightColor READ getFindHighlightColor WRITE setFindHighlightColor NOTIFY findHighlightColorChanged)
    Q_PROPERTY(
        QString otherHighlightColor_1 READ getOtherHighlightColor_1 WRITE setOtherHighlightColor_1 NOTIFY otherHighlightColor_1Changed)
    Q_PROPERTY(
        QString otherHighlightColor_2 READ getOtherHighlightColor_2 WRITE setOtherHighlightColor_2 NOTIFY otherHighlightColor_2Changed)
    Q_PROPERTY(
        QString otherHighlightColor_3 READ getOtherHighlightColor_3 WRITE setOtherHighlightColor_3 NOTIFY otherHighlightColor_3Changed)

public:

    SKRHighlighter(QTextDocument *parentDoc);
    Q_INVOKABLE void setTextToHighlight(QString string);
    Q_INVOKABLE void setCaseSensitivity(bool isCaseSensitive);
    SKRSpellChecker* getSpellChecker();


    int              getProjectId() const;
    void             setProjectId(int projectId);

    bool             getIsForMinimap() const;
    void             setIsForMinimap(bool newIsForMinimap);

    QString          spellCheckHighlightColor() const;
    void             setSpellCheckHighlightColor(const QString& newSpellCheckHighlightColor);

    QString          getFindHighlightColor() const;
    void             setFindHighlightColor(const QString& newFindHighlightColor);

    QString          getOtherHighlightColor_1() const;
    void             setOtherHighlightColor_1(const QString& newOtherHighlightColor_1);

    QString          getOtherHighlightColor_2() const;
    void             setOtherHighlightColor_2(const QString& newOtherHighlightColor_2);

    QString          getOtherHighlightColor_3() const;
    void             setOtherHighlightColor_3(const QString& newOtherHighlightColor_3);

protected:

    void highlightBlock(const QString& text) override;

signals:

    void projectIdChanged(int projectId);
    void suggestionListChanged(QStringList list);
    void suggestionOriginalWordChanged(QString word);
    void shakeTextSoHighlightsTakeEffectCalled();
    void paintUnderlineForSpellcheckCalled(QList<int>        positionList,
                                           const QTextBlock& textBloc);
    void isForMinimapChanged(bool isForMinimap);

    void spellCheckHighlightColorChanged();

    void findHighlightColorChanged();

    void otherHighlightColor_1Changed();

    void otherHighlightColor_2Changed();

    void otherHighlightColor_3Changed();

public slots:

private:

    void setSpellChecker(SKRSpellChecker *spellChecker);

private:

    QString textToHighLight;
    Qt::CaseSensitivity sensitivity;
    SKRSpellChecker *m_spellChecker;
    bool m_spellCheckerSet;
    int m_projectId;
    QStringList m_userDictList;
    bool m_isForMinimap;
    QString m_spellCheckHighlightColor;
    QString m_findHighlightColor;
    QString m_otherHighlightColor_1;
    QString m_otherHighlightColor_2;
    QString m_otherHighlightColor_3;
};

#endif // SKRHIGHLIGHTER_H

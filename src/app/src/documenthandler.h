/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: DocumentHandler.h
*                                                  *
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

#ifndef DOCUMENTHANDLER_H
#define DOCUMENTHANDLER_H

#include <QObject>
#include <QQuickTextDocument>
#include <QTextCursor>
#include "skrhighlighter.h"



class DocumentHandler : public QObject {
    Q_OBJECT
    Q_PROPERTY(QStringList allFontFamilies READ allFontFamilies CONSTANT)

    Q_PROPERTY(
        QQuickTextDocument *
        textDocument READ textDocument WRITE setTextDocument NOTIFY textDocumentChanged)

    Q_PROPERTY(
        int cursorPosition READ cursorPosition WRITE setCursorPosition NOTIFY cursorPositionChanged)
    Q_PROPERTY(
        int selectionStart READ selectionStart WRITE setSelectionStart NOTIFY cursorPositionChanged)
    Q_PROPERTY(
        int selectionEnd READ selectionEnd WRITE setSelectionEnd NOTIFY cursorPositionChanged)

    Q_PROPERTY(QString fontFamily READ fontFamily WRITE setFontFamily NOTIFY formatChanged)
    Q_PROPERTY(qreal fontSize READ fontSize WRITE setFontSize NOTIFY formatChanged)
    Q_PROPERTY(bool italic READ italic WRITE setItalic NOTIFY formatChanged)
    Q_PROPERTY(bool bold READ bold WRITE setBold NOTIFY formatChanged)
    Q_PROPERTY(bool underline READ underline WRITE setUnderline NOTIFY formatChanged)
    Q_PROPERTY(bool strikeout READ strikeout WRITE setStrikeout NOTIFY formatChanged)
    Q_PROPERTY(QColor color READ color WRITE setColor NOTIFY formatChanged)
    Q_PROPERTY(
        Qt::Alignment alignment READ alignment WRITE setAlignment NOTIFY formatChanged)
    Q_PROPERTY(bool bulletList READ bulletList WRITE setBulletList NOTIFY formatChanged)
    Q_PROPERTY(
        bool numberedList READ numberedList WRITE setNumberedList NOTIFY formatChanged)

    Q_PROPERTY(bool canUndo READ canUndo NOTIFY canUndoChanged)
    Q_PROPERTY(bool canRedo READ canRedo NOTIFY canRedoChanged)
    Q_PROPERTY(int paperId READ paperId NOTIFY idChanged)
    Q_PROPERTY(int projectId READ projectId WRITE setProjectId NOTIFY idChanged)

    Q_PROPERTY(
        qreal topMarginEverywhere READ topMarginEverywhere WRITE setTopMarginEverywhere NOTIFY topMarginEverywhereChanged)
    Q_PROPERTY(
        qreal indentEverywhere READ indentEverywhere WRITE setIndentEverywhere NOTIFY indentEverywhereChanged)

    Q_PROPERTY(QStringList suggestionList READ suggestionList NOTIFY suggestionListChanged)
    Q_PROPERTY(QString suggestionOriginalWord READ suggestionOriginalWord NOTIFY suggestionOriginalWordChanged)

    Q_PROPERTY(SKRHighlighter * highlighter READ getHighlighter)

public:

    DocumentHandler(QObject *parent = nullptr);

    QStringList         allFontFamilies() const;

    QQuickTextDocument* textDocument() const;
    void                setTextDocument(QQuickTextDocument *textDocument);

    int                 cursorPosition() const;
    void                setCursorPosition(int position);

    int                 selectionStart() const;
    void                setSelectionStart(int selectionStart);

    int                 selectionEnd() const;
    void                setSelectionEnd(int selectionEnd);

    QString             fontFamily() const;
    void                setFontFamily(const QString& fontFamily);

    qreal               fontSize() const;
    void                setFontSize(qreal fontSize);

    bool                italic() const;
    void                setItalic(bool italic);

    bool                bold() const;
    void                setBold(bool bold);

    bool                underline() const;
    void                setUnderline(bool underline);

    bool                strikeout() const;
    void                setStrikeout(bool strikeout);

    QColor              color() const;
    void                setColor(const QColor& color);

    bool                canUndo() const;
    bool                canRedo() const;

    Qt::Alignment       alignment() const;
    void                setAlignment(Qt::Alignment alignment);

    bool                bulletList() const;
    void                setBulletList(bool bulletList);

    bool                numberedList() const;
    void                setNumberedList(bool numberedList);

    Q_INVOKABLE void                setId(const int projectId,
                              const int paperId);
    int                 paperId() const;

    int                 projectId() const;
    void                setProjectId(const int projectId);

    Q_INVOKABLE int     maxCursorPosition() const;

    qreal               topMarginEverywhere() const;
    void                setTopMarginEverywhere(qreal topMargin);
    qreal               indentEverywhere() const;
    void                setIndentEverywhere(qreal indent);

    SKRHighlighter    * getHighlighter() {
        return m_highlighter;
    }

    Q_INVOKABLE QString getHtmlAtSelection(int start, int end);

    Q_INVOKABLE void insertHtml(int position, const QString &html);


    Q_INVOKABLE bool isWordMisspelled(int cursorPosition);
    Q_INVOKABLE void listAndSendSpellSuggestions(int cursorPosition);

    QString suggestionOriginalWord() const;

    QStringList suggestionList() const;

    Q_INVOKABLE void decrementHeadingLevel();
    Q_INVOKABLE void incrementHeadingLevel();
    void setHeadingLevel(int headingLevel);
    Q_INVOKABLE void removeHeadingLevel();
    int headingLevel();
public slots:
    void replaceWord(const QString &word, const QString &newWord);
    void learnWord(const QString &word);

    void addHorizontalLine();
    void indentBlock();
    void unindentBlock();

    void undo();
    void redo();
    void insertDocumentFragment(const QTextDocumentFragment &fragment);

signals:

    void textDocumentChanged();
    void cursorPositionChanged();
    void formatChanged();

    void canUndoChanged(bool canUndo);
    void canRedoChanged(bool canRedo);
    void idChanged();
    void projectIdChanged(int projectId);
    void topMarginEverywhereChanged(qreal topMargin);
    void indentEverywhereChanged(qreal indent);

    void charCountChanged(int count);
    void wordCountChanged(int count);


    void suggestionListChanged(const QStringList &list);
    void suggestionOriginalWordChanged(const QString &word);
private:

    QQuickTextDocument *m_textDoc;
    QTextCursor m_textCursor;
    QTextCursor m_selectionCursor;

    QTextCharFormat m_nextFormat;
    int m_formatPosition;
    int m_selectionStart;
    int m_selectionEnd;
    int m_projectId;
    int m_paperId;
    qreal m_topMarginEverywhere;
    qreal m_indentEverywhere;

    SKRHighlighter *m_highlighter;

    QString m_suggestionOriginalWord;
    QStringList m_suggestionList;

};

#endif // DOCUMENTHANDLER_H

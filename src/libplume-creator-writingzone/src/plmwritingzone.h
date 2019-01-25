/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                   *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmdata.h                                 *
*  This file is part of Plume Creator.                                    *
*                                                                         *
*  Plume Creator is free software: you can redistribute it and/or modify  *
*  it under the terms of the GNU General Public License as published by   *
*  the Free Software Foundation, either version 3 of the License, or      *
*  (at your option) any later version.                                    *
*                                                                         *
*  Plume Creator is distributed in the hope that it will be useful,       *
*  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
*  GNU General Public License for more details.                           *
*                                                                         *
*  You should have received a copy of the GNU General Public License      *
*  along with Plume Creator.  If not, see <http://www.gnu.org/licenses/>. *
***************************************************************************/
#ifndef PLMWRITINGZONE_H
#define PLMWRITINGZONE_H

#include <QWidget>
#include <QImage>
#include <QTextDocument>
#include "global_writingzone.h"

namespace Ui {
class PLMWritingZone;
}

class EXPORT_WRITINGZONE PLMWritingZone : public QWidget {
    Q_OBJECT

public:

    Q_PROPERTY(bool hasMinimap READ hasMinimap WRITE setHasMinimap DESIGNABLE true
               FINAL)
    Q_PROPERTY(bool isResizable READ isResizable WRITE setIsResizable DESIGNABLE true)
    Q_PROPERTY(
        bool hasSideToolBar READ hasSideToolBar WRITE setHasSideToolBar DESIGNABLE true)
    Q_PROPERTY(
        bool hasScrollbar READ hasScrollbar WRITE setHasSideToolBar DESIGNABLE true)

    explicit PLMWritingZone(QWidget *parent = nullptr);
    ~PLMWritingZone();

    void           setUse(const QString& use);

    bool           hasMinimap() const;
    void           setHasMinimap(bool value);

    bool           hasScrollbar() const;
    void           setHasScrollbar(bool hasScrollbar);

    bool           hasSideToolBar() const;
    void           setHasSideToolBar(bool hasSideToolBar);

    bool           isResizable() const;
    void           setIsResizable(bool isResizable);

    void           setFixedWidth(int width);

    bool           isMarkdown() const;
    void           setIsMarkdown(bool isMarkdown);

    QString        markdownText() const;
    void           setMarkdownText(const QString& markdownText);

    QString        htmlText() const;
    void           setHtmlText(const QString& htmlText);

    QTextDocument* textDocument();
    void           setTextDocument(QTextDocument *textDocument);

    // virtual QImage image(const QString& imageName) const = 0;

signals:

    void scrollBarValueChanged(int value);
    void cursorPositionChanged(int value);
    void focused();

public slots:

private slots:

    void widenTextEdit(int diff);

private:

    Ui::PLMWritingZone *ui;

    QString m_use;
    bool m_hasMinimap, m_hasScrollbar, m_hasSideToolBar, m_isResizable,
         m_isMarkdown;
    QString m_markdownText, m_htmlText;
    QTextDocument *m_textDocument;
    int m_cursorPosition, m_scrollBarValue;
    int m_fixedWidth;
};

#endif // PLMWRITINGZONE_H

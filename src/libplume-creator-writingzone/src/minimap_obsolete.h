/***************************************************************************
 *   Copyright (C) 2017 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: minimap.h                                                   *
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
#ifndef MINIMAP_H
#define MINIMAP_H

#include <QGraphicsView>
#include <QGraphicsItem>
#include <QGraphicsSceneMouseEvent>
#include <QTextEdit>
#include <QScrollBar>
#include <QThread>
#include <QTextLine>

#include "cursor.h"

class DocItem;
class ImageGenerator;

class Minimap : public QGraphicsView
{
    Q_OBJECT

public:
    explicit Minimap(QWidget *parent);

    void setTextEdit(QTextEdit *textEdit);
    QTextEdit *texEdit();
    void setCursorMoved(bool isCursorMoved);
    void setScrollbarValue(int value);

    bool isCursorMoved() const;

public slots:
    void update();
protected:
    void resizeEvent(QResizeEvent *event);
private slots:
    void updateWith(QPixmap pixmap);
    void changeCursorPos();
private:
    QGraphicsScene *m_scene;
    Cursor *m_navCursor;
    QList<QGraphicsItem *> m_graphicsItemList;
    QScrollBar *m_scrollbar;
    QPixmap m_pixmap;
    QTextEdit *m_textEdit;
    ImageGenerator *m_imgGenerator;
    void generateImage(QTextEdit *textEdit);
};

//-----------------------------------------------------------------
//-----------------------------------------------------------------
//-----------------------------------------------------------------



//-----------------------------------------------------------------
//-----------------------------------------------------------------
//-----------------------------------------------------------------

class DocItem  : public QObject
{
public:
    explicit DocItem(QObject *parent);
    explicit DocItem(DocItem *other);
    ~DocItem();


    QByteArray hash();

    QString blockText() const;
    void setBlockText(const QString &blockText);

    QTextFormat textFormat() const;
    void setTextFormat(const QTextFormat &textFormat);

    int index() const;
    void setIndex(int index);

    QImage image();

    int width() const;
    void setWidth(int width);

    QList<QTextLine> getLines() const;
    void setLines(const QList<QTextLine> &value);


private:
    QImage generateImage();
    QString preparedText() const;

    int m_charHeight;
    int m_charWidth;
    int m_spaceBetweenLines;
    QByteArray m_hash;
    QString m_blockText;
    QTextFormat m_textFormat;
    int m_index;
    QImage m_image;
    int m_width;
    QList<QTextLine> m_lines;
    void drawChar(QPainter &painter, int position, const QColor &penColor);
};


//-----------------------------------------------------------------
//-----------------------------------------------------------------
//-----------------------------------------------------------------

class ImageGenerator : public QThread
{
    Q_OBJECT
public:
    explicit ImageGenerator();
    void setItems(const QList<DocItem *> &itemList);
protected:
    void run();

signals:
    void imageGenerated(QPixmap);

private:
    QList<DocItem *> m_itemList;
    QList<DocItem *> m_oldItemList;
    QHash<QByteArray, DocItem *> m_hashItemDict;
    QHash<QByteArray, DocItem *> m_oldHashItemDict;
    int m_maxWidthFound;


};
#endif // MINIMAP_H

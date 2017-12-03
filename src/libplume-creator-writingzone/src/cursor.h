/***************************************************************************
 *   Copyright (C) 2017 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: cursor.h                                                   *
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
#ifndef CURSOR_H
#define CURSOR_H

#include <QGraphicsItem>
#include <QGraphicsSceneMouseEvent>


class Cursor : public QGraphicsItem
{
public:
    explicit Cursor();
    void setWidth(int value);

    void setHeight(int value);
    int height();
    void setTextEditHeight(int textEditHeight);

    void setMinimapHeight(int minimapHeight);

    void setDocHeight(int docHeight);

    //void setGraphicsView(Minimap *graphicsView);

    void setPosRatio(double posRatio);

    bool isCursorMoved() const;
    void setIsCursorMoved(bool isCursorMoved);

    int scrollbarValue() const;
    void setScrollbarValue(int scrollbarValue);

protected:
    void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget = Q_NULLPTR);
    QRectF boundingRect() const;
    void mousePressEvent(QGraphicsSceneMouseEvent *event);
    void mouseReleaseEvent(QGraphicsSceneMouseEvent *event);
    void hoverLeaveEvent(QGraphicsSceneMouseEvent *event);
    void mouseMoveEvent(QGraphicsSceneMouseEvent *event);


private:
    int m_width, m_height, m_textEditHeight, m_minimapHeight, m_docHeight;
    bool m_isCursorMoved;
    double m_posRatio;
    int m_scrollbarValue;
    //Minimap *m_graphicsView;

};
#endif // CURSOR_H

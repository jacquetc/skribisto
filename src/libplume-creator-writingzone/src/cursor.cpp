/***************************************************************************
 *   Copyright (C) 2017 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: cursor.cpp                                                   *
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
#include "cursor.h"

#include <QCursor>
#include <QPainter>


Cursor::Cursor()
{
    m_width = 1;
    m_height = 1;
    m_textEditHeight = 1;
    m_minimapHeight = 1;
    m_docHeight = 1;
    m_posRatio = 1;
    m_isCursorMoved = false;
    //m_graphicsView = 0;
    this->setCursor(QCursor(Qt::OpenHandCursor));
    this->setFlag(QGraphicsItem::ItemIsMovable, true);
}
//-----------------------------------------------------------------
void Cursor::setWidth(int value)
{
    this->prepareGeometryChange();
    m_width = value;
}
//-----------------------------------------------------------------
void Cursor::setHeight(int value)
{
    this->prepareGeometryChange();
    m_height = value;
}
//-----------------------------------------------------------------
int Cursor::height()
{
    return m_height;
}

void Cursor::setTextEditHeight(int textEditHeight)
{
    m_textEditHeight = textEditHeight;
}

void Cursor::setMinimapHeight(int minimapHeight)
{
    m_minimapHeight = minimapHeight;
}

void Cursor::setDocHeight(int docHeight)
{
    m_docHeight = docHeight;
}

void Cursor::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
    Q_UNUSED(option)
    Q_UNUSED(widget)
    QPen pen(Qt::red);
    pen.setWidth(2);
    painter->setPen(pen);
    painter->drawRoundedRect(1, 1, m_width - 3, m_height - 3, 5, 5);
}

QRectF Cursor::boundingRect() const
{
    return QRectF(0, 0, m_width, m_height);
}

void Cursor::mousePressEvent(QGraphicsSceneMouseEvent *event)
{
    if (event->buttons() == Qt::LeftButton) {
        this->setCursor(QCursor(Qt::ClosedHandCursor));
        m_isCursorMoved = true;
    }
}

void Cursor::mouseReleaseEvent(QGraphicsSceneMouseEvent *event)
{
    this->setCursor(QCursor(Qt::OpenHandCursor));
    m_isCursorMoved = false;
}

void Cursor::hoverLeaveEvent(QGraphicsSceneMouseEvent *event)
{
    this->setCursor(QCursor(Qt::OpenHandCursor));
    m_isCursorMoved = false;
}

void Cursor::mouseMoveEvent(QGraphicsSceneMouseEvent *event)
{
    m_isCursorMoved = true;
    QGraphicsItem::mouseMoveEvent(event);

    if (event->buttons() == Qt::LeftButton) {
        int maxYPos = m_minimapHeight - m_height;
// fix vertical :
        this->setX(0);

        if (this->y() < 0) {
            this->setY(0);
        } else if (this->y() > maxYPos && maxYPos < 0) {
            this->setY(0);
        } else if (this->y() > maxYPos && maxYPos > 0) {
            this->setY(maxYPos);
        }

// move textedit's scrollbar
        //qDebug() << "y()" << QString::number(this->y());
        //print("self.pos_ratio", self.pos_ratio)
        //print("/", int(self.y() / self.pos_ratio))
        double cursorRatio = this->y() / m_minimapHeight;
        //qDebug() << "cursorRatio" << QString::number(cursorRatio);
        double r = this->y() + this->height() * cursorRatio;
        //print("r", r)
        m_scrollbarValue = int(r / m_posRatio);
    }
}

int Cursor::scrollbarValue() const
{
    return m_scrollbarValue;
}

void Cursor::setScrollbarValue(int scrollbarValue)
{
    m_scrollbarValue = scrollbarValue;
}

bool Cursor::isCursorMoved() const
{
    return m_isCursorMoved;
}

void Cursor::setIsCursorMoved(bool isCursorMoved)
{
    m_isCursorMoved = isCursorMoved;
}

void Cursor::setPosRatio(double posRatio)
{
    m_posRatio = posRatio;
}

//void Cursor::setGraphicsView(Minimap *graphicsView)
//{
//    m_graphicsView = graphicsView;
//}

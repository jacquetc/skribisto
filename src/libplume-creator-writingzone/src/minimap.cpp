/***************************************************************************
 *   Copyright (C) 2017 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: minimap.cpp                                                   *
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
#include "minimap.h"

Minimap::Minimap(QWidget *parent) : QGraphicsView(parent)
{
}

Cursor::Cursor(QGraphicsItem *parent) : QGraphicsObject(parent)
{
    m_width = 10;
    m_height = 10;
    m_textEditHeight = 1;
    m_minimapHeight = 1;
    m_docHeight = 1;
    m_posRatio = 1;
    m_graphicsView = Q_NULLPTR;
    this->setCursor(Qt::OpenHandCursor);
    this->setFlag(QGraphicsItem::ItemIsMovable, true);
}

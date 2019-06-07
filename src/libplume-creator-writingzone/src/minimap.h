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

#include <QObject>
#include <QGraphicsView>
#include <QTextBrowser>
#include <QGraphicsObject>



class Minimap : public QGraphicsView
{
public:
    Minimap(QWidget *parent);
};

class TextBrowser : public QTextBrowser
{
public:
    TextBrowser(QWidget *parent);
};

class MCursor : public QGraphicsObject
{

public:
    MCursor(QGraphicsItem *parent = nullptr);

private:
    int m_width;
    int m_height;
    int m_textEditHeight;
    int m_minimapHeight;
    int m_docHeight;
    int m_posRatio;
    QGraphicsView *m_graphicsView;

signals:

};

#endif // MINIMAP_H

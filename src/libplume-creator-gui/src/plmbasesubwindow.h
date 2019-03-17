/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmbasesubwindow.h
*                                                  *
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
#ifndef PLMBASESUBWINDOW_H
#define PLMBASESUBWINDOW_H
#include <QMainWindow>
#include <QDebug>

class PLMBaseSubWindow : public QMainWindow {
    Q_OBJECT

public:

    PLMBaseSubWindow(QWidget *parent = nullptr);

protected:

    int id() const
    {
        return m_id;
    }

    void setId(int id)
    {
        m_id = id;
    }

    void mousePressEvent(QMouseEvent *event);

signals:

    void subWindowClosed(int id);
    void splitCalled(Qt::Orientation orientation,
                     int             id);

    void subWindowFocusActived(int id);

private:

    int m_id;
};

#endif // PLMBASESUBWINDOW_H

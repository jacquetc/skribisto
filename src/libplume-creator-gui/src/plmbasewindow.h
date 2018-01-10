/***************************************************************************
 *   Copyright (C) 2018 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmbasewindow.h                                                   *
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
#ifndef PLMBASEWINDOW_H
#define PLMBASEWINDOW_H

#include <QCloseEvent>
#include <QMainWindow>

class PLMBaseWindow : public QMainWindow
{
    Q_OBJECT
public:
    explicit PLMBaseWindow(QWidget *parent = nullptr);

protected:
    void closeEvent(QCloseEvent *event);

signals:
    void attachmentCalled(const QString &windowName);

public slots:
};

#endif // PLMBASEWINDOW_H

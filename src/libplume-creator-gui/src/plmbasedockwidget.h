/***************************************************************************
*   Copyright (C) 2018 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmbasewidget.h                                                   *
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
#ifndef PLMBASEDOCKWIDGET_H
#define PLMBASEDOCKWIDGET_H

#include <QWidget>
#include <QString>
#include "global.h"

class EXPORT_GUI PLMBaseDockWidget : public QWidget {
    Q_OBJECT

public:

    explicit PLMBaseDockWidget(QWidget *parent = nullptr);
    virtual QString name()          = 0;
    virtual QString displayedName() = 0;

public slots:

    virtual void applySettings();

signals:

public slots:
};

#endif // PLMBASEDOCKWIDGET_H

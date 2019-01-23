/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: writingwindow.cpp
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
#include "plmwritingwindow.h"
#include "ui_plmwritingwindow.h"

PLMWritingWindow::PLMWritingWindow(int id) : QMainWindow(), ui(new Ui::PLMWritingWindow),
    m_id(id)
{
    ui->setupUi(this);

    ui->writingZone->setHasMinimap(false);
    ui->writingZone->setIsResizable(true);

    this->setupActions();
}

PLMWritingWindow::~PLMWritingWindow()
{
    delete ui;
}

void PLMWritingWindow::setupActions()
{
    ui->closeWindowButton->setDefaultAction(ui->actionClose);


    connect(ui->actionClose,
            &QAction::triggered,
            [ = ]() {
        emit writingWindowClosed(this->id());
    });

    ui->splitButton->addAction(ui->actionSplitVertically);
    ui->splitButton->addAction(ui->actionSplitHorizontally);

    connect(ui->actionSplitVertically,
            &QAction::triggered,
            [ = ]() {
        emit splitCalled(Qt::Vertical, this->id());
    });

    connect(ui->actionSplitHorizontally,
            &QAction::triggered,
            [ = ]() {
        emit splitCalled(Qt::Horizontal, this->id());
    });
}

int PLMWritingWindow::id() const
{
    return m_id;
}

void PLMWritingWindow::setId(int id)
{
    m_id = id;
}

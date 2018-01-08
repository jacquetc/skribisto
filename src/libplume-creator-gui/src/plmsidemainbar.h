/***************************************************************************
*   Copyright (C) 2017 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmsidemainbar.h                                                   *
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
#ifndef PLMSIDEMAINBAR_H
#define PLMSIDEMAINBAR_H

#include <QAction>
#include <QWidget>

struct PLMSideBarAction {
public:
    explicit PLMSideBarAction(const QString &panelName, QAction *action)
    {
        this->m_panelName = panelName;
        this->m_action = action;
    }

    explicit PLMSideBarAction(const PLMSideBarAction &sideBarAction)
    {
        this->m_panelName = sideBarAction.panelName();
        this->m_action = sideBarAction.action();
    }
    QString panelName() const;
    void setPanelName(const QString &panelName);

    QAction *action() const;
    void setAction(QAction *action);

private:
    QString m_panelName;
    QAction *m_action;
};

namespace Ui
{
class PLMSideMainBar;
}

class PLMSideMainBar : public QWidget
{
    Q_OBJECT

public:
    explicit PLMSideMainBar(QWidget *parent = nullptr);

signals:

public slots:
private:
    Ui::PLMSideMainBar *ui;
    void loadPlugins();
};

#endif // PLMSIDEMAINBAR_H

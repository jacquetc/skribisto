/***************************************************************************
*   Copyright (C) 2017 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmsidemainbar.h
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
#ifndef PLMSIDEMAINBAR_H
#define PLMSIDEMAINBAR_H

#include <QAction>
#include <QWidget>
#include <QToolButton>
#include "global.h"

struct EXPORT_GUI PLMSideBarAction {
public:

    explicit PLMSideBarAction(const QString& windowName, QAction *action)
    {
        this->m_windowName = windowName;
        this->m_action     = action;
    }

    explicit PLMSideBarAction(const PLMSideBarAction& sideBarAction)
    {
        this->m_windowName = sideBarAction.windowName();
        this->m_action     = sideBarAction.action();
    }

    QString  windowName() const;
    void     setWindowName(const QString& windowName);

    QAction* action() const;
    void     setAction(QAction *action);

public slots:

private:

    QString  m_windowName;
    QAction *m_action;
};


namespace Ui {
class PLMSideMainBar;
}

class EXPORT_GUI PLMSideMainBar : public QWidget {
    Q_OBJECT

public:

    explicit PLMSideMainBar(QWidget *parent = nullptr);
    void setButtonChecked(const QString& windowName);

signals:

    void windowRaiseCalled(QString windowName);
    void windowDetachmentCalled(QString windowName);
    void windowAttachmentCalled(QString windowName);

public slots:

    void attachWindowByName(const QString& windowName);

    void readSettings();

private slots:

    void raiseWindow(bool checked);
    void showContextMenu(const QPoint& pos);
    void attachWindow();
    void detachWindow();

    void init();

    void buttonChecked();

private:

    Ui::PLMSideMainBar *ui;
    void loadPlugins();
    QActionGroup *actionGroup;
    QToolButton *m_currentButton;
    QHash<QString, QToolButton *>hash_windowNameAndButton;
};

#endif // PLMSIDEMAINBAR_H

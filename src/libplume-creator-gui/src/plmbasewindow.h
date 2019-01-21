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
#include "global.h"
#include "plmbasedock.h"
#include "plmdockwidgetinterface.h"

class EXPORT_GUI PLMBaseWindow : public QMainWindow {
    Q_OBJECT

public:

    explicit PLMBaseWindow(QWidget       *parent,
                           const QString& name);

    ~PLMBaseWindow();

    bool    detached() const;
    void    setDetached(bool detached);


    void    setForceTrueClosing(bool forceTrueClosing);


    QString name() const;

public slots:

    virtual PLMBaseDock* addRightDock();
    virtual PLMBaseDock* addBottomDock();
    virtual PLMBaseDock* addLeftDock();

    void                 applyStyleSheet();

protected:

    QList<PLMBaseDock *>rightDocks() const;
    QList<PLMBaseDock *>bottomDocks() const;
    QList<PLMBaseDock *>leftDocks() const;

    void                closeEvent(QCloseEvent *event);
    void                moveEvent(QMoveEvent *event);

    int                 rightDockDefaultCount() const;
    void                setRightDockDefaultCount(int rightDockDefaultCount);

    int                 bottomDockDefaultCount() const;
    void                setBottomDockDefaultCount(int bottomDockDefaultCount);

    int                 leftDockDefaultCount() const;
    void                setLeftDockDefaultCount(int leftDockDefaultCount);

protected slots:

    void setLeftSidebarVisible(bool value);
    void setBottomSidebarVisible(bool value);
    void setRightSidebarVisible(bool value);

signals:

    void attachmentCalled(const QString& windowName);

private slots:

    void saveDockSettings();
    void saveSettingsGeometry();
    void applyDockSettings();

private:

    void applySettingsState();
    void saveSettingsState();

    void applySettingsGeometry();
    void loadPlugins();

private:

    QString m_name;

    bool m_detached, m_forceTrueClosing;

    int m_rightDockDefaultCount, m_bottomDockDefaultCount, m_leftDockDefaultCount;
    QList<PLMBaseDock *>m_leftDocks;
    QList<PLMBaseDock *>m_bottomDocks;
    QList<PLMBaseDock *>m_rightDocks;
    QList<PLMDockWidgetInterface *>m_DockPluginList;
};

#endif // PLMBASEWINDOW_H

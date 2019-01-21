/***************************************************************************
*   Copyright (C) 2018 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmbaseleftdock.h
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
#ifndef PLMBASELEFTDOCK_H
#define PLMBASELEFTDOCK_H

#include <QDockWidget>
#include <QObject>

// #include "plmbasedockwidget.h"
#include "plmdockwidgetinterface.h"

struct PLMWidgetContainer : public QObject
{
    Q_OBJECT

public:

    PLMWidgetContainer(QWidget *parent, PLMDockWidgetInterface *dockPlugin) : QObject(
            parent), m_parent(parent) {
        m_plugin = dockPlugin;
    }

    ~PLMWidgetContainer() {
        if (dockHeader) dockHeader->deleteLater();

        if (dockBody) dockBody->deleteLater();
    }

    QString                    name() const;

    QString                    displayedName() const;

    QPointer<QWidget>          getDockHeader();

    QPointer<PLMBaseDockWidget>getDockBody();
    QWidget                  * getParent() {
        return m_parent;
    }

private:

    PLMDockWidgetInterface *m_plugin;


    QString                    m_name, m_displayedName;
    QPointer<QWidget>          dockHeader;
    QPointer<PLMBaseDockWidget>dockBody;
    QWidget                   *m_parent;
};


namespace Ui {
class PLMBaseDockHeader;
class PLMBaseDockBody;
}

class PLMBaseDock : public QDockWidget {
    Q_OBJECT

public:

    PLMBaseDock(QWidget       *parent,
                Qt::Edge       edge,
                const QString& defaultWidget);
    ~PLMBaseDock();
    QString  getCurrentWidgetName() const;
    void     setCurrentWidgetByName(const QString& widgetName);


    Qt::Edge getEdge() const;

    void     addWidgetContainer(const QString         & name,
                                PLMDockWidgetInterface *dockPlugin);


    QString getDefaultWidgetName() const;
    void    setDefaultWidgetName(const QString& defaultWidgetName);

protected:

    QString getDockSettingName() const;

signals:

    void addDockCalled();
    void currentWidgetChanged(const QString& name);

private slots:

    void changeCurrentWidget(const QString& text);
    void applyCurrentWidgetSetting();

private:

    Ui::PLMBaseDockHeader *headerUi;
    Ui::PLMBaseDockBody *bodyUi;
    QHash<QString, PLMWidgetContainer *>m_widgetContainerHash;
    Qt::Edge m_edge;
    int m_dockId;
    QHash<QString, QWidget *>m_widgetHeadersHash;
    bool m_containerIsBeingAdded;
    QString m_defaultWidgetName;
};


#endif // PLMBASELEFTDOCK_H

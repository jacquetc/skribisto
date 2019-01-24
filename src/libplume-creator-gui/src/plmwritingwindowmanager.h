/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmwritingwindowmanager.h
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
#ifndef PLMWRITINGWINDOWMANAGER_H
#define PLMWRITINGWINDOWMANAGER_H

#include "plmwritingwindow.h"

#include <QObject>
#include <QPointer>

#include <QBoxLayout>
#include <QSplitter>

class WindowContainer {
    Q_GADGET

public:

    WindowContainer();
    WindowContainer(const WindowContainer& otherContainer);
    ~WindowContainer();
    bool                       operator!() const;
    operator bool() const;
    WindowContainer          & operator=(const WindowContainer& otherContainer);
    bool                       operator==(const WindowContainer& otherContainer) const;

    QPointer<PLMWritingWindow> window() const;
    void                       setWindow(const QPointer<PLMWritingWindow>& window);

    int                        parentId() const;
    void                       setParentId(int value);

    int                        id() const;
    void                       setId(int value);

    QString                    name() const;
    bool                       setName(const QString& name);

    Qt::Orientation            orientation() const;
    void                       setOrientation(const Qt::Orientation& orientation);

    void                       setOrientation(const QString& orientation);
    QList<QPointer<QSplitter> >splitterList() const;
    void                       addSplitter(const QPointer<QSplitter>& layout);

    void                       setSplitterList(
        const QList<QPointer<QSplitter> >& layoutList);

    QList<int>                 splitterSizes() const;
    void                       setSplitterSizes(QList<int>sizes);

private:

    QPointer<PLMWritingWindow>m_window;
    int m_parentId;
    int m_id;
    Qt::Orientation m_orientation;
    QList<QPointer<QBoxLayout> >m_layoutList;
    QList<QPointer<QSplitter> >m_splitterList;
};
Q_DECLARE_METATYPE(WindowContainer)

class PLMWritingWindowManager : public QObject {
    Q_OBJECT

public:

    explicit PLMWritingWindowManager(QWidget       *parent,
                                     const QString& objectName);
    ~PLMWritingWindowManager();

signals:

public slots:

    PLMWritingWindow* addWritingWindow(Qt::Orientation orientation,
                                       int             parentZone,
                                       int             forcedId = -2);

    void applySettings();
    void writeSettings();

private slots:

    void closeWritingWindow(int id);

private:

    QList<WindowContainer>m_windowContainerList;
};

#endif // PLMWRITINGWINDOWMANAGER_H

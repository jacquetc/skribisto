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
#ifndef PLMSUBWINDOWMANAGER_H
#define PLMSUBWINDOWMANAGER_H

#include "plmbasesubwindow.h"

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

    QPointer<PLMBaseSubWindow> window() const;
    void                       setWindow(const QPointer<PLMBaseSubWindow>& window);

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
    void                       addSplitter(const QPointer<QSplitter>& splitter);
    void                       cleanSplitters();
    void                       setSplitterList(
        const QList<QPointer<QSplitter> >& splitterList);

    QList<int>                 splitterSizes() const;
    void                       setSplitterSizes(QList<int>sizes);

    QString                    subWindowType() const;
    void                       setSubWindowType(const QString& subWindowType);

    bool                       isLastActive() const;
    void                       setIsLastActive(bool isLastActive);

private:

    QPointer<PLMBaseSubWindow>m_window;
    int m_parentId;
    int m_id;
    Qt::Orientation m_orientation;
    QList<QPointer<QBoxLayout> >m_layoutList;
    QList<QPointer<QSplitter> >m_splitterList;
    QString m_subWindowType;
    bool m_isLastActive;
};
Q_DECLARE_METATYPE(WindowContainer)

class PLMBaseSubWindowManager : public QObject {
    Q_OBJECT

public:

    explicit PLMBaseSubWindowManager(QWidget       *parent,
                                     const QString& objectName);
    ~PLMBaseSubWindowManager();

    bool              haveOneWindow(const QString& subWindowType);
    PLMBaseSubWindow* getWindowByType(const QString& subWindowType);

signals:

public slots:

    PLMBaseSubWindow* addSubWindow(const QString & subWindowType,
                                   Qt::Orientation orientation,
                                   int             parentZone,
                                   int             forcedId = -2);

    void applySettings();
    void writeSettings();

protected:

    virtual PLMBaseSubWindow* subWindowByName(const QString& subWindowType,
                                              int            id) = 0;
    virtual QString           defaultSubWindowType() const = 0;

private slots:

    void closeSubWindow(int id);
    void setLastActiveWindowByType(int id);

private:

    QString subWindowTypeById(int id) const;

    QList<WindowContainer>m_windowContainerList;
    int m_lastNumber;
};

#endif // PLMSUBWINDOWMANAGER_H

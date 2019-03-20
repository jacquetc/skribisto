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
#include <QMetaEnum>
#include <QDateTime>

class Widget {
    Q_GADGET

public:

    enum SubWindowType { Splitter, SubWindow };
    Q_ENUM(SubWindowType)

    Widget();
    Widget(const QString       & tableName,
           const QString       & documentTableName,
           int                   id,
           Widget::SubWindowType type);
    Widget(const Widget& otherWidget);
    ~Widget();
    bool                      operator!() const;
    Widget                  & operator=(const Widget& otherWidget);
    bool                      operator==(const Widget& otherWidget) const;

    int                       parentId() const;
    void                      setParentId(int value);

    int                       id() const;
    void                      setId(int value);

    QString                   name() const;
    bool                      setName(const QString& name);


    void                      addChildWidget(int id);
    void                      removeChildWidget(int id);
    int                       getChildWidgetId(int index) const;
    QList<int>                getChildWidgetList() const;
    int                       getChildCount() const;
    void                      replaceChild(int oldId,
                                           int newId);
    void                      balanceChildrenSizes() const;

    void                      setSplitterSizes(const QList<int>sizes) const;
    QList<int>                splitterSizes() const;

    QString                   tableName() const;

    void                      setSubWindowType(const SubWindowType& type);
    SubWindowType             subWindowType() const;

    QPointer<PLMBaseSubWindow>getWindow() const;
    void                      setWindow(const QPointer<PLMBaseSubWindow>& window);

    QPointer<QSplitter>       getSplitter() const;
    void                      setSplitter(const QPointer<QSplitter>& splitter);

    void                      loadValues();

    void                      setOrientation(const Qt::Orientation& orientation);

    Qt::Orientation           getOrientation() const;

    QString                   documentTableName() const;


    QDateTime                 getLastFocused() const;
    void                      setLastFocused(const QDateTime& lastFocused);

private:

    int m_parentId;
    int m_id;
    Qt::Orientation m_orientation;
    QPointer<PLMBaseSubWindow>m_window;
    QPointer<QSplitter>m_splitter;
    SubWindowType m_subWindowType;
    QList<int>m_childWidgetList;
    bool m_isLastActive;
    QString m_tableName;
    QString m_documentTableName;
    QList<int>m_splitterSizes;
    QDateTime m_lastFocused;
};
Q_DECLARE_METATYPE(Widget)

class PLMBaseSubWindowManager : public QObject {
    Q_OBJECT

public:

    explicit PLMBaseSubWindowManager(QBoxLayout    *parentLayout,
                                     const QString& objectName);
    ~PLMBaseSubWindowManager();

    PLMBaseSubWindow* getWindowByType(const QString& subWindowType);
    PLMBaseSubWindow* getLastFocusedWindow();

    virtual QString   tableName() const         = 0;
    virtual QString   documentTableName() const = 0;

signals:

public slots:

    int               addSubWindow(Qt::Orientation orientation,
                                   int             callerSubWindowId);

    void              applySettings();
    void              writeSettings();

    void              applyUserSettings();
    void              clear();

    void              writeUserSettingsOnOtherProjects();
    PLMBaseSubWindow* getSubWindowById(int id);

protected:

private slots:

    void closeSubWindow(int id);

private:

    virtual void             afterApplyUserSetting(int projectId)     = 0;
    virtual PLMBaseDocument* getDocument(const QString& documentType) = 0;

    bool                     haveOneSubWindow();
    PLMBaseSubWindow       * getFirstSubWindow();
    int                      addSplitter(int             parentId,
                                         Qt::Orientation orientation);
    int                      addSubWindow(int parentId);

    Widget::SubWindowType    getSubWindowTypeById(int id) const;
    Widget                   getWidgetById(int id);
    int                      getFreeNumber();
    void                     saveWidgetToList(Widget widget);

    QList<Widget>m_widgetList;
    QBoxLayout *m_parentLayout;
};

#endif // PLMSUBWINDOWMANAGER_H

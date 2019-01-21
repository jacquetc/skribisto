#ifndef PLMTESTWIDGET_H
#define PLMTESTWIDGET_H

#include <QLabel>
#include <QList>
#include <QObject>
#include <QString>

 #include "plmdockwidgetinterface.h"

// #include "plugininterface.h"

class PLMSheetTreeWidget : public QObject,
                           public PLMDockWidgetInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "com.PlumeSoft.Plume-Creator.DockWidgetInterface/1.0" FILE
        "plmsheettreewidget.json")
    Q_INTERFACES(PLMDockWidgetInterface)

public:

    PLMSheetTreeWidget(QObject *parent = nullptr);
    ~PLMSheetTreeWidget();

    // BaseInterface

    QString use() const;
    QString name() const
    {
        return "SheetTree";
    }

    QString displayedName() const
    {
        return tr("Sheets");
    }

    PLMBaseDockWidget* dockBodyWidget(QWidget *parent);

    QWidget          * dockHeaderWidget(QWidget *parent);

    Qt::Edge           getEdges() {
        return Qt::LeftEdge;
    }

    QString getParentWindowName() const {
        return "WriteWindow";
    }

    PLMDockWidgetInterface* clone() const;

private slots:

private:

    void instanciate(QWidget *parent);

    QString m_name;
    QLabel *m_label;
};

#endif // TESTWIDGET_H

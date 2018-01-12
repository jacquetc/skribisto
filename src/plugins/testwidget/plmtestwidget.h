#ifndef PLMTESTWIDGET_H
#define PLMTESTWIDGET_H

#include <QLabel>
#include <QList>
#include <QObject>

#include "plmcoreinterface.h"
#include "plugininterface.h"
#include "plmbasewidget.h"

class PLMTestWidget : public QObject,
                      public PLMBaseInterface,
                      public PLMWriteLeftDockInterface
                       {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "com.PlumeSoft.Plume-Creator.WidgetInterface/1.0" FILE
        "plmtestwidget.json")
    Q_INTERFACES(PLMBaseInterface PLMWriteLeftDockInterface)

public :

        PLMTestWidget(QObject *parent = 0);
    ~PLMTestWidget();

        // BaseInterface

    QString         use() const;
    QString         name() const
    {
        return "TestWidget";
    }

    PLMBaseWidget *dockBodyWidget(QWidget *parent);

private slots:

    void setText();
private:

    QString m_name;
    QLabel *m_label;
};

#endif // TESTWIDGET_H

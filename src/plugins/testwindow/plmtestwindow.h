#ifndef PLMTESTWINDOW_H
#define PLMTESTWINDOW_H

#include <QLabel>
#include <QList>
#include <QObject>

#include "plmguiinterface.h"
#include "plmsidemainbar.h"
#include "plmbasewindow.h"

class PLMTestWindow : public QObject,
                      public PLMWindowInterface,
                      public PLMSideMainBarIconInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "com.PlumeSoft.Plume-Creator.WindowInterface/1.0" FILE
        "plmtestwindow.json")
    Q_INTERFACES(PLMWindowInterface PLMSideMainBarIconInterface)

public :

        PLMTestWindow(QObject *parent = 0);
    ~PLMTestWindow();

    QString         use() const;

    // PanelInterface
    PLMBaseWindow* window();
    QString         name() const
    {
        return "TestWindow";
    }

    // SideMainBarIconInterface
    QList<PLMSideBarAction>sideMainBarActions(QObject *parent);

private slots:

private:

    QString m_name;
};

#endif // TESTWINDOW_H

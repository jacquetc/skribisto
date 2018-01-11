#ifndef PLMWRITEWINDOW_H
#define PLMWRITEWINDOW_H

#include <QLabel>
#include <QList>
#include <QObject>

#include "plmcoreinterface.h"
#include "plmguiinterface.h"
#include "plmsidemainbar.h"
#include "plmbasewindow.h"

class PLMWriteWindow : public QObject,
                      public PLMBaseInterface,
                      public PLMWindowInterface,
                      public PLMSideMainBarIconInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "com.PlumeSoft.Plume-Creator.WindowInterface/1.0" FILE
        "plmwritewindow.json")
    Q_INTERFACES(PLMBaseInterface PLMWindowInterface PLMSideMainBarIconInterface)

public :

        PLMWriteWindow(QObject *parent = 0);
    ~PLMWriteWindow();

        //BaseInterface
    QString         use() const;
    QString         name() const
    {
        return "WriteWindow";
    }

    // WindowInterface
    PLMBaseWindow* window();

    // SideMainBarIconInterface
    QList<PLMSideBarAction>sideMainBarActions(QObject *parent);

private slots:

    void init();
private:

    QString m_name;
};

#endif // WRITEWINDOW_H

#ifndef PLMWELCOMEWINDOW_H
#define PLMWELCOMEWINDOW_H

#include <QLabel>
#include <QList>
#include <QObject>

#include "plmcoreinterface.h"
#include "plmguiinterface.h"
#include "plmsidemainbar.h"
#include "plmbasewindow.h"

class PLMWelcomeWindow : public QObject,
                      public PLMBaseInterface,
                      public PLMWindowInterface,
                      public PLMSideMainBarIconInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "com.PlumeSoft.Plume-Creator.WindowInterface/1.0" FILE
        "plmwelcomewindow.json")
    Q_INTERFACES(PLMBaseInterface PLMWindowInterface PLMSideMainBarIconInterface)

public :

        PLMWelcomeWindow(QObject *parent = 0);
        void init();
    ~PLMWelcomeWindow();

        //BaseInterface
    QString         use() const;
    QString         name() const
    {
        return "WelcomeWindow";
    }

    // WindowInterface
    PLMBaseWindow* window();

    // SideMainBarIconInterface
    QList<PLMSideBarAction>sideMainBarActions(QObject *parent);

private slots:

private:

    QString m_name;
};

#endif // WELCOMEWINDOW_H

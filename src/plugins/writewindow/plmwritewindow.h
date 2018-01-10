#ifndef PLMWRITEWINDOW_H
#define PLMWRITEWINDOW_H

#include <QLabel>
#include <QList>
#include <QObject>

#include "plmguiinterface.h"
#include "plmsidemainbar.h"
#include "plmbasewindow.h"

class PLMWriteWindow : public QObject,
                      public PLMWindowInterface,
                      public PLMSideMainBarIconInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "com.PlumeSoft.Plume-Creator.WindowInterface/1.0" FILE
        "plmwritewindow.json")
    Q_INTERFACES(PLMWindowInterface PLMSideMainBarIconInterface)

public :

        PLMWriteWindow(QObject *parent = 0);
    ~PLMWriteWindow();

    QString         use() const;

    // WindowInterface
    PLMBaseWindow* window();
    QString         name() const
    {
        return "WriteWindow";
    }

    // SideMainBarIconInterface
    QList<PLMSideBarAction>mainBarActions(QObject *parent);

private slots:

private:

    QString m_name;
};

#endif // WRITEWINDOW_H

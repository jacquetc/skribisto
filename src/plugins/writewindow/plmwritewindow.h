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

public:

    PLMWriteWindow(QObject *parent = nullptr);
    void init();
    ~PLMWriteWindow();

    // BaseInterface
    QString use() const;
    QString name() const
    {
        return "WriteWindow";
    }

    QString displayedName() const
    {
        return tr("Write Window");
    }

    // WindowInterface
    PLMBaseWindow        * window();

    // SideMainBarIconInterface
    QList<PLMSideBarAction>sideMainBarActions(QObject *parent);

private slots:

private:

    QString m_name;
};

#endif // WRITEWINDOW_H

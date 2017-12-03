#ifndef PLMTESTWINDOW_H
#define PLMTESTWINDOW_H

#include <QLabel>
#include <QList>
#include <QObject>

#include "plmguiinterface.h"
#include "plmpanelwindow.h"
#include "plmsidemainbar.h"

class PLMTestWindow : public QObject,
    public PLMPanelInterface,
    public PLMSideMainBarIconInterface
{
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "com.PlumeSoft.Plume-Creator.PanelInterface/1.0" FILE "plmtestwindow.json")
    Q_INTERFACES(PLMPanelInterface PLMSideMainBarIconInterface)

public :

    PLMTestWindow(QObject *parent = 0);
    ~PLMTestWindow();

    QString panelName() const;

    //PanelInterface
    PLMPanelWindow *panel();
    QString name() const
    {
        return "TestWindow";
    }

    //SideMainBarIconInterface
    QList<PLMSideBarAction> mainBarActions(QObject *parent);

private:
    QString m_name;
};

#endif // TESTWINDOW_H

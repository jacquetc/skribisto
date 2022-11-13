#ifndef DESKTOPAPPLICATION_H
#define DESKTOPAPPLICATION_H

#include <QApplication>
#include <QObject>
#include "skribisto_desktop_common_global.h"

class SKRDESKTOPCOMMONEXPORT DesktopApplication : public QApplication
{
    Q_OBJECT
public:
    DesktopApplication(int &argc, char **argv);

signals:
    void settingsChanged(QHash<QString, QVariant>);
};

#endif // DESKTOPAPPLICATION_H

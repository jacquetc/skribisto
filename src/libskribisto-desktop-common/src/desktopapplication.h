#pragma once


#include "skribisto_desktop_common_global.h"
#include <QApplication>
#include <QObject>

class SKRDESKTOPCOMMONEXPORT DesktopApplication : public QApplication
{
    Q_OBJECT
  public:
    DesktopApplication(int &argc, char **argv);

  signals:
    void settingsChanged(QHash<QString, QVariant>);
};



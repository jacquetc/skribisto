#ifndef SETTINGSSUBPANEL_H
#define SETTINGSSUBPANEL_H

#include "skribisto_desktop_common_global.h"
#include <QWidget>

namespace Ui {
class SettingsSubPanel;
}

class SKRDESKTOPCOMMONEXPORT SettingsSubPanel : public QWidget {
  Q_OBJECT

public:
  explicit SettingsSubPanel(QWidget *parent = nullptr);
  ~SettingsSubPanel();

public slots:
    virtual void accept() = 0;
    virtual void reset() = 0;

protected:
  void setCentralWidget(QWidget *widget);

private:
  Ui::SettingsSubPanel *ui;
};

#endif // SETTINGSSUBPANEL_H

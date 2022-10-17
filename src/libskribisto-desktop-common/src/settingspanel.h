#ifndef SETTINGSPANEL_H
#define SETTINGSPANEL_H

#include "skribisto_desktop_common_global.h"
#include <QWidget>

namespace Ui {
class SettingsPanel;
}

class SKRDESKTOPCOMMONEXPORT SettingsPanel : public QWidget {
  Q_OBJECT

public:
  explicit SettingsPanel(QWidget *parent = nullptr);
  ~SettingsPanel();

public slots:
   virtual void accept() = 0;

protected:
  void setCentralWidget(QWidget *widget);

private:
  Ui::SettingsPanel *ui;
};

#endif // SETTINGSPANEL_H

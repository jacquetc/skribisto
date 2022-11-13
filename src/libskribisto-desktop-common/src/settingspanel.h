#ifndef SETTINGSPAGE_H
#define SETTINGSPAGE_H

#include "skribisto_desktop_common_global.h"
#include <QListWidgetItem>
#include <QWidget>
#include "basicsettingspanel.h"

namespace Ui {
class SettingsPanel;
}

class SKRDESKTOPCOMMONEXPORT SettingsPanel : public BasicSettingsPanel {
  Q_OBJECT

public:
  explicit SettingsPanel(QWidget *parent = nullptr);
  ~SettingsPanel();

  const QString &settingsGroup() const;
  void setSettingsGroup(const QString &newSettingsGroup);

signals:
  void settingsGroupChanged();

private slots:
  void on_listWidget_itemActivated(QListWidgetItem *item);

private:
  Ui::SettingsPanel *ui;
  QString m_settingsGroup;
  Q_PROPERTY(QString settingsGroup READ settingsGroup WRITE setSettingsGroup
                 NOTIFY settingsGroupChanged)
};

#endif // SETTINGSPAGE_H

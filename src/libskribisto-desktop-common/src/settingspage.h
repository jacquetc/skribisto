#ifndef SETTINGSPAGE_H
#define SETTINGSPAGE_H

#include "skribisto_desktop_common_global.h"
#include <QListWidgetItem>
#include <QWidget>

namespace Ui {
class SettingsPage;
}

class SKRDESKTOPCOMMONEXPORT SettingsPage : public QWidget {
  Q_OBJECT

public:
  explicit SettingsPage(QWidget *parent = nullptr);
  ~SettingsPage();

  const QString &settingsGroup() const;
  void setSettingsGroup(const QString &newSettingsGroup);
  void accept();

signals:
  void settingsGroupChanged();
  void accepted();

private slots:
  void on_listWidget_itemActivated(QListWidgetItem *item);

private:
  Ui::SettingsPage *ui;
  QString m_settingsGroup;
  Q_PROPERTY(QString settingsGroup READ settingsGroup WRITE setSettingsGroup
                 NOTIFY settingsGroupChanged)
};

#endif // SETTINGSPAGE_H

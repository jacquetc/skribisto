#include "settingspanel.h"
#include "skrdata.h"
#include "ui_settingspanel.h"

#include <interfaces/settingspanelinterface.h>

SettingsPanel::SettingsPanel(QWidget *parent)
    : QWidget(parent), ui(new Ui::SettingsPanel) {
  ui->setupUi(this);
}

SettingsPanel::~SettingsPanel() { delete ui; }

const QString &SettingsPanel::settingsGroup() const { return m_settingsGroup; }

void SettingsPanel::setSettingsGroup(const QString &newSettingsGroup) {
  if (m_settingsGroup == newSettingsGroup)
    return;
  m_settingsGroup = newSettingsGroup;

  ui->listWidget->clear();

  QList<SettingsPanelInterface *> pluginList =
      skrpluginhub->pluginsByType<SettingsPanelInterface>();

  for (auto *plugin : pluginList) {
    if (plugin->settingsGroup() == m_settingsGroup) {
      auto *panel = plugin->settingsPanel();
      connect(this, &SettingsPanel::accepted, panel, &SettingsSubPanel::accept);
      ui->stackedWidget->addWidget(panel);
      new QListWidgetItem(QIcon(plugin->settingsPanelIconSource()),
                          plugin->settingsPanelButtonText(), ui->listWidget);
    }
  }
  emit settingsGroupChanged();
}

void SettingsPanel::accept() { emit this->accepted(); }

void SettingsPanel::on_listWidget_itemActivated(QListWidgetItem *item) {
  ui->stackedWidget->setCurrentIndex(ui->listWidget->row(item));
}

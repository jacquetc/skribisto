#include "settingspage.h"
#include "skrdata.h"
#include "ui_settingspage.h"

#include <interfaces/settingspanelinterface.h>

SettingsPage::SettingsPage(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::SettingsPage)
{
    ui->setupUi(this);
}

SettingsPage::~SettingsPage()
{
    delete ui;
}

const QString &SettingsPage::settingsGroup() const
{
    return m_settingsGroup;
}

void SettingsPage::setSettingsGroup(const QString &newSettingsGroup)
{
    if (m_settingsGroup == newSettingsGroup)
        return;
    m_settingsGroup = newSettingsGroup;

    ui->listWidget->clear();

    QList<SettingsPanelInterface *> pluginList =
            skrpluginhub->pluginsByType<SettingsPanelInterface>();

    for(auto *plugin : pluginList){
        if(plugin->settingsGroup() == m_settingsGroup){
            auto *panel = plugin->settingsPanel();
            connect(this, &SettingsPage::accepted, panel, &SettingsPanel::accept);
            ui->stackedWidget->addWidget(panel);
            new QListWidgetItem(QIcon(plugin->settingsPanelIconSource()), plugin->settingsPanelButtonText(), ui->listWidget);
        }
    }
    emit settingsGroupChanged();
}

void SettingsPage::accept()
{
    emit this->accepted();
}

void SettingsPage::on_listWidget_itemActivated(QListWidgetItem *item)
{
    ui->stackedWidget->setCurrentIndex(ui->listWidget->row(item));
}


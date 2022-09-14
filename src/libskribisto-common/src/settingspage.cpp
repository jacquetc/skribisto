#include "settingspage.h"
#include "ui_settingspage.h"

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
    emit settingsGroupChanged();
}

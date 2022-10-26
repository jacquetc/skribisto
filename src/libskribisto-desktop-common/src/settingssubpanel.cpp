#include "settingssubpanel.h"
#include "ui_settingssubpanel.h"

SettingsSubPanel::SettingsSubPanel(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::SettingsSubPanel)
{
    ui->setupUi(this);
}

SettingsSubPanel::~SettingsSubPanel()
{
    delete ui;
}

void SettingsSubPanel::setCentralWidget(QWidget *widget)
{

        ui->centralLayout->addWidget(widget);

}

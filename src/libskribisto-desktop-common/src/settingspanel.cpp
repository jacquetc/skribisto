#include "settingspanel.h"
#include "ui_settingspanel.h"

SettingsPanel::SettingsPanel(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::SettingsPanel)
{
    ui->setupUi(this);
}

SettingsPanel::~SettingsPanel()
{
    delete ui;
}

void SettingsPanel::setCentralWidget(QWidget *widget)
{

        ui->centralLayout->addWidget(widget);

}

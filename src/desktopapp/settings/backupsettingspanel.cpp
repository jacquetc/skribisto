#include "backupsettingspanel.h"
#include "ui_backupsettingspanel.h"

BackupSettingsPanel::BackupSettingsPanel(QWidget *parent) :
    BasicSettingsPanel(parent),
    ui(new Ui::BackupSettingsPanel)
{
    ui->setupUi(this);
}

BackupSettingsPanel::~BackupSettingsPanel()
{
    delete ui;
}

void BackupSettingsPanel::accept()
{

}

void BackupSettingsPanel::reset()
{

}

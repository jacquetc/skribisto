#include "installnewdictwizard.h"
#include "ui_installnewdictwizard.h"

InstallNewDictWizard::InstallNewDictWizard(QWidget *parent) :
    QWizard(parent),
    ui(new Ui::InstallNewDictWizard)
{
    ui->setupUi(this);
}

InstallNewDictWizard::~InstallNewDictWizard()
{
    delete ui;
}

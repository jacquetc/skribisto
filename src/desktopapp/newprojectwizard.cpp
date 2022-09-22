#include "newprojectwizard.h"
#include "ui_newprojectwizard.h"

NewProjectWizard::NewProjectWizard(QWidget *parent) :
    QWizard(parent),
    ui(new Ui::NewProjectWizard)
{
    ui->setupUi(this);
}

NewProjectWizard::~NewProjectWizard()
{
    delete ui;
}

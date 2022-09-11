#include "themewizard.h"
#include "ui_themewizard.h"

ThemeWizard::ThemeWizard(QWidget *parent) :
    QWizard(parent),
    ui(new Ui::ThemeWizard)
{
    ui->setupUi(this);
}

ThemeWizard::~ThemeWizard()
{
    delete ui;
}

#include "settingsdialog.h"
#include "thememanager.h"
#include "ui_settingsdialog.h"

SettingsDialog::SettingsDialog(QWidget *parent) :
    QDialog(parent),
    ui(new Ui::SettingsDialog)
{
    //TODO: move in another classe Appareance

    ui->setupUi(this);
    on_backToolButton_clicked();

    themeManager->scanChildrenAndAddWidgetsHoldingIcons(this);
    ui->appearanceToolButton->setFocus();



    connect(ui->buttonBox, &QDialogButtonBox::clicked, this, [this](QAbstractButton *button){

        if(ui->buttonBox->buttonRole(button) == QDialogButtonBox::ResetRole){
            ui->appearancePanel->reset();
            ui->pagesPanel->reset();
        }
        if(ui->buttonBox->buttonRole(button) == QDialogButtonBox::AcceptRole){

            ui->appearancePanel->accept();
            ui->pagesPanel->accept();

            this->close();
        }
        if(ui->buttonBox->buttonRole(button) == QDialogButtonBox::ApplyRole){

            ui->appearancePanel->accept();
            ui->pagesPanel->accept();

        }
    });
}

SettingsDialog::~SettingsDialog()
{
    delete ui;
}


void SettingsDialog::on_backToolButton_clicked()
{
    ui->stackedWidget->setCurrentWidget(ui->mainPage);
    ui->backToolButton->hide();
    ui->pageTitle->hide();
}





void SettingsDialog::on_appearanceToolButton_clicked()
{
    ui->stackedWidget->setCurrentWidget(ui->appearancePanel);
    ui->pageTitle->setText("**" + ui->appearanceToolButton->text() + "**");
    ui->pageTitle->show();
    ui->backToolButton->show();
}


void SettingsDialog::on_pagesToolButton_clicked()
{
    ui->stackedWidget->setCurrentWidget(ui->pagesPanel);
    ui->pageTitle->setText("**" + ui->pagesToolButton->text() + "**");
    ui->pageTitle->show();
    ui->backToolButton->show();
}

void SettingsDialog::on_backupToolButton_clicked()
{
    ui->stackedWidget->setCurrentWidget(ui->backupPanel);
    ui->pageTitle->setText("**" + ui->backupToolButton->text() + "**");
    ui->pageTitle->show();
    ui->backToolButton->show();
}


#include "settingsdialog.h"
#include "thememanager.h"
#include "themewizard.h"
#include "ui_settingsdialog.h"

SettingsDialog::SettingsDialog(QWidget *parent) :
    QDialog(parent),
    ui(new Ui::SettingsDialog)
{
    ui->setupUi(this);
    on_backToolButton_clicked();

    connect(ui->createThemeButton, &QPushButton::clicked, this, [this](){
        ThemeWizard wizard(this);
        wizard.exec();
    });

    themeManager->scanChildrenAndAddWidgetsHoldingIcons(this);
    ui->appearancePage->setFocus();


    // themes
    themeManager->reloadThemes();

    ui->dayThemeComboBox->addItems(themeManager->lightThemeWithLocationMap().keys());
    ui->dayThemeComboBox->setCurrentText(themeManager->lightTheme());
    ui->nightThemeComboBox->addItems(themeManager->darkThemeWithLocationMap().keys());
    ui->nightThemeComboBox->setCurrentText(themeManager->darkTheme());

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
    ui->stackedWidget->setCurrentWidget(ui->appearancePage);
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


void SettingsDialog::on_buttonBox_accepted()
{
    themeManager->setLightTheme(ui->dayThemeComboBox->currentText());
    themeManager->setDarkTheme(ui->nightThemeComboBox->currentText());

    ui->pagesPanel->accept();
}


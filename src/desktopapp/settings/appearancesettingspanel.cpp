#include "appearancesettingspanel.h"
#include "thememanager.h"
#include "themewizard.h"
#include "ui_appearancesettingspanel.h"

AppearanceSettingsPanel::AppearanceSettingsPanel(QWidget *parent) :
    BasicSettingsPanel(parent),
    ui(new Ui::AppearanceSettingsPanel)
{
    ui->setupUi(this);


    connect(ui->createThemeButton, &QPushButton::clicked, this, [this](){
        ThemeWizard wizard(this);
        wizard.exec();
        AppearanceSettingsPanel::reset();
    });


    AppearanceSettingsPanel::reset();
}

AppearanceSettingsPanel::~AppearanceSettingsPanel()
{
    delete ui;
}

void AppearanceSettingsPanel::accept(){

    themeManager->setLightTheme(ui->dayThemeComboBox->currentText());
    themeManager->setDarkTheme(ui->nightThemeComboBox->currentText());
}

void AppearanceSettingsPanel::reset()
{

    // themes
    themeManager->reloadThemes();

    ui->dayThemeComboBox->clear();
    ui->nightThemeComboBox->clear();

    ui->dayThemeComboBox->addItems(themeManager->lightThemeWithLocationMap().keys());
    ui->dayThemeComboBox->setCurrentText(themeManager->lightTheme());
    ui->nightThemeComboBox->addItems(themeManager->darkThemeWithLocationMap().keys());
    ui->nightThemeComboBox->setCurrentText(themeManager->darkTheme());


}

#include "backupsettingspanel.h"
#include "backupmanager.h"
#include "ui_backupsettingspanel.h"

#include "desktopapplication.h"
#include <QFileDialog>
#include <QSettings>
#include <QStandardPaths>

BackupSettingsPanel::BackupSettingsPanel(QWidget *parent) :
    BasicSettingsPanel(parent),
    ui(new Ui::BackupSettingsPanel)
{
    ui->setupUi(this);

    BackupSettingsPanel::reset();

    connect(ui->removeBackupPathButton, &QAbstractButton::clicked, this, [this](){
        for(auto *item : ui->backupPathListWidget->selectedItems()){
            m_urlPathsToRemove << item->data(Qt::UserRole).toUrl();
            ui->backupPathListWidget->takeItem(ui->backupPathListWidget->row(item));
            delete item;
        }
        checkConfiguration();
    });

    connect(ui->addBackupPathButton, &QAbstractButton::clicked, this, [this](){

        QString startPath = QStandardPaths::writableLocation(QStandardPaths::DocumentsLocation);


        QUrl url = QFileDialog::getExistingDirectoryUrl(this, tr("Select a folder for backups"), QUrl::fromLocalFile(startPath),  QFileDialog::ShowDirsOnly
                                                        | QFileDialog::DontResolveSymlinks);

        if(!url.isEmpty()){
            m_urlPathsToAdd << url;

            auto item = new QListWidgetItem(url.toLocalFile(), ui->backupPathListWidget);
            item->setData(Qt::UserRole, url);
        }



    });

    ui->periodicalBackUpSpinBox->setSuffix(tr(" hour"));

    connect(ui->enablePeriodicalBackupCheckBox, &QCheckBox::stateChanged, ui->periodicalBackUpSpinBox, &QSpinBox::setEnabled);
    connect(ui->enablePeriodicalBackupCheckBox, &QCheckBox::stateChanged, ui->periodicalBackUpLabel, &QSpinBox::setEnabled);
    connect(ui->enablePeriodicalBackupCheckBox, &QCheckBox::stateChanged, this, &BackupSettingsPanel::checkConfiguration);
    connect(ui->backUpOnceADayCheckBox, &QCheckBox::stateChanged, this, &BackupSettingsPanel::checkConfiguration);
    connect(ui->periodicalBackUpSpinBox, &QSpinBox::valueChanged, this, &BackupSettingsPanel::checkConfiguration);
    connect(ui->periodicalBackUpSpinBox, &QSpinBox::valueChanged, this, [this](int value){
        ui->periodicalBackUpSpinBox->setSuffix(value == 1 ? tr(" hour") : tr(" hours"));
    });

    checkConfiguration();
}

BackupSettingsPanel::~BackupSettingsPanel()
{
    delete ui;
}

void BackupSettingsPanel::checkConfiguration()
{
    if(ui->backupPathListWidget->count() == 0) {
        ui->backupInfoLabel->setText(tr("Backups disabled: backup paths missing."));
    }
    else if(!ui->enablePeriodicalBackupCheckBox->isChecked() && ui->backUpOnceADayCheckBox->isChecked()) {
        ui->backupInfoLabel->setText(tr("Active project is backed up only once a day."));
    }
    else if(!ui->enablePeriodicalBackupCheckBox->isChecked() && !ui->backUpOnceADayCheckBox->isChecked()) {
        ui->backupInfoLabel->setText(tr("All periodic backups are disabled !"));
    }
    else {
        ui->backupInfoLabel->setText(tr("Next periodic backup at %1").arg(backupManager->timeOfNextBackup().toString("hh:mm")));
    }

}


void BackupSettingsPanel::accept()
{
    for(const QUrl &url : m_urlPathsToRemove){
        backupManager->removePath(url);
    }

    for(const QUrl &url : m_urlPathsToAdd){
        backupManager->addPath(url);
    }

    QHash<QString, QVariant> newValuesHash;

    QSettings settings;

    bool enablePeriodicalBackup =  ui->enablePeriodicalBackupCheckBox->isChecked();
    settings.setValue("backup/enablePeriodicalBackup", enablePeriodicalBackup);
    newValuesHash.insert("backup/enablePeriodicalBackup", enablePeriodicalBackup);

    int periodicalBackup =  ui->periodicalBackUpSpinBox->value();
    settings.setValue("backup/periodicalBackup", periodicalBackup);
    newValuesHash.insert("backup/periodicalBackup", periodicalBackup);

    bool backUpOnceADay =  ui->backUpOnceADayCheckBox->isChecked();
    settings.setValue("backup/backUpOnceADay", backUpOnceADay);
    newValuesHash.insert("backup/backUpOnceADay", backUpOnceADay);


    QHash<QString, QVariant> changedValuesHash;
    QHash<QString, QVariant>::const_iterator i = newValuesHash.constBegin();
    while (i != newValuesHash.constEnd()) {

        if(m_defaultValuesHash.value(i.key()) != i.value()){
            changedValuesHash.insert(i.key(), i.value());
        }
        ++i;
    }


    if(!changedValuesHash.isEmpty()){
        emit static_cast<DesktopApplication *>(qApp)->settingsChanged(changedValuesHash);
    }
}

void BackupSettingsPanel::reset()
{
    m_urlPathsToRemove.clear();
    m_urlPathsToAdd.clear();
    ui->backupPathListWidget->clear();
    m_defaultValuesHash.clear();

    QList<QUrl> urlList = backupManager->getBackupPathes();



    for(const QUrl &url : urlList){

        auto item = new QListWidgetItem(url.toLocalFile(), ui->backupPathListWidget);
        item->setData(Qt::UserRole, url);
    }



    QSettings settings;

    bool enablePeriodicalBackup = settings.value("backup/enablePeriodicalBackup", true).toBool();
    ui->enablePeriodicalBackupCheckBox->setChecked(enablePeriodicalBackup);
    m_defaultValuesHash.insert("backup/enablePeriodicalBackup", enablePeriodicalBackup);

    ui->periodicalBackUpSpinBox->setEnabled(enablePeriodicalBackup);
    ui->periodicalBackUpLabel->setEnabled(enablePeriodicalBackup);

    int periodicalBackup = settings.value("backup/periodicalBackup", 2).toInt();
    ui->periodicalBackUpSpinBox->setValue(periodicalBackup);
    m_defaultValuesHash.insert("backup/periodicalBackup", periodicalBackup);
    ui->periodicalBackUpSpinBox->setSuffix(periodicalBackup == 1 ? tr(" hour") : tr(" hours"));

    bool backUpOnceADay = settings.value("backup/backUpOnceADay", true).toBool();
    ui->backUpOnceADayCheckBox->setChecked(backUpOnceADay);
    m_defaultValuesHash.insert("backup/backUpOnceADay", backUpOnceADay);
}

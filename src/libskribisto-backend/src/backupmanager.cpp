#include "backupmanager.h"
#include "project/plmprojectmanager.h"
#include "projectcommands.h"
#include "skrdata.h"

#include <QFileInfo>

BackupManager::BackupManager(QObject *parent)
    : QObject{parent}
{
    m_instance = this;


    connect(skrdata->projectHub(), &PLMProjectHub::projectLoaded, this, [this](int projectId){


        QSettings settings;
        bool backUpOnceADay = settings.value("backup/backUpOnceADay", true).toBool();

        if(!backUpOnceADay){
            return;
        }

        QList<QUrl> urlList = this->getBackupPathes();
        for(const QUrl &url : urlList){
            if(!doesBackupOfTheDayExistAtPath(projectId, url)){
                this->backupAProject(projectId, "skrib", url);

            }
        }


    });

    m_backupTimer = new QTimer(this);
    m_backupTimer->setInterval(120000);
    m_backupTimer->setSingleShot(false);

    QSettings settings;
    int periodicalBackup = settings.value("backup/periodicalBackup").toInt();
    m_nextPeriodicalBackup = QDateTime::currentDateTime().addSecs(3600 * periodicalBackup);

    bool enablePeriodicalBackup = settings.value("backup/enablePeriodicalBackup").toBool();
    enablePeriodicalBackup ? m_backupTimer->start() : m_backupTimer->stop();


    connect(m_backupTimer, &QTimer::timeout, this, [this](){

        if(m_nextPeriodicalBackup > QDateTime::currentDateTime()){
            return;
        }

        if(skrdata->projectHub()->getActiveProject() == -2){
            return;
        }

        QList<QUrl> urlList = this->getBackupPathes();
        for(const QUrl &url : urlList){
            this->backupAProject(skrdata->projectHub()->getActiveProject(), "skrib", url);
        }
    });

}
BackupManager *BackupManager::m_instance = nullptr;

//----------------------------------------------------------

SKRResult BackupManager::backupAProject(int projectId, const QString &type, const QUrl &folderPath)
{
    SKRResult result(this);

    PLMProject * project = plmProjectManager->project(projectId);
    QUrl projectPath = project->getFileName();

    if (projectPath.isEmpty()) {
        result = SKRResult(SKRResult::Warning, this, "no_path");
        result.addData("projectId", projectId);
    }

//    if (projectPath.scheme() == "qrc") {
//        result = SKRResult(SKRResult::Warning, this, "qrc_projects_cant_back_up");
//        result.addData("projectId", projectId);
//    }


    IFOK(result) {
        // verify backup path
        QFileInfo folderInfo(folderPath.toLocalFile());

        if (!folderInfo.exists()) {
            result = SKRResult(SKRResult::Critical, this, "path_dont_exist");
            result.addData("projectId", projectId);
        }

        if (!folderInfo.isDir()) {
            result = SKRResult(SKRResult::Critical, this, "path_not_a_directory");
            result.addData("projectId", projectId);
        }

        if (!folderInfo.isWritable()) {
            result = SKRResult(SKRResult::Critical, this, "path_not_writable");
            result.addData("projectId", projectId);
        }
    }


    // determine file base
    QFileInfo info(projectPath.toLocalFile());
    QFileInfo backupFolderInfo(folderPath.toLocalFile());

    QString   backupFile;
    if(projectPath.scheme() == "qrc"){
        backupFile = backupFolderInfo.filePath() + "/" + projectPath.path().split("/").last();
        backupFile.remove(".skrib");
    }
    else{
        backupFile = backupFolderInfo.filePath() + "/" + info.completeBaseName();
    }

    // add date and time :
    QDateTime now     = QDateTime::currentDateTime();
    QString   nowText = now.toString("_yyyy-MM-dd-HHmmss");

    backupFile = backupFile + nowText;

    // add suffix :
    backupFile = backupFile + "." + type;

    // firstly, save the project
    if(projectPath.scheme() != "qrc"){
        IFOKDO(result, projectCommands->save(projectId));
    }

    // then create a copy
    IFOK(result) {
        result = projectCommands->saveAProjectCopy(projectId,
                                             type,
                                             QUrl::fromLocalFile(backupFile));
    }

//    IFKO(result) {
//        emit errorSent(result);
//    }

    return result;
}

// ----------------------------------------------------------------------------

QList<QUrl> BackupManager::getBackupPathes() const
{
    QSettings settings;
    QString pathsString = settings.value("backup/paths", "").toString();

    QStringList pathsStringList = pathsString.split(";", Qt::SkipEmptyParts);

    QList<QUrl> urlList;

    for(const QString &path : pathsStringList){
        urlList << QUrl::fromLocalFile(path);
    }

    return urlList;
}

SKRResult BackupManager::addPath(const QUrl &folderPath)
{
    SKRResult result(this);

    //TODO: add file checks

    // read
    QSettings settings;
    QString pathsString = settings.value("backup/paths", "").toString();

    QStringList pathsStringList = pathsString.split(";", Qt::SkipEmptyParts);

    QList<QUrl> urlList;

    for(const QString &path : pathsStringList){
        urlList << QUrl::fromLocalFile(path);
    }

    // modify
    urlList.append(folderPath);

    // write back
    pathsStringList.clear();
     for(const QUrl &pathUrl : urlList){
        pathsStringList << pathUrl.toLocalFile();
    }

    settings.setValue("backup/paths", pathsStringList.join(";"));


    return result;
}

// ----------------------------------------------------------------------------

void BackupManager::removePath(const QUrl &folderPath)
{
    // read
    QSettings settings;
    QString pathsString = settings.value("backup/paths", "").toString();

    QStringList pathsStringList = pathsString.split(";", Qt::SkipEmptyParts);

    QList<QUrl> urlList;

    for(const QString &path : pathsStringList){
        urlList << QUrl::fromLocalFile(path);
    }

    // modify
    urlList.removeAll(folderPath);

    // write back
    pathsStringList.clear();
     for(const QUrl &pathUrl : urlList){
        pathsStringList << pathUrl.toLocalFile();
    }

    settings.setValue("backup/paths", pathsStringList.join(";"));

}

// ----------------------------------------------------------------------------

bool BackupManager::doesBackupOfTheDayExistAtPath(int projectId, const QUrl& folderPath)
{
    SKRResult result(this);

    QUrl projectPath = skrdata->projectHub()->getPath(projectId);

    if (projectPath.isEmpty()) {
        result = SKRResult(SKRResult::Warning, this, "no_path");
        result.addData("projectId", projectId);
    }
    IFOK(result) {
        // verify backup path
        QFileInfo folderInfo(folderPath.toLocalFile());

        if (!folderInfo.exists()) {
            result = SKRResult(SKRResult::Critical, this, "path_dont_exist");
            result.addData("projectId", projectId);
        }

        if (!folderInfo.isDir()) {
            result = SKRResult(SKRResult::Critical, this, "path_not_a_directory");
            result.addData("projectId", projectId);
        }

        if (!folderInfo.isWritable()) {
            result = SKRResult(SKRResult::Critical, this, "path_not_writable");
            result.addData("projectId", projectId);
        }
    }
    IFKO(result) {
       // emit errorSent(result);

        return true;
    }


    // determine file base
    QFileInfo info(projectPath.toLocalFile());
    QString   backupFileOfTheDay = info.completeBaseName();

    // add date and time :
    QDateTime now     = QDateTime::currentDateTime();
    QString   nowText = now.toString("_yyyy-MM-dd");

    backupFileOfTheDay = backupFileOfTheDay + nowText + "*";

    // find file begining with backupFileOfTheDay

    QDir dir(folderPath.toLocalFile());
    QStringList nameFilters;

    nameFilters << backupFileOfTheDay;
    QStringList entries = dir.entryList(nameFilters);

    if (entries.isEmpty()) {
        return false;
    }
    return true;
}

SKRResult BackupManager::createBackups(int projectId)
{

    SKRResult result(this);

    QList<QUrl> urlList = this->getBackupPathes();

    for(const QUrl &url : urlList){
            result = this->backupAProject(projectId, "skrib", url);

            IFKO(result){
                break;
            }


    }

    return result;
}

QDateTime BackupManager::timeOfNextBackup() const
{
    return m_nextPeriodicalBackup;
}

//---------------------------------------------------

void BackupManager::applySettingsChanges(const QHash<QString, QVariant> &newSettings)
{
    if(newSettings.contains("backup/enablePeriodicalBackup")){
        bool enablePeriodicalBackup = newSettings.value("backup/enablePeriodicalBackup").toBool();
        enablePeriodicalBackup ? m_backupTimer->start() : m_backupTimer->stop();

    }
    if(newSettings.contains("backup/periodicalBackup")){
        int periodicalBackup = newSettings.value("backup/periodicalBackup").toInt();
        m_nextPeriodicalBackup = QDateTime::currentDateTime().addSecs(3600 * periodicalBackup);

    }
    if(newSettings.contains("backup/backUpOnceADay")){
        bool backUpOnceADay = newSettings.value("backup/backUpOnceADay").toBool();


        if(!backUpOnceADay || skrdata->projectHub()->getActiveProject() == -2){
            return;
        }

        QList<QUrl> urlList = this->getBackupPathes();
        for(const QUrl &url : urlList){
            if(!doesBackupOfTheDayExistAtPath(skrdata->projectHub()->getActiveProject(), url)){
                this->backupAProject(skrdata->projectHub()->getActiveProject(), "skrib", url);

            }
        }

    }
}

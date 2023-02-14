#ifndef BACKUPMANAGER_H
#define BACKUPMANAGER_H

#include <QObject>
#include <QTimer>
#include <QDateTime>
#include "skribisto_backend_global.h"
#include "skrresult.h"

#define backupManager BackupManager::instance()

class SKRBACKENDEXPORT BackupManager : public QObject
{
    Q_OBJECT
public:
    explicit BackupManager(QObject *parent = nullptr);

    static BackupManager* instance()
    {
        return m_instance;
    }
    SKRResult backupAProject(int projectId, const QString &type, const QUrl &folderPath);

    QList<QUrl> getBackupPathes() const;
    SKRResult addPath(const QUrl &folderPath);
    void removePath(const QUrl &folderPath);

    bool doesBackupOfTheDayExistAtPath(int projectId, const QUrl &folderPath);

    SKRResult createBackups(int projectId);

    QDateTime timeOfNextBackup() const;

public slots:
    void applySettingsChanges(const QHash<QString, QVariant> &newSettings);

signals:

private:
    static BackupManager *m_instance;
    QTimer *m_backupTimer;
    QDateTime m_nextPeriodicalBackup;
};

#endif // BACKUPMANAGER_H

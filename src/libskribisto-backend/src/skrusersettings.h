#ifndef SKRUSERSETTINGS_H
#define SKRUSERSETTINGS_H

#include <QObject>
#include <QSettings>
#include <QString>
#include <QVariant>
#include <QHash>
#include "skribisto_backend_global.h"

class SKRBACKENDEXPORT SKRUserSettings : public QObject {
    Q_OBJECT

public:

    explicit SKRUserSettings(QObject *parent = nullptr);
    Q_INVOKABLE static void setProjectSetting(int            projectId,
                                       const QString& key,
                                       QVariant            value);
    Q_INVOKABLE static QVariant getProjectSetting(int            projectId,
                                       const QString& key,
                                       QVariant defaultValue);
    Q_INVOKABLE static void removeProjectSetting(int            projectId,
                                          const QString& key);

    Q_INVOKABLE static void insertInProjectSettingHash(int             projectId,
                                                const QString & key,
                                                const QString & hashKey,
                                                const QVariant& value);
    Q_INVOKABLE static QVariant getFromProjectSettingHash(int             projectId,
                                                   const QString & key,
                                                   const QString & hashKey,
                                                   const QVariant& defaultValue);
    Q_INVOKABLE static void removeFromProjectSettingHash(int            projectId,
                                                  const QString& key,
                                                  const QString& hashKey);

    Q_INVOKABLE static void insertInSomeGroupSettingHash(const QString & settingGroup,
                                                const QString & key,
                                                const QString & hashKey,
                                                const QVariant& value);
    Q_INVOKABLE static QVariant getFromSomeGroupSettingHash(const QString & settingGroup,
                                                   const QString & key,
                                                   const QString & hashKey,
                                                   const QVariant& defaultValue);
    Q_INVOKABLE static void removeFromSomeGroupSettingHash(const QString & settingGroup,
                                                  const QString& key,
                                                  const QString& hashKey);

    Q_INVOKABLE static void setSetting(const QString &group, const QString &key, QVariant value);
    Q_INVOKABLE static QVariant getSetting(const QString &group, const QString &key, QVariant defaultValue);
    Q_INVOKABLE static void removeSetting(const QString &group, const QString &key);
signals:

private:

    static QByteArray              serializingHash(const QHash<QString, QVariant>& hash);
    static QHash<QString, QVariant>deserializingHash(QByteArray hashArray);
};

#endif // SKRUSERSETTINGS_H

#ifndef SKRUSERSETTINGS_H
#define SKRUSERSETTINGS_H

#include <QObject>
#include <QSettings>
#include <QString>
#include <QVariant>
#include <QHash>
#include "skrdata.h"

class SKRUserSettings : public QObject {
    Q_OBJECT

public:

    explicit SKRUserSettings(QObject *parent = nullptr);
    Q_INVOKABLE void setProjectSetting(int            projectId,
                                       const QString& key,
                                       QVariant            value);
    Q_INVOKABLE QVariant getProjectSetting(int            projectId,
                                       const QString& key,
                                       QVariant defaultValue);
    Q_INVOKABLE void removeProjectSetting(int            projectId,
                                          const QString& key);

    Q_INVOKABLE void insertInProjectSettingHash(int             projectId,
                                                const QString & key,
                                                const QString & hashKey,
                                                const QVariant& value);
    Q_INVOKABLE QVariant getFromProjectSettingHash(int             projectId,
                                                   const QString & key,
                                                   const QString & hashKey,
                                                   const QVariant& defaultValue);
    Q_INVOKABLE void removeFromProjectSettingHash(int            projectId,
                                                  const QString& key,
                                                  const QString& hashKey);

    Q_INVOKABLE void insertInSomeGroupSettingHash(const QString & settingGroup,
                                                const QString & key,
                                                const QString & hashKey,
                                                const QVariant& value);
    Q_INVOKABLE QVariant getFromSomeGroupSettingHash(const QString & settingGroup,
                                                   const QString & key,
                                                   const QString & hashKey,
                                                   const QVariant& defaultValue);
    Q_INVOKABLE void removeFromSomeGroupSettingHash(const QString & settingGroup,
                                                  const QString& key,
                                                  const QString& hashKey);

    Q_INVOKABLE void setSetting(const QString &group, const QString &key, QVariant value);
    Q_INVOKABLE QVariant getSetting(const QString &group, const QString &key, QVariant defaultValue);
    Q_INVOKABLE void removeSetting(const QString &group, const QString &key);
signals:

private:

    QByteArray              serializingHash(const QHash<QString, QVariant>& hash) const;
    QHash<QString, QVariant>deserializingHash(QByteArray hashArray);
};

#endif // SKRUSERSETTINGS_H

#ifndef SKRUSERSETTINGS_H
#define SKRUSERSETTINGS_H

#include <QObject>
#include <QSettings>
#include <QString>
#include <QVariant>
#include <QHash>
#include "plmdata.h"

class SkrUserSettings : public QObject
{
    Q_OBJECT
public:
    explicit SkrUserSettings(QObject *parent = nullptr);
    Q_INVOKABLE void setProjectSetting(int projectId, const QString &key, int value);
    Q_INVOKABLE int getProjectSetting(int projectId, const QString &key, int defaultValue);
    Q_INVOKABLE void removeProjectSetting(int projectId, const QString &key);

    Q_INVOKABLE void insertInProjectSettingHash(int projectId, const QString &key, const QString &hashKey, const QVariant &value);
    Q_INVOKABLE QVariant getFromProjectSettingHash(int projectId, const QString &key, const QString &hashKey, const QVariant &defaultValue);
    Q_INVOKABLE void removeFromProjectSettingHash(int projectId, const QString &key, const QString &hashKey);



signals:
private:
    QByteArray serializingHash(const QHash<QString, QVariant> &hash) const;
    QHash<QString, QVariant> deserializingHash(QByteArray hashArray);
};

#endif // SKRUSERSETTINGS_H

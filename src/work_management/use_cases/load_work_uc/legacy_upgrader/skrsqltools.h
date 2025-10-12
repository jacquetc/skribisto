#pragma once

#include "skrresult.h"
#include <QFile>
#include <QObject>
#include <QtSql/QSqlDatabase>

namespace Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule
{
using namespace Qt::StringLiterals;

class SKRSqlTools : public QObject
{
    Q_OBJECT

  public:
    explicit SKRSqlTools(QObject *parent = nullptr);
    static SKRResult executeSQLFile(const QString &fileName, QSqlDatabase &sqlDB);
    static SKRResult executeSQLString(const QString &sqlString, QSqlDatabase &sqlDB);

    static QString getProjectTemplateDBVersion(SKRResult *result);
    static double getProjectDBVersion(SKRResult *result, const QString &sqlDbConnectionName);
    static SKRResult renumberTreeSortOrder(QSqlDatabase &sqlDb);
    static SKRResult addStringTreeProperty(QSqlDatabase &sqlDb, int tree_id, const QString &name, const QString &value);
    static SKRResult trimTreePropertyTable(QSqlDatabase &sqlDb);
    static SKRResult trimTagRelationshipTable(QSqlDatabase &sqlDb);
    static SKRResult trimTreeRelationshipTable(QSqlDatabase &sqlDb);
};
} // namespace Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule
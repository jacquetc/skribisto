#ifndef SKRSQLTOOLS_H
#define SKRSQLTOOLS_H


#include <QFile>
#include <QObject>
#include <QtSql/QSqlDatabase>
#include "skrresult.h"

class SKRSqlTools : public QObject {
    Q_OBJECT

public:

    explicit SKRSqlTools(QObject *parent = nullptr);
    static SKRResult executeSQLFile(const QString& fileName,
                                   QSqlDatabase & sqlDB);
    static SKRResult executeSQLString(const QString& sqlString,
                                     QSqlDatabase & sqlDB);

    static QString getProjectTemplateDBVersion(SKRResult *result);
    static double getProjectDBVersion(SKRResult *result, QSqlDatabase &sqlDb);
signals:
};

#endif // SKRSQLTOOLS_H

#include "skrsqltools.h"

#include <QtSql/QSqlQuery>
#include <QSqlDriver>
#include <QSqlError>
#include <QRegularExpression>
#include <QDebug>

SKRSqlTools::SKRSqlTools(QObject *parent) : QObject(parent)
{}

// -----------------------------------------------------------------------------------------------

SKRResult SKRSqlTools::executeSQLFile(const QString& fileName, QSqlDatabase& sqlDB) {
    SKRResult result("SKRSqlTools::executeSQLFile");
    QFile    file(fileName);

    // Read query file content
    file.open(QIODevice::ReadOnly);
    result = SKRSqlTools::executeSQLString(file.readAll(), sqlDB);
    file.close();

    return result;
}

// -----------------------------------------------------------------------------------------------

SKRResult SKRSqlTools::executeSQLString(const QString& sqlString, QSqlDatabase& sqlDB)
{
    SKRResult result("SKRSqlTools::executeSQLString");

    QSqlQuery query(sqlDB);


    QString queryStr = sqlString + "\n";

    // Check if SQL Driver supports Transactions
    if (sqlDB.driver()->hasFeature(QSqlDriver::Transactions)) {
        // protect TRIGGER's END
        queryStr =
            queryStr.replace(QRegularExpression("(;.*END)",
                                                QRegularExpression::CaseInsensitiveOption
                                                | QRegularExpression::MultilineOption),
                             "$END");
        // Replace comments and tabs and new lines with space
        queryStr =
            queryStr.replace(QRegularExpression("(\\/\\*(.)*?\\*\\/|^--.*\\n|\\t|\\n)",
                                                QRegularExpression::CaseInsensitiveOption
                                                | QRegularExpression::MultilineOption),
                             " ");
        queryStr = queryStr.replace(";", ";\n");
        queryStr = queryStr.replace("$END", ";END");

        queryStr = queryStr.trimmed();
        qDebug() << queryStr;

        // Extracting queries
        QStringList qList = queryStr.split('\n', Qt::SkipEmptyParts);


        QRegularExpression re_transaction("\\bbegin.transaction",
                                          QRegularExpression::CaseInsensitiveOption);
        QRegularExpression re_commit("\\bcommit",
                                     QRegularExpression::CaseInsensitiveOption);

        // Check if query file is already wrapped with a transaction
        bool isStartedWithTransaction = false;

        if (qList.size() > 1) {
            isStartedWithTransaction = qMax(re_transaction.match(qList.at(0)).hasMatch(),
                                            re_transaction.match(qList.at(1)).hasMatch());
        }

        if (!isStartedWithTransaction) sqlDB.transaction();

        // Execute each individual queries


        for (const QString& s : qList) {
            if (re_transaction.match(s).hasMatch()){
                sqlDB.transaction();
            }
            else if (re_commit.match(s).hasMatch()) {
                sqlDB.commit();
            }
            else {
                query.exec(s);

                if (query.lastError().type() != QSqlError::NoError) {
                    result = SKRResult(SKRResult::Critical, "SKRSqlTools::executeSQLString", "sql_error");
                    result.addData("SQLError", query.lastError().text());
                    result.addData("SQL string", s);
                    sqlDB.rollback();

                    return result;

                    //
                }
            }
        }

        if (!isStartedWithTransaction) sqlDB.commit();


        // Sql Driver doesn't supports transaction
    } else {
        // ...so we need to remove special queries (`begin transaction` and
        // `commit`)
        queryStr =
            queryStr.replace(QRegularExpression(
                                 "(\\bbegin.transaction.*;|\\bcommit.*;|\\/\\*(.|\\n)*?\\*\\/|^--.*\\n|\\t|\\n)",
                                 QRegularExpression::CaseInsensitiveOption
                                 | QRegularExpression::MultilineOption),
                             " ");
        queryStr = queryStr.trimmed();

        // Execute each individual queries
        QStringList qList = queryStr.split(';', Qt::SkipEmptyParts);

        for (const QString& s : qList) {
            query.exec(s);

            if (query.lastError().type() != QSqlError::NoError) {
                result = SKRResult(SKRResult::Critical, "SKRSqlTools::executeSQLString", "sql_error");
                result.addData("SQLError", query.lastError().text());
                result.addData("SQL string", s);
                sqlDB.rollback();
                return result;
            }
        }
        sqlDB.commit();
    }
    return result;
}

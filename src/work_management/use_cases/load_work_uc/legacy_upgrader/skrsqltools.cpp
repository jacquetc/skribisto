#include "skrsqltools.h"

#include <QDebug>
#include <QRegularExpression>
#include <QSqlDriver>
#include <QSqlError>
#include <QtSql/QSqlQuery>

namespace Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule
{
SKRSqlTools::SKRSqlTools(QObject *parent) : QObject(parent)
{
}

// -----------------------------------------------------------------------------------------------

SKRResult SKRSqlTools::executeSQLFile(const QString &fileName, QSqlDatabase &sqlDB)
{
    SKRResult result("SKRSqlTools::executeSQLFile"_L1);
    QFile file(fileName);

    // Read query file content
    file.open(QIODevice::ReadOnly);
    result = SKRSqlTools::executeSQLString(QString::fromLatin1(file.readAll()), sqlDB);
    file.close();

    return result;
}

// -----------------------------------------------------------------------------------------------

SKRResult SKRSqlTools::executeSQLString(const QString &sqlString, QSqlDatabase &sqlDB)
{
    SKRResult result("SKRSqlTools::executeSQLString"_L1);

    QSqlQuery query(sqlDB);

    QString queryStr = sqlString + "\n"_L1;

    // Check if SQL Driver supports Transactions
    if (sqlDB.driver()->hasFeature(QSqlDriver::Transactions))
    {
        // protect TRIGGER's END
        queryStr = queryStr.replace(QRegularExpression("(;.*END)"_L1, QRegularExpression::CaseInsensitiveOption |
                                                                          QRegularExpression::MultilineOption),
                                    "$END"_L1);

        // Replace comments and tabs and new lines with space
        queryStr = queryStr.replace(
            QRegularExpression("(\\/\\*(.)*?\\*\\/|^--.*\\n|\\t|\\n)"_L1,
                               QRegularExpression::CaseInsensitiveOption | QRegularExpression::MultilineOption),
            " "_L1);
        queryStr = queryStr.replace("_L1;"_L1, "_L1;\n"_L1);
        queryStr = queryStr.replace("$END"_L1, "_L1;END"_L1);

        queryStr = queryStr.trimmed();

        // qDebug() << queryStr;

        // Extracting queries
        QStringList qList = queryStr.split('\n'_L1, Qt::SkipEmptyParts);

        QRegularExpression re_transaction("\\bbegin.transaction"_L1, QRegularExpression::CaseInsensitiveOption);
        QRegularExpression re_commit("\\bcommit"_L1, QRegularExpression::CaseInsensitiveOption);

        // Check if query file is already wrapped with a transaction
        bool isStartedWithTransaction = false;

        if (qList.size() > 1)
        {
            isStartedWithTransaction =
                qMax(re_transaction.match(qList.at(0)).hasMatch(), re_transaction.match(qList.at(1)).hasMatch());
        }

        if (!isStartedWithTransaction)
            sqlDB.transaction();

        // Execute each individual queries

        for (const QString &s : qList)
        {
            if (re_transaction.match(s).hasMatch())
            {
                sqlDB.transaction();
            }
            else if (re_commit.match(s).hasMatch())
            {
                sqlDB.commit();
            }
            else
            {
                query.exec(s);

                if (query.lastError().type() != QSqlError::NoError)
                {
                    result = SKRResult(SKRResult::Critical, "SKRSqlTools::executeSQLString"_L1, "sql_error"_L1);
                    result.addData("SQLError"_L1, query.lastError().text());
                    result.addData("SQL string"_L1, s);
                    sqlDB.rollback();

                    return result;

                    //
                }
            }
        }

        if (!isStartedWithTransaction)
            sqlDB.commit();

        // Sql Driver doesn't supports transaction
    }
    else
    {
        // ...so we need to remove special queries (`begin transaction` and
        // `commit`)
        queryStr = queryStr.replace(
            QRegularExpression("(\\bbegin.transaction.*;|\\bcommit.*;|\\/\\*(.|\\n)*?\\*\\/|^--.*\\n|\\t|\\n)"_L1,
                               QRegularExpression::CaseInsensitiveOption | QRegularExpression::MultilineOption),
            " "_L1);
        queryStr = queryStr.trimmed();

        // Execute each individual queries
        QStringList qList = queryStr.split(';'_L1, Qt::SkipEmptyParts);

        for (const QString &s : qList)
        {
            query.exec(s);

            if (query.lastError().type() != QSqlError::NoError)
            {
                result = SKRResult(SKRResult::Critical, "SKRSqlTools::executeSQLString"_L1, "sql_error"_L1);
                result.addData("SQLError"_L1, query.lastError().text());
                result.addData("SQL string"_L1, s);
                sqlDB.rollback();
                return result;
            }
        }
        sqlDB.commit();
    }
    return result;
}

// ------------------------------------------------------------------

QString SKRSqlTools::getProjectTemplateDBVersion(SKRResult *result)
{
    QString dbVersion = "-2"_L1;
    QFile file(":/sql/sqlite_project.sql"_L1);

    file.open(QIODevice::ReadOnly);

    for (const QString &line : QString::fromLatin1(file.readAll()).split("\n"_L1))
    {
        if (line.contains("-- skribisto_db_version:"_L1))
        {
            QStringList splittedLine = line.split(":"_L1);

            if (splittedLine.count() == 2)
            {
                dbVersion = splittedLine.at(1);
            }

            break;
        }
    }
    file.close();

    if (dbVersion == "-2"_L1)
    {
        *result = SKRResult(SKRResult::Critical, "SKRSqlTools::getProjectTemplateVersion"_L1,
                            "no_db_version_found_in_sql_file"_L1);
    }

    return dbVersion;
}

// ------------------------------------------------------------------

double SKRSqlTools::getProjectDBVersion(SKRResult *result, const QString &sqlDbConnectionName)
{
    double dbVersion = -1;

    QSqlQuery query(QSqlDatabase::database(sqlDbConnectionName));
    QString queryStr = "SELECT dbl_database_version FROM tbl_project"_L1;

    query.prepare(queryStr);
    query.exec();

    while (query.next())
    {
        dbVersion = query.value(0).toDouble();
    }

    if (dbVersion == -1)
    {
        *result = SKRResult(SKRResult::Critical, "PLMUpgrader::getProjectDBVersion"_L1, "no_version_found"_L1);
    }

    return dbVersion;
}

SKRResult SKRSqlTools::renumberTreeSortOrder(QSqlDatabase &sqlDb)
{
    SKRResult result("SKRSqlTools::renumberTreeSortOrder"_L1);
    int renumInterval = 1000;

    // Renumber all non-trashed paper in this version. DOES NOT COMMIT - Caller
    // should
    QSqlQuery query(sqlDb);
    QString queryStr = "SELECT l_tree_id"
                       " FROM tbl_tree"
                       " ORDER BY l_sort_order"_L1;

    /*bool prepareOk = */ query.prepare(queryStr);

    //    qDebug() << "prepareOk" << prepareOk;
    //    qDebug() << query.lastError().text();
    query.exec();

    if (query.lastError().isValid())
    {
        result = SKRResult(SKRResult::Critical, "SKRSqlTools::renumberTreeSortOrder"_L1, "sql_error"_L1);
        result.addData("SQLError"_L1, query.lastError().text());
        result.addData("SQL string"_L1, queryStr);
        sqlDb.rollback();
        return result;
    }

    //    qDebug() << query.lastError().text();

    int dest = 0;
    QList<int> list;

    while (query.next())
    {
        list.append(query.value(0).toInt());
    }

    for (int &id : list)
    {
        // For each note to renumber, pass it to the renum function.. For speed
        // we commit after all rows renumbered

        {
            QString queryStr = "UPDATE tbl_tree"
                               " SET l_sort_order = :value"
                               " WHERE l_tree_id = :id"_L1;
            query.prepare(queryStr);
            query.bindValue(":id"_L1, id);
            query.bindValue(":value"_L1, dest);
            query.exec();

            if (query.lastError().isValid())
            {
                result = SKRResult(SKRResult::Critical, "SKRSqlTools::renumberTreeSortOrder"_L1, "sql_error"_L1);
                result.addData("SQLError"_L1, query.lastError().text());
                result.addData("SQL string"_L1, queryStr);
                sqlDb.rollback();

                return result;
            }
        }

        dest += renumInterval;
    }

    return result;
}

// ------------------------------------------------------------------------------

SKRResult SKRSqlTools::trimTreePropertyTable(QSqlDatabase &sqlDb)
{
    SKRResult result("SKRSqlTools::trimTreePropertyTable"_L1);

    QSqlQuery query(sqlDb);

    QList<int> treePropertyIdList;
    QList<int> treeCodeList;

    QString queryStr = "SELECT l_tree_property_id, l_tree_code FROM tbl_tree_property"_L1;

    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid())
    {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTreePropertyTable"_L1, "sql_error"_L1);
        result.addData("SQLError"_L1, query.lastError().text());
        result.addData("SQL string"_L1, queryStr);

        return result;
    }

    while (query.next())
    {
        treePropertyIdList.append(query.value(0).toInt());
        treeCodeList.append(query.value(1).toInt());
    }

    // retreive all tree ids
    QList<int> treeIdList;

    queryStr = "SELECT l_tree_id FROM tbl_tree"_L1;

    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid())
    {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTreePropertyTable"_L1, "sql_error"_L1);
        result.addData("SQLError"_L1, query.lastError().text());
        result.addData("SQL string"_L1, queryStr);
        return result;
    }

    while (query.next())
    {
        treeIdList.append(query.value(0).toInt());
    }

    sqlDb.transaction();

    for (int i = 0; i < treePropertyIdList.count(); i++)
    {
        int treePropertyId = treePropertyIdList.at(i);
        int treeCode = treeCodeList.at(i);

        if (!treeIdList.contains(treeCode))
        {
            queryStr = "DELETE FROM tbl_tree_property WHERE l_tree_property_id = :treePropertyId"_L1;

            query.prepare(queryStr);
            query.bindValue(":treePropertyId"_L1, treePropertyId);
            query.exec();

            if (query.lastError().isValid())
            {
                result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTreePropertyTable"_L1, "sql_error"_L1);
                result.addData("SQLError"_L1, query.lastError().text());
                result.addData("SQL string"_L1, queryStr);
                sqlDb.rollback();
                return result;
            }
        }

        sqlDb.commit();
    }

    return result;
}

// ------------------------------------------------------------------------------

SKRResult SKRSqlTools::trimTagRelationshipTable(QSqlDatabase &sqlDb)
{
    SKRResult result("SKRSqlTools::trimTagRelationshipTable"_L1);

    QSqlQuery query(sqlDb);

    QList<int> tagRelationshipIdList;
    QList<int> treeCodeList;

    QString queryStr = "SELECT l_tag_relationship_id, l_tree_code FROM tbl_tag_relationship"_L1;

    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid())
    {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTagRelationshipTable"_L1, "sql_error"_L1);
        result.addData("SQLError"_L1, query.lastError().text());
        result.addData("SQL string"_L1, queryStr);

        return result;
    }

    while (query.next())
    {
        tagRelationshipIdList.append(query.value(0).toInt());
        treeCodeList.append(query.value(1).toInt());
    }

    // retreive all tree ids
    QList<int> treeIdList;

    queryStr = "SELECT l_tree_id FROM tbl_tree"_L1;

    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid())
    {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTagRelationshipTable"_L1, "sql_error"_L1);
        result.addData("SQLError"_L1, query.lastError().text());
        result.addData("SQL string"_L1, queryStr);
        return result;
    }

    while (query.next())
    {
        treeIdList.append(query.value(0).toInt());
    }

    sqlDb.transaction();

    for (int i = 0; i < tagRelationshipIdList.count(); i++)
    {
        int tagRelationshipId = tagRelationshipIdList.at(i);
        int treeCode = treeCodeList.at(i);

        if (!treeIdList.contains(treeCode))
        {
            queryStr = "DELETE FROM tbl_tag_relationship WHERE l_tag_relationship_id = :tagRelationshipId"_L1;

            query.prepare(queryStr);
            query.bindValue(":treePropertyId"_L1, tagRelationshipId);
            query.exec();

            if (query.lastError().isValid())
            {
                result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTagRelationshipTable"_L1, "sql_error"_L1);
                result.addData("SQLError"_L1, query.lastError().text());
                result.addData("SQL string"_L1, queryStr);
                sqlDb.rollback();
                return result;
            }
        }

        sqlDb.commit();
    }

    return result;
}

// ------------------------------------------------------------------------------

SKRResult SKRSqlTools::trimTreeRelationshipTable(QSqlDatabase &sqlDb)
{
    SKRResult result("SKRSqlTools::trimTreeRelationshipTable"_L1);

    QSqlQuery query(sqlDb);

    QList<int> treeRelationshipIdList;
    QList<int> treeSourceCodeList;
    QList<int> treeReceiverCodeList;

    QString queryStr =
        "SELECT l_tree_relationship_id, l_tree_source_code, l_tree_receiver_code FROM tbl_tree_relationship"_L1;

    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid())
    {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTreeRelationshipTable"_L1, "sql_error"_L1);
        result.addData("SQLError"_L1, query.lastError().text());
        result.addData("SQL string"_L1, queryStr);

        return result;
    }

    while (query.next())
    {
        treeRelationshipIdList.append(query.value(0).toInt());
        treeSourceCodeList.append(query.value(1).toInt());
        treeReceiverCodeList.append(query.value(2).toInt());
    }

    // retreive all tree ids
    QList<int> treeIdList;

    queryStr = "SELECT l_tree_id FROM tbl_tree"_L1;

    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid())
    {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTreeRelationshipTable"_L1, "sql_error"_L1);
        result.addData("SQLError"_L1, query.lastError().text());
        result.addData("SQL string"_L1, queryStr);
        return result;
    }

    while (query.next())
    {
        treeIdList.append(query.value(0).toInt());
    }

    sqlDb.transaction();

    for (int i = 0; i < treeRelationshipIdList.count(); i++)
    {
        int treeRelationshipId = treeRelationshipIdList.at(i);
        int treeSourceCode = treeSourceCodeList.at(i);
        int treeReceiverCode = treeReceiverCodeList.at(i);

        if (!treeIdList.contains(treeSourceCode) || !treeIdList.contains(treeReceiverCode))
        {
            queryStr = "DELETE FROM tbl_tree_relationship WHERE l_tree_relationship_id = :treeRelationshipId"_L1;

            query.prepare(queryStr);
            query.bindValue(":treeRelationshipId"_L1, treeRelationshipId);
            query.exec();

            if (query.lastError().isValid())
            {
                result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTreeRelationshipTable"_L1, "sql_error"_L1);
                result.addData("SQLError"_L1, query.lastError().text());
                result.addData("SQL string"_L1, queryStr);
                sqlDb.rollback();
                return result;
            }
        }

        sqlDb.commit();
    }

    return result;
}

// ------------------------------------------------------------------------------

SKRResult SKRSqlTools::addStringTreeProperty(QSqlDatabase &sqlDb, int tree_id, const QString &name,
                                             const QString &value)
{
    SKRResult result("SKRSqlTools::addStringTreeProperty"_L1);

    QSqlQuery query(sqlDb);

    QString queryStr = "INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value)"
                       "VALUES (:treeCode, :name, 'STRING', :value)"_L1;

    query.prepare(queryStr);
    query.bindValue(":treeCode"_L1, tree_id);
    query.bindValue(":name"_L1, name);
    query.bindValue(":value"_L1, value);
    query.exec();

    if (query.lastError().isValid())
    {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::addStringTreeProperty"_L1, "sql_error"_L1);
        result.addData("SQLError"_L1, query.lastError().text());
        result.addData("SQL string"_L1, queryStr);
    }

    return result;
}
} // namespace Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule
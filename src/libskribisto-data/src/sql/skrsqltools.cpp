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
    QFile     file(fileName);

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

        // qDebug() << queryStr;

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
            if (re_transaction.match(s).hasMatch()) {
                sqlDB.transaction();
            }
            else if (re_commit.match(s).hasMatch()) {
                sqlDB.commit();
            }
            else {
                query.exec(s);

                if (query.lastError().type() != QSqlError::NoError) {
                    result = SKRResult(SKRResult::Critical, "SKRSqlTools::executeSQLString", "sql_error");
                    result.addData("SQLError",   query.lastError().text());
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
                result.addData("SQLError",   query.lastError().text());
                result.addData("SQL string", s);
                sqlDB.rollback();
                return result;
            }
        }
        sqlDB.commit();
    }
    return result;
}

// ------------------------------------------------------------------

QString SKRSqlTools::getProjectTemplateDBVersion(SKRResult *result) {
    QString dbVersion = "-2";
    QFile   file(":/sql/sqlite_project.sql");

    file.open(QIODevice::ReadOnly);

    for (const QString& line : QString(file.readAll()).split("\n")) {
        if (line.contains("-- skribisto_db_version:")) {
            QStringList splittedLine = line.split(":");

            if (splittedLine.count() == 2) {
                dbVersion = splittedLine.at(1);
            }

            break;
        }
    }
    file.close();

    if (dbVersion == "-2") {
        *result = SKRResult(SKRResult::Critical,
                            "SKRSqlTools::getProjectTemplateVersion",
                            "no_db_version_found_in_sql_file");
    }

    return dbVersion;
}

// ------------------------------------------------------------------

double SKRSqlTools::getProjectDBVersion(SKRResult *result, const QString& sqlDbConnectionName) {
    double dbVersion = -1;

    QSqlQuery query(QSqlDatabase::database(sqlDbConnectionName));
    QString   queryStr = "SELECT dbl_database_version FROM tbl_project";


    query.prepare(queryStr);
    query.exec();

    while (query.next()) {
        dbVersion = query.value(0).toDouble();
    }

    if (dbVersion == -1) {
        *result = SKRResult(SKRResult::Critical, "PLMUpgrader::getProjectDBVersion", "no_version_found");
    }

    return dbVersion;
}

SKRResult SKRSqlTools::renumberTreeSortOrder(QSqlDatabase& sqlDb)
{
    SKRResult result("SKRSqlTools::renumberTreeSortOrder");
    int renumInterval = 1000;

    // Renumber all non-trashed paper in this version. DOES NOT COMMIT - Caller
    // should
    QSqlQuery query(sqlDb);
    QString   queryStr = "SELECT l_tree_id"
                         " FROM tbl_tree"
                         " ORDER BY l_sort_order"
    ;

    /*bool prepareOk = */ query.prepare(queryStr);

    //    qDebug() << "prepareOk" << prepareOk;
    //    qDebug() << query.lastError().text();
    query.exec();

    if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "SKRSqlTools::renumberTreeSortOrder", "sql_error");
        result.addData("SQLError",   query.lastError().text());
        result.addData("SQL string", queryStr);
        sqlDb.rollback();
        return result;
    }

    //    qDebug() << query.lastError().text();

    int dest = 0;
    QList<int> list;

    while (query.next()) {
        list.append(query.value(0).toInt());
    }

    for (int& id : list) {
        // For each note to renumber, pass it to the renum function.. For speed
        // we commit after all rows renumbered


        {
            QString queryStr = "UPDATE tbl_tree"
                               " SET l_sort_order = :value"
                               " WHERE l_tree_id = :id"
            ;
            query.prepare(queryStr);
            query.bindValue(":id",    id);
            query.bindValue(":value", dest);
            query.exec();

            if (query.lastError().isValid()) {
                result = SKRResult(SKRResult::Critical, "SKRSqlTools::renumberTreeSortOrder", "sql_error");
                result.addData("SQLError",   query.lastError().text());
                result.addData("SQL string", queryStr);
                sqlDb.rollback();

                return result;
            }
        }


        dest += renumInterval;
    }

    return result;
}

// ------------------------------------------------------------------------------

SKRResult SKRSqlTools::trimTreePropertyTable(QSqlDatabase& sqlDb)
{
    SKRResult result("SKRSqlTools::trimTreePropertyTable");


    QSqlQuery query(sqlDb);

    QList<int> treePropertyIdList;
    QList<int> treeCodeList;

    QString queryStr = "SELECT l_tree_property_id, l_tree_code FROM tbl_tree_property";


    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTreePropertyTable", "sql_error");
        result.addData("SQLError",   query.lastError().text());
        result.addData("SQL string", queryStr);

        return result;
    }


    while (query.next()) {
        treePropertyIdList.append(query.value(0).toInt());
        treeCodeList.append(query.value(1).toInt());
    }


    // retreive all tree ids
    QList<int> treeIdList;

    queryStr = "SELECT l_tree_id FROM tbl_tree";


    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTreePropertyTable", "sql_error");
        result.addData("SQLError",   query.lastError().text());
        result.addData("SQL string", queryStr);
        return result;
    }


    while (query.next()) {
        treeIdList.append(query.value(0).toInt());
    }


    sqlDb.transaction();

    for (int i = 0; i < treePropertyIdList.count(); i++) {
        int treePropertyId = treePropertyIdList.at(i);
        int treeCode       = treeCodeList.at(i);


        if (!treeIdList.contains(treeCode)) {
            queryStr = "DELETE FROM tbl_tree_property WHERE l_tree_property_id = :treePropertyId";


            query.prepare(queryStr);
            query.bindValue(":treePropertyId", treePropertyId);
            query.exec();

            if (query.lastError().isValid()) {
                result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTreePropertyTable", "sql_error");
                result.addData("SQLError",   query.lastError().text());
                result.addData("SQL string", queryStr);
                sqlDb.rollback();
                return result;
            }
        }

        sqlDb.commit();
    }

    return result;
}

// ------------------------------------------------------------------------------

SKRResult SKRSqlTools::trimTagRelationshipTable(QSqlDatabase& sqlDb)
{
    SKRResult result("SKRSqlTools::trimTagRelationshipTable");


    QSqlQuery query(sqlDb);

    QList<int> tagRelationshipIdList;
    QList<int> treeCodeList;

    QString queryStr = "SELECT l_tag_relationship_id, l_tree_code FROM tbl_tag_relationship";


    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTagRelationshipTable", "sql_error");
        result.addData("SQLError",   query.lastError().text());
        result.addData("SQL string", queryStr);

        return result;
    }


    while (query.next()) {
        tagRelationshipIdList.append(query.value(0).toInt());
        treeCodeList.append(query.value(1).toInt());
    }


    // retreive all tree ids
    QList<int> treeIdList;

    queryStr = "SELECT l_tree_id FROM tbl_tree";


    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTagRelationshipTable", "sql_error");
        result.addData("SQLError",   query.lastError().text());
        result.addData("SQL string", queryStr);
        return result;
    }


    while (query.next()) {
        treeIdList.append(query.value(0).toInt());
    }


    sqlDb.transaction();

    for (int i = 0; i < tagRelationshipIdList.count(); i++) {
        int tagRelationshipId = tagRelationshipIdList.at(i);
        int treeCode          = treeCodeList.at(i);


        if (!treeIdList.contains(treeCode)) {
            queryStr = "DELETE FROM tbl_tag_relationship WHERE l_tag_relationship_id = :tagRelationshipId";


            query.prepare(queryStr);
            query.bindValue(":treePropertyId", tagRelationshipId);
            query.exec();

            if (query.lastError().isValid()) {
                result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTagRelationshipTable", "sql_error");
                result.addData("SQLError",   query.lastError().text());
                result.addData("SQL string", queryStr);
                sqlDb.rollback();
                return result;
            }
        }

        sqlDb.commit();
    }

    return result;
}

// ------------------------------------------------------------------------------

SKRResult SKRSqlTools::trimTreeRelationshipTable(QSqlDatabase& sqlDb)
{
    SKRResult result("SKRSqlTools::trimTreeRelationshipTable");


    QSqlQuery query(sqlDb);

    QList<int> treeRelationshipIdList;
    QList<int> treeSourceCodeList;
    QList<int> treeReceiverCodeList;

    QString queryStr =
        "SELECT l_tree_relationship_id, l_tree_source_code, l_tree_receiver_code FROM tbl_tree_relationship";


    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTreeRelationshipTable", "sql_error");
        result.addData("SQLError",   query.lastError().text());
        result.addData("SQL string", queryStr);

        return result;
    }


    while (query.next()) {
        treeRelationshipIdList.append(query.value(0).toInt());
        treeSourceCodeList.append(query.value(1).toInt());
        treeReceiverCodeList.append(query.value(2).toInt());
    }


    // retreive all tree ids
    QList<int> treeIdList;

    queryStr = "SELECT l_tree_id FROM tbl_tree";


    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTreeRelationshipTable", "sql_error");
        result.addData("SQLError",   query.lastError().text());
        result.addData("SQL string", queryStr);
        return result;
    }


    while (query.next()) {
        treeIdList.append(query.value(0).toInt());
    }


    sqlDb.transaction();

    for (int i = 0; i < treeRelationshipIdList.count(); i++) {
        int treeRelationshipId = treeRelationshipIdList.at(i);
        int treeSourceCode     = treeSourceCodeList.at(i);
        int treeReceiverCode   = treeReceiverCodeList.at(i);


        if (!treeIdList.contains(treeSourceCode) || !treeIdList.contains(treeReceiverCode)) {
            queryStr = "DELETE FROM tbl_tree_relationship WHERE l_tree_relationship_id = :treeRelationshipId";


            query.prepare(queryStr);
            query.bindValue(":treeRelationshipId", treeRelationshipId);
            query.exec();

            if (query.lastError().isValid()) {
                result = SKRResult(SKRResult::Critical, "PLMUpgrader::trimTreeRelationshipTable", "sql_error");
                result.addData("SQLError",   query.lastError().text());
                result.addData("SQL string", queryStr);
                sqlDb.rollback();
                return result;
            }
        }

        sqlDb.commit();
    }

    return result;
}

// ------------------------------------------------------------------------------

SKRResult SKRSqlTools::addStringTreeProperty(QSqlDatabase& sqlDb, int tree_id, const QString& name,
                                             const QString& value)
{
    SKRResult result("SKRSqlTools::addStringTreeProperty");


    QSqlQuery query(sqlDb);

    QString queryStr = "INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value)"
                       "VALUES (:treeCode, :name, 'STRING', :value)";


    query.prepare(queryStr);
    query.bindValue(":treeCode", tree_id);
    query.bindValue(":name",     name);
    query.bindValue(":value",    value);
    query.exec();

    if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "PLMUpgrader::addStringTreeProperty", "sql_error");
        result.addData("SQLError",   query.lastError().text());
        result.addData("SQL string", queryStr);
    }


    return result;
}

#pragma once
#include "QtConcurrent/qtconcurrenttask.h"
#include "QtSql/qsqlerror.h"
#include "database/entity_table_sql_generator.h"
#include "database/interface_database_context.h"
#include "dummy_entity.h"
#include <QSqlDatabase>
#include <QSqlQuery>
#include <QString>
#include <QThreadPool>
#include <QUuid>

class DummyDatabaseContext : public Contracts::Database::InterfaceDatabaseContext
{
  public:
    DummyDatabaseContext();
    ~DummyDatabaseContext();

  private:
    QString m_databaseName;

    // InterfaceDatabaseContext interface
  public:
    QSqlDatabase getConnection() override;
};

DummyDatabaseContext::DummyDatabaseContext()
{

    m_databaseName = ":memory:";
    auto db = DummyDatabaseContext::getConnection();

    {
        QSqlQuery query(db);

        QStringList sqlList;
        sqlList << Database::EntityTableSqlGenerator::generateEntitySql<Domain::DummyEntity>();

        for (const QString &queryStr : sqlList)
        {
            qDebug() << queryStr;
            if (!query.prepare(queryStr))
            {
                qCritical() << query.lastError().text() << queryStr;
            }
            if (!query.exec())
            {
                qCritical() << query.lastError().text() << queryStr;
            }
        }
    }
}

DummyDatabaseContext::~DummyDatabaseContext()
{
}

QSqlDatabase DummyDatabaseContext::getConnection()
{
    QString connectionName = QString("Thread_%1").arg(uintptr_t(QThread::currentThreadId()));
    if (!QSqlDatabase::contains(connectionName))
    {
        QSqlDatabase database = QSqlDatabase::addDatabase("QSQLITE", connectionName);
        database.setDatabaseName(m_databaseName);
        if (!database.open())
        {
            QSqlDatabase::removeDatabase(connectionName);
            qDebug() << Q_FUNC_INFO << "sql_error" << database.lastError().text();
        }
    }
    qDebug() << QSqlDatabase::connectionNames();

    return QSqlDatabase::database(connectionName);
}

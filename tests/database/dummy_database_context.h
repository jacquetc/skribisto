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

    QString databaseName() const override;
    QThreadPool &threadPool() override;

  private:
    QThreadPool m_threadPool;
    QString m_databaseName;
};

DummyDatabaseContext::DummyDatabaseContext()
{
    m_threadPool.setMaxThreadCount(1);
    m_threadPool.setExpiryTimeout(0);

    QtConcurrent::task([this]() {
        m_databaseName = QUuid::createUuid().toString();
        auto db = QSqlDatabase::addDatabase("QSQLITE", ":memory:");
        db.open();

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
    })
        .onThreadPool(m_threadPool)
        .spawn()
        .waitForFinished();
}

DummyDatabaseContext::~DummyDatabaseContext()
{
    // QSqlDatabase::removeDatabase(":memory:");
}

QString DummyDatabaseContext::databaseName() const
{
    return ":memory:";
}

QThreadPool &DummyDatabaseContext::threadPool()
{
    return m_threadPool;
}

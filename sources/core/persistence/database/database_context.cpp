#include "database_context.h"
#include "QtSql/qsqlerror.h"
#include <QSqlDatabase>
#include <QSqlQuery>
#include <QString>
#include <QTemporaryFile>

using namespace Database;

DatabaseContext::DatabaseContext() : QObject()
{
}

DatabaseContext::~DatabaseContext()
{
    // QFile::remove(m_databaseName);
}

//-------------------------------------------------

Result<void> DatabaseContext::init()
{

    Result<QString> databaseNameResult = createEmptyDatabase();

    if (databaseNameResult.isError())
    {
        return Result<void>(databaseNameResult.error());
    }

    return Result<void>();
}

//-------------------------------------------------

QSqlDatabase DatabaseContext::getConnection()
{
    QMutexLocker locker(&mutex);
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
    // qDebug() << QSqlDatabase::connectionNames();

    return QSqlDatabase::database(connectionName);
}

void DatabaseContext::setSqlEmptyDatabaseQueryFunction(const SqlEmptyDatabaseQueryFunction &function)
{
    m_sqlEmptyDatabaseQueryFunction = function;
}

QStringList DatabaseContext::entityClassNames() const
{
    return m_entityClassNames;
}

void DatabaseContext::setEntityClassNames(const QStringList &newEntityClassNames)
{
    m_entityClassNames = newEntityClassNames;
}

//-------------------------------------------------

Result<QString> DatabaseContext::createEmptyDatabase()
{
    QString databaseName;

    // create a temporary file to copy the database to
    QTemporaryFile tempFile;
    tempFile.open();
    tempFile.setAutoRemove(false);
    QString tempFileName = tempFile.fileName();

    {
        m_databaseName = tempFileName;
        databaseName = m_databaseName;
        // qDebug() << m_databaseName;

        QSqlDatabase sqlDb = getConnection();

        // start a transaction
        sqlDb.transaction();

        // execute each table creation as a single query within the transaction
        QSqlQuery query(sqlDb);
        for (const QString &string : this->sqlEmptyDatabaseQuery())
        {
            if (!query.prepare(string))
            {
                return Result<QString>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), string));
            }
            if (!query.exec())
            {
                return Result<QString>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), string));
            }
        }

        // database optimization options
        QStringList optimization;
        optimization << QStringLiteral("PRAGMA case_sensitive_like=true")
                     << QStringLiteral("PRAGMA journal_mode=MEMORY") << QStringLiteral("PRAGMA temp_store=MEMORY")
                     << QStringLiteral("PRAGMA locking_mode=NORMAL") << QStringLiteral("PRAGMA synchronous = OFF")
                     << QStringLiteral("PRAGMA recursive_triggers=true");

        // execute each optimization option as a single query within the transaction

        for (const QString &string : qAsConst(optimization))
        {
            query.prepare(string);
            query.exec();
        }

        sqlDb.commit();
    }

    // return the name of the copied database file
    return Result<QString>(databaseName);
}

QStringList DatabaseContext::sqlEmptyDatabaseQuery() const
{
    return m_sqlEmptyDatabaseQueryFunction();
}

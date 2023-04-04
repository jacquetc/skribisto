#include "database_context.h"
#include "QtConcurrent/qtconcurrenttask.h"
#include "atelier.h"
#include "author.h"
#include "entity_table_sql_generator.h"
#include <QSqlDatabase>
#include <QSqlQuery>
#include <QString>
#include <QTemporaryFile>

using namespace Database;

DatabaseContext::DatabaseContext() : QObject()
{
    m_threadPool.setMaxThreadCount(1);
    m_threadPool.setExpiryTimeout(0);
}

DatabaseContext::~DatabaseContext()
{
}

//-------------------------------------------------

Result<void> DatabaseContext::init()
{

    Result<QString> databaseNameResult = createEmptyDatabase();

    if (databaseNameResult.isError())
    {
        return Result<void>(databaseNameResult.error());
    }

    m_databaseName = databaseNameResult.value();

    return Result<void>();
}

//-------------------------------------------------

QString DatabaseContext::databaseName() const
{
    return m_databaseName;
}

//-------------------------------------------------

QUrl DatabaseContext::fileName() const
{
    return m_fileName;
}

//-------------------------------------------------

void DatabaseContext::setFileName(const QUrl &newFileName)
{
    m_fileName = newFileName;
}

//-------------------------------------------------

Result<QString> DatabaseContext::createEmptyDatabase()
{
    return QtConcurrent::task([this]() {
               // create a temporary file to copy the database to
               QTemporaryFile tempFile;
               tempFile.open();
               tempFile.setAutoRemove(false);
               QString tempFileName = tempFile.fileName();

               QSqlDatabase sqlDb = QSqlDatabase::addDatabase("QSQLITE", tempFileName);
               sqlDb.setHostName("localhost");
               sqlDb.setDatabaseName(tempFileName);

               // try to open the copied database file
               bool ok = sqlDb.open();

               if (!ok)
               {
                   return Result<QString>(Error(Q_FUNC_INFO, Error::Critical, "cant_open_database",
                                                "Can't open database " + tempFileName, tempFileName));
               }
               // start a transaction
               sqlDb.transaction();

               // execute each table creation as a single query within the transaction
               QSqlQuery query(sqlDb);
               for (const QString &string : this->SqlEmptyDatabaseQuery())
               {
                   query.prepare(string);
                   query.exec();
               }

               // database optimization options
               QStringList optimization;
               optimization << QStringLiteral("PRAGMA case_sensitive_like=true")
                            << QStringLiteral("PRAGMA journal_mode=MEMORY")
                            << QStringLiteral("PRAGMA temp_store=MEMORY")
                            << QStringLiteral("PRAGMA locking_mode=EXCLUSIVE")
                            << QStringLiteral("PRAGMA synchronous = OFF")
                            << QStringLiteral("PRAGMA recursive_triggers=true");

               // execute each optimization option as a single query within the transaction

               for (const QString &string : qAsConst(optimization))
               {
                   query.prepare(string);
                   query.exec();
               }

               sqlDb.commit();

               // return the name of the copied database file
               return Result<QString>(sqlDb.databaseName());
           })
        .onThreadPool(m_threadPool)
        .spawn()
        .result();
}

QStringList DatabaseContext::SqlEmptyDatabaseQuery() const
{
    QStringList queryList;

    queryList << EntityTableSqlGenerator::generateEntitySql<Domain::Author>();
    queryList << EntityTableSqlGenerator::generateEntitySql<Domain::Atelier>();
    queryList << EntityTableSqlGenerator::generateEntitySql<Domain::Book>();

    return queryList;
}

QThreadPool &DatabaseContext::threadPool()
{
    return m_threadPool;
}

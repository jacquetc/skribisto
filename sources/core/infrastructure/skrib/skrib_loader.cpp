#include "skrib_loader.h"
#include "persistence/interface_author_repository.h"

#include <QSqlDatabase>
#include <QSqlQuery>
#include <QTemporaryFile>

using namespace Infrastructure::Skrib;
using namespace Contracts::Persistence;

SkribLoader::SkribLoader(InterfaceRepositoryProvider *repositoryProvider) : m_repositoryProvider(repositoryProvider)
{
}

Result<void> SkribLoader::load(const LoadSystemDTO &dto)
{
    QString databaseName;
    // open
    Result<void> result = loadDatabase(dto.fileName(), databaseName);
    if (result.hasError())
    {
        return result;
    }

    // update
    result = updateDatabase(databaseName);
    if (result.hasError())
    {
        return result;
    }

    // fill repositories

    result = fillRepositories(m_repositoryProvider, databaseName);
    if (result.hasError())
    {
        return result;
    }

    // close database and clean up
    result = closeDatabase(databaseName);
    if (result.hasError())
    {
        return result;
    }

    return Result<void>();
}

Result<void> SkribLoader::loadDatabase(const QUrl &fileName, QString &databaseName)
{
    // create a temporary file to copy the database to
    QTemporaryFile tempFile;
    tempFile.open();
    tempFile.setAutoRemove(false);
    QString tempFileName = tempFile.fileName();

    // copy db file to temp
    QString fileNameString;

    // check the file scheme, and create a valid file name string
    if (fileName.scheme() == "qrc")
    {
        fileNameString = fileName.toString(QUrl::RemoveScheme);
        fileNameString = ":" + fileNameString;
    }
    else if (fileName.path().at(2) == ':')
    { // means Windows
        fileNameString = fileName.path().remove(0, 1);
    }
    else
    {
        fileNameString = fileName.path();
    }

    // open the original database file and copy its contents to the temporary file
    QFile file(fileNameString);

    if (!file.exists())
    {
        return Result<void>(Error("SkribLoader", Error::Critical, "absent_filename", fileNameString + " doesn't exist",
                                  fileNameString));
    }

    if (!file.open(QIODevice::ReadOnly))
    {
        return Result<void>(Error("SkribLoader", Error::Critical, "readonly_filename",
                                  fileNameString + " can't be opened", fileNameString));
    }

    QByteArray array(file.readAll());
    tempFile.write(array);
    tempFile.close();

    // open temp file
    QSqlDatabase sqlDb = QSqlDatabase::addDatabase("QSQLITE", tempFileName);
    sqlDb.setHostName("localhost");
    sqlDb.setDatabaseName(tempFileName);

    // try to open the copied database file
    bool ok = sqlDb.open();

    if (!ok)
    {
        return Result<void>(Error("SkribLoader", Error::Critical, "cant_open_database",
                                  "Can't open database " + tempFileName, tempFileName));
    }

    // return the name of the copied database file
    databaseName = sqlDb.databaseName();
    return Result<void>();
}

Result<void> SkribLoader::updateDatabase(QString &databaseName)
{
    QSqlDatabase sqlDb = QSqlDatabase::database(databaseName);
    // database optimization options
    QStringList optimization;
    optimization << QStringLiteral("PRAGMA case_sensitive_like=true") << QStringLiteral("PRAGMA journal_mode=MEMORY")
                 << QStringLiteral("PRAGMA temp_store=MEMORY") << QStringLiteral("PRAGMA locking_mode=EXCLUSIVE")
                 << QStringLiteral("PRAGMA synchronous = OFF") << QStringLiteral("PRAGMA recursive_triggers=true");

    // start a transaction and execute each optimization option as a separate query
    sqlDb.transaction();

    for (const QString &string : qAsConst(optimization))
    {
        QSqlQuery query(sqlDb);

        query.prepare(string);
        query.exec();
    }

    sqlDb.commit();

    // TODO: upgrade :
    //     Result result = Upgrader::upgradeSQLite(databaseName));
    //     if (result)
    //     {
    //         return Result<void>(
    //             Error(this, Error::Critical, "upgrade_sqlite_failed", "Upgrade of database failed",
    //             tempFileName));
    //     }

    return Result<void>();
}

Result<void> SkribLoader::fillRepositories(InterfaceRepositoryProvider *repositoryProvider, QString &databaseName)
{
    QSqlDatabase sqlDb = QSqlDatabase::database(databaseName);

    // author:
    auto authorRepository = qSharedPointerCast<InterfaceAuthorRepository>(
        repositoryProvider->repository(InterfaceRepositoryProvider::Author));

    QSqlQuery query(sqlDb);
    query.prepare("SELECT * FROM author");
    query.exec();

    while (query.next())
    {
        QUuid uuid = query.value("uuid").toUuid();
        QString name = query.value("name").toString();
        QUuid relative = query.value("relative").toUuid();
        QDateTime creationDate = query.value("creationDate").toDateTime();
        QDateTime updateDate = query.value("updateDate").toDateTime();

        Domain::Author author(uuid, name, relative, creationDate, updateDate);
        Result<Domain::Author> addResult = authorRepository->add(std::move(author));

        if (addResult.hasError())
        {
            // Handle the error if necessary, e.g., log it, return an error Result, etc.
            return Result<void>(addResult.error());
        }
    }

    return Result<void>();
}

Result<void> SkribLoader::closeDatabase(QString &databaseName)
{
    QSqlDatabase::removeDatabase(databaseName);
    QFile file(databaseName);
    bool removalResult = file.remove();

    if (!removalResult)
    {
        return Result<void>(
            Error("SkribLoader", Error::Critical, "tempfile_not_removed", databaseName + " not removed", databaseName));
    }
    return Result<void>();
}

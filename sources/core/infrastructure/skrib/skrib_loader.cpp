#include "skrib_loader.h"
#include "persistence/interface_author_repository.h"

#include <QSqlDatabase>
#include <QSqlQuery>
#include <QTemporaryFile>

using namespace Infrastructure::Skrib;
using namespace Contracts::Persistence;

SkribLoader::SkribLoader(InterfaceRepositoryProvider *repositoryProvider) : m_repositoryProvider(repositoryProvider)
{}

Result<void>SkribLoader::load(QPromise<Result<void> >& progressPromise, const LoadSystemDTO& dto)
{
    QString databaseName;

    // open
    Result<void> result = loadDatabase(progressPromise, dto.fileName(), databaseName);

    if (result.hasError())
    {
        return result;
    }

    // update
    result = updateDatabase(progressPromise, databaseName);

    if (result.hasError())
    {
        return result;
    }

    // fill repositories

    result = fillRepositories(progressPromise, m_repositoryProvider, databaseName);

    if (result.hasError())
    {
        return result;
    }

    // close database and clean up
    result = closeDatabase(progressPromise, databaseName);

    if (result.hasError())
    {
        return result;
    }

    return Result<void>();
}

Result<void>SkribLoader::loadDatabase(QPromise<Result<void> >& progressPromise, const QUrl& fileName,
                                      QString& databaseName)
{
    progressPromise.setProgressValueAndText(0, "loading file");

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

    // open the original database file and copy its contents to the temporary
    // file
    QFile file(fileNameString);

    if (!file.exists())
    {
        return Result<void>(
            Error(Q_FUNC_INFO, Error::Critical, "absent_filename", fileNameString + " doesn't exist", fileNameString));
    }

    if (!file.open(QIODevice::ReadOnly))
    {
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "readonly_filename",
                                  fileNameString + " can't be opened", fileNameString));
    }

    // copy to temp file
    QByteArray array(file.readAll());
    tempFile.write(array);
    tempFile.close();

    // open temp file
    QSqlDatabase sqlDb = QSqlDatabase::addDatabase("QSQLITE", tempFileName);
    sqlDb.setDatabaseName(tempFileName);

    // try to open the copied database file
    bool ok = sqlDb.open();

    if (!ok)
    {
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "cant_open_database",
                                  "Can't open database " + tempFileName, tempFileName));
    }

    // return the name of the copied database file
    databaseName = sqlDb.databaseName();

    return Result<void>();
}

Result<void>SkribLoader::updateDatabase(QPromise<Result<void> >& progressPromise, QString& databaseName)
{
    progressPromise.setProgressValueAndText(5, "updating");

    QSqlDatabase sqlDb = QSqlDatabase::database(databaseName);

    // database optimization options
    QStringList optimization;
    optimization << QStringLiteral("PRAGMA case_sensitive_like=true") << QStringLiteral("PRAGMA journal_mode=MEMORY")
                 << QStringLiteral("PRAGMA temp_store=MEMORY") << QStringLiteral("PRAGMA locking_mode=EXCLUSIVE")
                 << QStringLiteral("PRAGMA synchronous = OFF") << QStringLiteral("PRAGMA recursive_triggers=true");

    // start a transaction and execute each optimization option as a separate
    // query
    sqlDb.transaction();

    for (const QString& string : qAsConst(optimization))
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
    //             Error(Q_FUNC_INFO, Error::Critical, "upgrade_sqlite_failed",
    // "Upgrade of database failed",
    //             tempFileName));
    //     }

    return Result<void>();
}

Result<void>SkribLoader::fillRepositories(QPromise<Result<void> >& progressPromise,
                                          InterfaceRepositoryProvider *repositoryProvider, QString& databaseName)
{
    progressPromise.setProgressValueAndText(10, "loading content");
    int progressIndex   = 1;
    int repositoryCount = 3;

    QSqlDatabase sqlDb = QSqlDatabase::database(databaseName);

    // -----------------------------------
    // author
    // -----------------------------------

    auto authorRepository = qSharedPointerCast<InterfaceAuthorRepository>(
        repositoryProvider->repository("author"));

    QSqlQuery query(sqlDb);
    query.prepare("SELECT * FROM author");
    query.exec();

    authorRepository->beginChanges();

    while (query.next())
    {
        int       id           = query.value("id").toInt();
        QUuid     uuid         = query.value("uuid").toUuid();
        QString   name         = query.value("name").toString();
        QDateTime creationDate = query.value("creationDate").toDateTime();
        QDateTime updateDate   = query.value("updateDate").toDateTime();

        Domain::Author author(id, uuid, creationDate, updateDate, name);
        Result<Domain::Author> addResult = authorRepository->add(std::move(author));

        if (addResult.hasError())
        {
            // Handle the error if necessary, e.g., log it, return an error
            // Result, etc.
            return Result<void>(addResult.error());
        }
    }
    progressPromise.setProgressValueAndText(20 + progressIndex++ * (80 / repositoryCount), "loading content");
    authorRepository->saveChanges();

    int waitTime = 0;

    while (waitTime < 1000000000)
    {
        waitTime++;
    }

    progressPromise.setProgressValueAndText(20 + progressIndex++ * (80 / repositoryCount), "loading content");

    waitTime = 0;

    while (waitTime < 1000000000)
    {
        waitTime++;
    }

    progressPromise.setProgressValueAndText(20 + progressIndex++ * (80 / repositoryCount), "loading content");

    waitTime = 0;

    while (waitTime < 1000000000)
    {
        waitTime++;
    }

    return Result<void>();
}

Result<void>SkribLoader::closeDatabase(QPromise<Result<void> >& progressPromise, QString& databaseName)
{
    QSqlDatabase::removeDatabase(databaseName);
    QFile file(databaseName);
    bool  removalResult = file.remove();

    if (!removalResult)
    {
        return Result<void>(
            Error(Q_FUNC_INFO, Error::Critical, "tempfile_not_removed", databaseName + " not removed", databaseName));
    }

    progressPromise.setProgressValueAndText(99, "loading finished");

    return Result<void>();
}

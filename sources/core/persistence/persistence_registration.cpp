#include "persistence_registration.h"
#include "database/database_context.h"
#include "database/database_table.h"
#include "database/entity_table_sql_generator.h"
#include "repositories/atelier_repository.h"
#include "repositories/author_repository.h"
#include "repositories/book_repository.h"
#include "repositories/chapter_repository.h"
#include "repositories/repository_provider.h"

using namespace Repository;
using namespace Database;
using namespace Persistence;

PersistenceRegistration::PersistenceRegistration(QObject *parent) : QObject{parent}
{
    auto *context = new DatabaseContext();

    QStringList entityClassNameList;

    entityClassNameList << "Author"
                        << "Scene"
                        << "Chapter"
                        << "Book"
                        << "Atelier";

    context->setEntityClassNames(entityClassNameList);

    // create the empty internal database:
    context->setSqlEmptyDatabaseQueryFunction([entityClassNameList]() {
        QStringList queryList;
        EntityTableSqlGenerator sqlGenerator(entityClassNameList);

        queryList << sqlGenerator.generateEntitySql<Domain::Author>();
        queryList << sqlGenerator.generateEntitySql<Domain::Chapter>();
        queryList << sqlGenerator.generateEntitySql<Domain::Book>();
        queryList << sqlGenerator.generateEntitySql<Domain::Atelier>();

        return queryList;
    });

    Result<void> initResult = context->init();

    if (initResult.hasError())
    {
        Error error = initResult.error();
        qCritical() << error.className() + "\n" + error.code() + "\n" + error.message() + "\n" + error.data();
    }

    // database tables:
    auto authorDatabaseTable  = new DatabaseTable<Domain::Author>(context);
    auto chapterDatabaseTable = new DatabaseTable<Domain::Chapter>(context);
    auto bookDatabaseTable    = new DatabaseTable<Domain::Book>(context);
    auto atelierDatabaseTable = new DatabaseTable<Domain::Atelier>(context);

    // repositories:
    QSharedPointer<AuthorRepository>  authorRepository(new AuthorRepository(authorDatabaseTable));
    QSharedPointer<ChapterRepository> chapterRepository(new ChapterRepository(chapterDatabaseTable));
    QSharedPointer<BookRepository>    bookRepository(new BookRepository(bookDatabaseTable, chapterDatabaseTable));
    QSharedPointer<AtelierRepository> atelierRepository(new AtelierRepository(atelierDatabaseTable, bookDatabaseTable));

    // register repositories:
    Repository::RepositoryProvider::instance()->registerRepository("author", authorRepository);
    Repository::RepositoryProvider::instance()->registerRepository("atelier", atelierRepository);
    Repository::RepositoryProvider::instance()->registerRepository("book", bookRepository);
    Repository::RepositoryProvider::instance()->registerRepository("chapter", chapterRepository);
}

RepositoryProvider * PersistenceRegistration::repositoryProvider()
{
    return Repository::RepositoryProvider::instance();
}

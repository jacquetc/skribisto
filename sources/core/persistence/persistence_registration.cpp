#include "persistence_registration.h"
#include "database/database_context.h"
#include "database/database_table.h"
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
    Result<void> initResult = context->init();
    if (initResult.hasError())
    {
        Error error = initResult.error();
        qCritical() << error.className() + "\n" + error.code() + "\n" + error.message() + "\n" + error.data();
    }

    // repositories:
    QSharedPointer<AuthorRepository> authorRepository(new AuthorRepository(new DatabaseTable<Domain::Author>(context)));

    QSharedPointer<AtelierRepository> atelierRepository(
        new AtelierRepository(new DatabaseTable<Domain::Atelier>(context)));

    QSharedPointer<BookRepository> bookRepository(new BookRepository(new DatabaseTable<Domain::Book>(context)));

    QSharedPointer<ChapterRepository> chapterRepository(
        new ChapterRepository(new DatabaseTable<Domain::Chapter>(context)));

    // register repositories:
    Repository::RepositoryProvider::instance()->registerRepository(RepositoryProvider::Author, authorRepository);
    Repository::RepositoryProvider::instance()->registerRepository(RepositoryProvider::Atelier, atelierRepository);
    Repository::RepositoryProvider::instance()->registerRepository(RepositoryProvider::Book, bookRepository);
    Repository::RepositoryProvider::instance()->registerRepository(RepositoryProvider::Chapter, chapterRepository);
}

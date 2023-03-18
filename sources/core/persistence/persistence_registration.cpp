#include "persistence_registration.h"
#include "database/database_context.h"
#include "database/internal_database.h"
#include "repositories/atelier_repository.h"
#include "repositories/author_repository.h"
#include "repositories/book_repository.h"
#include "repositories/repository_provider.h"

using namespace Repository;
using namespace Database;
using namespace Persistence;

PersistenceRegistration::PersistenceRegistration(QObject *parent) : QObject{parent}
{

    auto *context = new DatabaseContext();
    context->init();

    // repositories:
    QSharedPointer<AuthorRepository> authorRepository(
        new AuthorRepository(new InternalDatabase<Domain::Author>(context)));

    QSharedPointer<AtelierRepository> atelierRepository(
        new AtelierRepository(new InternalDatabase<Domain::Atelier>(context)));

    QSharedPointer<BookRepository> bookRepository(new BookRepository(new InternalDatabase<Domain::Book>(context)));

    // register repositories:
    Repository::RepositoryProvider::instance()->registerRepository(RepositoryProvider::Author, authorRepository);
    Repository::RepositoryProvider::instance()->registerRepository(RepositoryProvider::Atelier, atelierRepository);
    Repository::RepositoryProvider::instance()->registerRepository(RepositoryProvider::Book, bookRepository);
}

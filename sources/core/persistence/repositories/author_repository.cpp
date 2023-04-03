#include "author_repository.h"

using namespace Repository;

AuthorRepository::AuthorRepository(InterfaceDatabaseTable<Domain::Author> *database)
    : QObject(nullptr), Repository::GenericRepository<Domain::Author>(database)
{
}

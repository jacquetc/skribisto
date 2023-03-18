#include "author_repository.h"

using namespace Repository;

AuthorRepository::AuthorRepository(InterfaceDatabase<Domain::Author> *database)
    : QObject(nullptr), Repository::GenericRepository<Domain::Author>(database)
{
}

#include "author_repository.h"

using namespace Repository;

AuthorRepository::AuthorRepository(InterfaceDatabaseTable<Domain::Author> *authorDatabase)
    : QObject(nullptr), Repository::GenericRepository<Domain::Author>(authorDatabase)
{
}





#include "book_repository.h"

using namespace Repository;

BookRepository::BookRepository(InterfaceDatabaseTable<Domain::Book> *database)
    : QObject(nullptr), Repository::GenericRepository<Domain::Book>(database)
{
}

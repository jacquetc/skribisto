#include "book_repository.h"

using namespace Repository;

BookRepository::BookRepository(InterfaceDatabase<Domain::Book> *database)
    : QObject(nullptr), Repository::GenericRepository<Domain::Book>(database)
{
}

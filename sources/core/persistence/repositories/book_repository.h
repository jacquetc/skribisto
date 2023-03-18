#pragma once

#include "book.h"
#include "database/interface_database.h"
#include "generic_repository.h"
#include "persistence/interface_book_repository.h"
#include "persistence_global.h"
#include <QObject>

namespace Repository
{
class SKR_PERSISTENCE_EXPORT BookRepository : public QObject,
                                              public Repository::GenericRepository<Domain::Book>,
                                              public Contracts::Persistence::InterfaceBookRepository
{
    Q_OBJECT
    Q_INTERFACES(Contracts::Persistence::InterfaceBookRepository)
  public:
    explicit BookRepository(InterfaceDatabase<Domain::Book> *database);
};

} // namespace Repository

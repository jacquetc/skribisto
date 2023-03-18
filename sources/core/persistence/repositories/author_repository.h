#pragma once

#include "author.h"
#include "database/interface_database.h"
#include "generic_repository.h"
#include "persistence/interface_author_repository.h"
#include "persistence_global.h"
#include <QObject>

namespace Repository
{
class SKR_PERSISTENCE_EXPORT AuthorRepository : public QObject,
                                              public Repository::GenericRepository<Domain::Author>,
                                              public Contracts::Persistence::InterfaceAuthorRepository
{
    Q_OBJECT
    Q_INTERFACES(Contracts::Persistence::InterfaceAuthorRepository)
  public:
    explicit AuthorRepository(InterfaceDatabase<Domain::Author> *database);
};

} // namespace Repository

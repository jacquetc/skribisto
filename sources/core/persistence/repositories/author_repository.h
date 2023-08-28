#pragma once

#include "author.h"

#include "database/interface_database_table.h"
#include "generic_repository.h"
#include "persistence/interface_author_repository.h"
#include "persistence_global.h"

using namespace Contracts::Persistence;

namespace Repository
{

class SKR_PERSISTENCE_EXPORT AuthorRepository : public Repository::GenericRepository<Domain::Author>,
                                                public Contracts::Persistence::InterfaceAuthorRepository
{
  public:
    explicit AuthorRepository(Domain::EntitySchema *entitySchema,
                              InterfaceDatabaseTable<Domain::Author> *authorDatabase);

    QHash<int, QList<int>> removeInCascade(QList<int> ids) override;

  private:
    Domain::EntitySchema *m_entitySchema;
};

} // namespace Repository
#pragma once

#include "atelier.h"

#include "persistence/interface_book_repository.h"

#include "database/interface_database_table.h"
#include "generic_repository.h"
#include "persistence/interface_atelier_repository.h"
#include "persistence_global.h"

using namespace Contracts::Persistence;

namespace Repository
{

class SKR_PERSISTENCE_EXPORT AtelierRepository : public Repository::GenericRepository<Domain::Atelier>,
                                                 public Contracts::Persistence::InterfaceAtelierRepository
{
  public:
    explicit AtelierRepository(Domain::EntitySchema *entitySchema,
                               InterfaceDatabaseTable<Domain::Atelier> *atelierDatabase,
                               InterfaceBookRepository *bookRepository);

    Domain::Atelier::BooksLoader fetchBooksLoader() override;

    QHash<int, QList<int>> removeInCascade(QList<int> ids) override;

  private:
    Domain::EntitySchema *m_entitySchema;

    InterfaceBookRepository *m_bookRepository;
};

} // namespace Repository
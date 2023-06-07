#pragma once

#include "atelier.h"
#include "database/interface_database_table.h"
#include "generic_repository.h"
#include "persistence/interface_atelier_repository.h"
#include "persistence_global.h"
#include <QObject>

namespace Repository
{
class SKR_PERSISTENCE_EXPORT AtelierRepository : public QObject,
                                          public Repository::GenericRepository<Domain::Atelier>,
                                          public Contracts::Persistence::InterfaceAtelierRepository
{
    Q_OBJECT
    Q_INTERFACES(Contracts::Persistence::InterfaceAtelierRepository)
  public:
    explicit AtelierRepository(InterfaceDatabaseTable<Domain::Atelier> *atelierDatabase, InterfaceDatabaseTable<Domain::Book> *bookDatabase);

    
    Domain::Atelier::BooksLoader fetchBooksLoader();
    

   private:
    InterfaceDatabaseTable<Domain::Atelier> *m_atelierDatabase;
    
    InterfaceDatabaseTable<Domain::Book> *m_bookDatabase;
    
};

} // namespace Repository
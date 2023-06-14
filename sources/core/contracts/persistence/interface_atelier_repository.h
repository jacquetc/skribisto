#pragma once

#include "atelier.h"
#include "contracts_global.h"
#include "interface_generic_repository.h"
#include "interface_repository.h"

namespace Contracts::Persistence {
class SKR_CONTRACTS_EXPORT InterfaceAtelierRepository
    : public virtual Contracts::Persistence::InterfaceGenericRepository<Domain::Atelier>,
      public InterfaceRepository {
public:

    virtual ~InterfaceAtelierRepository()
    {}

    virtual Domain::Atelier::BooksLoader fetchBooksLoader() = 0;
};
} // namespace Contracts::Persistence
#define InterfaceAtelierRepository_iid "eu.skribisto.Contracts.Persistence.InterfaceAtelierRepository"
Q_DECLARE_INTERFACE(Contracts::Persistence::InterfaceAtelierRepository, InterfaceAtelierRepository_iid)

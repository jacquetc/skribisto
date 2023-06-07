#pragma once

#include "book.h"
#include "contracts_global.h" 
#include "interface_generic_repository.h"
#include "interface_repository.h"

namespace Contracts::Persistence
{
class SKR_CONTRACTS_EXPORT InterfaceBookRepository
    : public virtual Contracts::Persistence::InterfaceGenericRepository<Domain::Book>,
      public InterfaceRepository
{
  public:
    virtual ~InterfaceBookRepository()
    {
    }
};
} // namespace Contracts::Persistence
#define InterfaceBookRepository_iid "eu.skribisto.Contracts.Persistence.InterfaceBookRepository"
Q_DECLARE_INTERFACE(Contracts::Persistence::InterfaceBookRepository, InterfaceBookRepository_iid)
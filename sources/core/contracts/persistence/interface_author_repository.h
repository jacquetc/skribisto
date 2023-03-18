#pragma once

#include "author.h"
#include "contracts_global.h"
#include "interface_generic_repository.h"
#include "interface_repository.h"

namespace Contracts::Persistence
{
class SKR_CONTRACTS_EXPORT InterfaceAuthorRepository
    : public virtual Contracts::Persistence::InterfaceGenericRepository<Domain::Author>,
      public InterfaceRepository
{
  public:
    virtual ~InterfaceAuthorRepository()
    {
    }
};
} // namespace Contracts::Persistence
#define InterfaceAuthorRepository_iid "eu.skribisto.Contracts.Persistence.InterfaceAuthorRepository"
Q_DECLARE_INTERFACE(Contracts::Persistence::InterfaceAuthorRepository, InterfaceAuthorRepository_iid)

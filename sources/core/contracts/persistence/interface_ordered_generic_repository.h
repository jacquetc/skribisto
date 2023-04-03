#pragma once

#include "contracts_global.h"
#include "interface_generic_repository.h"
#include "interface_repository.h"

namespace Contracts::Persistence
{
template <class T>
class SKR_CONTRACTS_EXPORT InterfaceOrderedGenericRepository
    : public virtual Contracts::Persistence::InterfaceGenericRepository<T>,
      public InterfaceRepository
{
  public:
    virtual ~InterfaceOrderedGenericRepository()
    {
    }
};
} // namespace Contracts::Persistence

#pragma once

#include "contracts_global.h"
#include "interface_generic_repository.h"
#include "interface_repository.h"

namespace Contracts::Persistence
{
template <class T>
class SKR_CONTRACTS_EXPORT InterfaceUniqueGenericRepository
    : public virtual Contracts::Persistence::InterfaceGenericRepository<T>,
      public InterfaceRepository
{
  public:
    virtual ~InterfaceUniqueGenericRepository()
    {
    }
};
} // namespace Contracts::Persistence

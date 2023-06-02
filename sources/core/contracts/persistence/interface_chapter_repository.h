#pragma once

#include "chapter.h"
#include "contracts_global.h"
#include "interface_generic_repository.h"
#include "interface_repository.h"

namespace Contracts::Persistence
{
class SKR_CONTRACTS_EXPORT InterfaceChapterRepository
    : public virtual Contracts::Persistence::InterfaceGenericRepository<Domain::Chapter>,
      public InterfaceRepository
{
  public:
    virtual ~InterfaceChapterRepository()
    {
    }
};
} // namespace Contracts::Persistence
#define InterfaceChapterRepository_iid "eu.skribisto.Contracts.Persistence.InterfaceChapterRepository"
Q_DECLARE_INTERFACE(Contracts::Persistence::InterfaceChapterRepository, InterfaceChapterRepository_iid)

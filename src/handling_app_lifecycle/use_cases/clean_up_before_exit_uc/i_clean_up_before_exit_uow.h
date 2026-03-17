// Adapted for CleanUpBeforeExit use case

#pragma once

#include "entities/root.h"
#include "unit_of_work/unit_of_work.h"

namespace Skribisto::HandlingAppLifecycle
{
namespace SCE = Common::Entities;

class ICleanUpBeforeExitUnitOfWork : public virtual Common::UnitOfWork::ITransactional
{
  public:
    ~ICleanUpBeforeExitUnitOfWork() override = default;

    DECLARE_UOW_ENTITY_GET_ALL(Root);
    DECLARE_UOW_ENTITY_REMOVE(Root);

    virtual void publishCleanUpBeforeExitSignal() = 0;
};
} // namespace Skribisto::HandlingAppLifecycle

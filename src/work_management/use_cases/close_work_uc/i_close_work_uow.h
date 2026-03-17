// Adapted for CloseWork use case

#pragma once

#include "direct_access/root/i_root_repository.h"
#include "direct_access/system/i_system_repository.h"
#include "entities/root.h"
#include "entities/trash_info.h"
#include "entities/work.h"
#include "entities/work_info.h"
#include "unit_of_work/unit_of_work.h"

namespace Skribisto::WorkManagement
{
namespace SCE = Common::Entities;
namespace SCDRoot = Common::DirectAccess::Root;
namespace SCDSystem = Common::DirectAccess::System;

class ICloseWorkUnitOfWork : public virtual Common::UnitOfWork::ITransactional
{
  public:
    ~ICloseWorkUnitOfWork() override = default;

    DECLARE_UOW_ENTITY_GET_ALL(Root);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(Root, SCDRoot::RootRelationshipField);

    DECLARE_UOW_ENTITY_RELATIONSHIPS(System, SCDSystem::SystemRelationshipField);

    DECLARE_UOW_ENTITY_REMOVE(Work);
    DECLARE_UOW_ENTITY_REMOVE(TrashInfo);

    DECLARE_UOW_ENTITY_GET(WorkInfo);
    DECLARE_UOW_ENTITY_UPDATE(WorkInfo);

    virtual void publishCloseWorkSignal() = 0;
};
} // namespace Skribisto::WorkManagement

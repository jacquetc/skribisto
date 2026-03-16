// Adapted for TrashBinder use case

#pragma once

#include "direct_access/binder/i_binder_repository.h"
#include "direct_access/binder_item/i_binder_item_repository.h"
#include "direct_access/system/i_system_repository.h"
#include "entities/binder.h"
#include "entities/binder_item.h"
#include "entities/content.h"
#include "entities/system.h"
#include "entities/trash_info.h"
#include "unit_of_work/unit_of_work.h"

namespace Skribisto::TrashManagement
{
namespace SCE = Common::Entities;
namespace SCDSystem = Common::DirectAccess::System;
namespace SCDBinder = Common::DirectAccess::Binder;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

class ITrashBinderUnitOfWork : public virtual Common::UnitOfWork::ITransactional
{
  public:
    ~ITrashBinderUnitOfWork() override = default;

    DECLARE_UOW_ENTITY_GET(System);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(System, SCDSystem::SystemRelationshipField);
    DECLARE_UOW_ENTITY_CREATE_ORPHANS(TrashInfo);
    DECLARE_UOW_ENTITY_GET(Binder);
    DECLARE_UOW_ENTITY_UPDATE(Binder);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(Binder, SCDBinder::BinderRelationshipField);
    DECLARE_UOW_ENTITY_GET(BinderItem);
    DECLARE_UOW_ENTITY_UPDATE(BinderItem);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(BinderItem, SCDBinderItem::BinderItemRelationshipField);
    DECLARE_UOW_ENTITY_GET(Content);
    DECLARE_UOW_ENTITY_UPDATE(Content);

    virtual void publishTrashBinderSignal() = 0;
};
} // namespace Skribisto::TrashManagement

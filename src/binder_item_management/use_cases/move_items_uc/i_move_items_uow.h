// Adapted for MoveItems use case

#pragma once

#include "database/snapshot_types.h"
#include "direct_access/binder/i_binder_repository.h"
#include "direct_access/binder_item/i_binder_item_repository.h"
#include "entities/binder.h"
#include "entities/binder_item.h"
#include "unit_of_work/unit_of_work.h"

namespace Skribisto::BinderItemManagement
{
namespace SCE = Common::Entities;
namespace SCDatabase = Common::Database;
namespace SCDBinder = Common::DirectAccess::Binder;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

class IMoveItemsUnitOfWork : public virtual Common::UnitOfWork::ITransactional
{
  public:
    ~IMoveItemsUnitOfWork() override = default;

    DECLARE_UOW_ENTITY_GET_ALL(Binder);
    DECLARE_UOW_ENTITY_SNAPSHOT(Binder);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(Binder, SCDBinder::BinderRelationshipField);

    DECLARE_UOW_ENTITY_GET(BinderItem);
    DECLARE_UOW_ENTITY_UPDATE(BinderItem);

    virtual void publishMoveItemsSignal() = 0;
};
} // namespace Skribisto::BinderItemManagement

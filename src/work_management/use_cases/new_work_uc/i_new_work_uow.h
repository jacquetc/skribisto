// Adapted for NewWork use case

#pragma once

#include "direct_access/binder/i_binder_repository.h"
#include "direct_access/binder_item/i_binder_item_repository.h"
#include "direct_access/root/i_root_repository.h"
#include "direct_access/system/i_system_repository.h"
#include "direct_access/work/i_work_repository.h"
#include "entities/binder.h"
#include "entities/binder_item.h"
#include "entities/content.h"
#include "entities/root.h"
#include "entities/system.h"
#include "entities/work.h"
#include "entities/work_info.h"
#include "unit_of_work/unit_of_work.h"

namespace Skribisto::WorkManagement
{
namespace SCE = Common::Entities;
namespace SCDRoot = Common::DirectAccess::Root;
namespace SCDSystem = Common::DirectAccess::System;
namespace SCDWork = Common::DirectAccess::Work;
namespace SCDBinder = Common::DirectAccess::Binder;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

class INewWorkUnitOfWork : public virtual Common::UnitOfWork::ITransactional
{
  public:
    ~INewWorkUnitOfWork() override = default;

    DECLARE_UOW_ENTITY_CREATE_ORPHANS(Root);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(Root, SCDRoot::RootRelationshipField);

    DECLARE_UOW_ENTITY_CREATE_ORPHANS(System);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(System, SCDSystem::SystemRelationshipField);

    DECLARE_UOW_ENTITY_CREATE_ORPHANS(WorkInfo);

    DECLARE_UOW_ENTITY_CREATE_ORPHANS(Work);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(Work, SCDWork::WorkRelationshipField);

    DECLARE_UOW_ENTITY_CREATE_ORPHANS(Binder);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(Binder, SCDBinder::BinderRelationshipField);

    DECLARE_UOW_ENTITY_CREATE_ORPHANS(BinderItem);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(BinderItem, SCDBinderItem::BinderItemRelationshipField);

    DECLARE_UOW_ENTITY_CREATE_ORPHANS(Content);

    virtual void publishNewWorkSignal() = 0;
};
} // namespace Skribisto::WorkManagement

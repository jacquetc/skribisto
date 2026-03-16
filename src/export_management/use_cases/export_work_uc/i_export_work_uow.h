// Adapted for ExportWork use case

#pragma once

#include "direct_access/binder/i_binder_repository.h"
#include "direct_access/binder_item/i_binder_item_repository.h"
#include "direct_access/work/i_work_repository.h"
#include "entities/binder.h"
#include "entities/binder_item.h"
#include "entities/binder_tag.h"
#include "entities/content.h"
#include "entities/work.h"
#include "unit_of_work/unit_of_work.h"

namespace Skribisto::ExportManagement
{
namespace SCE = Common::Entities;
namespace SCDWork = Common::DirectAccess::Work;
namespace SCDBinder = Common::DirectAccess::Binder;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

class IExportWorkUnitOfWork : public virtual Common::UnitOfWork::ITransactional
{
  public:
    ~IExportWorkUnitOfWork() override = default;

    DECLARE_UOW_ENTITY_GET(Work);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(Work, SCDWork::WorkRelationshipField);
    DECLARE_UOW_ENTITY_GET(Binder);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(Binder, SCDBinder::BinderRelationshipField);
    DECLARE_UOW_ENTITY_GET(BinderItem);
    DECLARE_UOW_ENTITY_RELATIONSHIPS(BinderItem, SCDBinderItem::BinderItemRelationshipField);
    DECLARE_UOW_ENTITY_GET(BinderTag);
    DECLARE_UOW_ENTITY_GET(Content);

    virtual void publishExportWorkSignal() = 0;
};
} // namespace Skribisto::ExportManagement

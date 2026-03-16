// Adapted for RestoreItems use case

#pragma once

#include "direct_access/event_registry.h"
#include "direct_access/repository_factory.h"
#include "features/feature_event_registry.h"
#include "unit_of_work/unit_of_work.h"
#include "use_cases/restore_items_uc/i_restore_items_uow.h"

namespace Skribisto::TrashManagement
{
namespace SCDatabase = Common::Database;
namespace SCD = Common::DirectAccess;
namespace SCF = Common::Features;
namespace SCDSystem = Common::DirectAccess::System;
namespace SCDBinder = Common::DirectAccess::Binder;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

class RestoreItemsUnitOfWork : public Common::UnitOfWork::UnitOfWorkBase, public IRestoreItemsUnitOfWork
{
  public:
    RestoreItemsUnitOfWork(SCDatabase::DbContext &db, QPointer<SCD::EventRegistry> eventRegistry,
                           QPointer<SCF::FeatureEventRegistry> featureEventRegistry)
        : UnitOfWorkBase(db, eventRegistry), m_featureEventRegistry(featureEventRegistry)
    {
    }

    UOW_ENTITY_GET(System);
    UOW_ENTITY_RELATIONSHIPS(System, SCDSystem::SystemRelationshipField);
    UOW_ENTITY_GET(TrashInfo);
    UOW_ENTITY_REMOVE(TrashInfo);
    UOW_ENTITY_GET(Binder);
    UOW_ENTITY_UPDATE(Binder);
    UOW_ENTITY_RELATIONSHIPS(Binder, SCDBinder::BinderRelationshipField);
    UOW_ENTITY_GET(BinderItem);
    UOW_ENTITY_UPDATE(BinderItem);
    UOW_ENTITY_RELATIONSHIPS(BinderItem, SCDBinderItem::BinderItemRelationshipField);
    UOW_ENTITY_GET(Content);
    UOW_ENTITY_UPDATE(Content);

    void publishRestoreItemsSignal() override;

  private:
    QPointer<SCF::FeatureEventRegistry> m_featureEventRegistry;
};

inline void RestoreItemsUnitOfWork::publishRestoreItemsSignal()
{
    m_featureEventRegistry->trashManagementEvents()->publishRestoreItemsSignal();
}

} // namespace Skribisto::TrashManagement

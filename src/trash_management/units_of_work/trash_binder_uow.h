// Adapted for TrashBinder use case

#pragma once

#include "direct_access/event_registry.h"
#include "direct_access/repository_factory.h"
#include "features/feature_event_registry.h"
#include "unit_of_work/unit_of_work.h"
#include "use_cases/trash_binder_uc/i_trash_binder_uow.h"

namespace Skribisto::TrashManagement
{
namespace SCDatabase = Common::Database;
namespace SCD = Common::DirectAccess;
namespace SCF = Common::Features;
namespace SCDSystem = Common::DirectAccess::System;
namespace SCDBinder = Common::DirectAccess::Binder;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

class TrashBinderUnitOfWork : public Common::UnitOfWork::UnitOfWorkBase, public ITrashBinderUnitOfWork
{
  public:
    TrashBinderUnitOfWork(SCDatabase::DbContext &db, QPointer<SCD::EventRegistry> eventRegistry,
                          QPointer<SCF::FeatureEventRegistry> featureEventRegistry)
        : UnitOfWorkBase(db, eventRegistry), m_featureEventRegistry(featureEventRegistry)
    {
    }

    UOW_ENTITY_GET(System);
    UOW_ENTITY_RELATIONSHIPS(System, SCDSystem::SystemRelationshipField);
    UOW_ENTITY_CREATE_ORPHANS(TrashInfo);
    UOW_ENTITY_GET(Binder);
    UOW_ENTITY_UPDATE(Binder);
    UOW_ENTITY_RELATIONSHIPS(Binder, SCDBinder::BinderRelationshipField);
    UOW_ENTITY_GET(BinderItem);
    UOW_ENTITY_UPDATE(BinderItem);
    UOW_ENTITY_RELATIONSHIPS(BinderItem, SCDBinderItem::BinderItemRelationshipField);
    UOW_ENTITY_GET(Content);
    UOW_ENTITY_UPDATE(Content);

    void publishTrashBinderSignal() override;

  private:
    QPointer<SCF::FeatureEventRegistry> m_featureEventRegistry;
};

inline void TrashBinderUnitOfWork::publishTrashBinderSignal()
{
    m_featureEventRegistry->trashManagementEvents()->publishTrashBinderSignal();
}

} // namespace Skribisto::TrashManagement

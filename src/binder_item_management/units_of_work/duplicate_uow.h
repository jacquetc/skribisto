// Adapted for Duplicate use case

#pragma once

#include "direct_access/event_registry.h"
#include "direct_access/repository_factory.h"
#include "features/feature_event_registry.h"
#include "unit_of_work/unit_of_work.h"
#include "use_cases/duplicate_uc/i_duplicate_uow.h"

namespace Skribisto::BinderItemManagement
{
namespace SCDatabase = Common::Database;
namespace SCD = Common::DirectAccess;
namespace SCF = Common::Features;
namespace SCDBinder = Common::DirectAccess::Binder;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

class DuplicateUnitOfWork : public Common::UnitOfWork::UnitOfWorkBase, public IDuplicateUnitOfWork
{
  public:
    DuplicateUnitOfWork(SCDatabase::DbContext &db, QPointer<SCD::EventRegistry> eventRegistry,
                        QPointer<SCF::FeatureEventRegistry> featureEventRegistry)
        : UnitOfWorkBase(db, eventRegistry), m_featureEventRegistry(featureEventRegistry)
    {
    }

    UOW_ENTITY_GET_ALL(Binder);
    UOW_ENTITY_SNAPSHOT(Binder);
    UOW_ENTITY_RELATIONSHIPS(Binder, SCDBinder::BinderRelationshipField);

    UOW_ENTITY_GET(BinderItem);
    UOW_ENTITY_CREATE(BinderItem);
    UOW_ENTITY_REMOVE(BinderItem);
    UOW_ENTITY_RELATIONSHIPS(BinderItem, SCDBinderItem::BinderItemRelationshipField);

    UOW_ENTITY_GET(Content);
    UOW_ENTITY_CREATE(Content);
    UOW_ENTITY_REMOVE(Content);

    void publishDuplicateSignal() override;

  private:
    QPointer<SCF::FeatureEventRegistry> m_featureEventRegistry;
};

inline void DuplicateUnitOfWork::publishDuplicateSignal()
{
    m_featureEventRegistry->binderItemManagementEvents()->publishDuplicateSignal();
}

} // namespace Skribisto::BinderItemManagement

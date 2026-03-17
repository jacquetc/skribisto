// Adapted for MoveItems use case

#pragma once

#include "direct_access/event_registry.h"
#include "direct_access/repository_factory.h"
#include "features/feature_event_registry.h"
#include "unit_of_work/unit_of_work.h"
#include "use_cases/move_items_uc/i_move_items_uow.h"

namespace Skribisto::BinderItemManagement
{
namespace SCDatabase = Common::Database;
namespace SCD = Common::DirectAccess;
namespace SCF = Common::Features;
namespace SCDBinder = Common::DirectAccess::Binder;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

class MoveItemsUnitOfWork : public Common::UnitOfWork::UnitOfWorkBase, public IMoveItemsUnitOfWork
{
  public:
    MoveItemsUnitOfWork(SCDatabase::DbContext &db, QPointer<SCD::EventRegistry> eventRegistry,
                        QPointer<SCF::FeatureEventRegistry> featureEventRegistry)
        : UnitOfWorkBase(db, eventRegistry), m_featureEventRegistry(featureEventRegistry)
    {
    }

    UOW_ENTITY_GET_ALL(Binder);
    UOW_ENTITY_SNAPSHOT(Binder);
    UOW_ENTITY_RELATIONSHIPS(Binder, SCDBinder::BinderRelationshipField);

    UOW_ENTITY_GET(BinderItem);
    UOW_ENTITY_UPDATE(BinderItem);

    void publishMoveItemsSignal() override;

  private:
    QPointer<SCF::FeatureEventRegistry> m_featureEventRegistry;
};

inline void MoveItemsUnitOfWork::publishMoveItemsSignal()
{
    m_featureEventRegistry->binderItemManagementEvents()->publishMoveItemsSignal();
}

} // namespace Skribisto::BinderItemManagement

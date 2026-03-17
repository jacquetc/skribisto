// Adapted for CloseWork use case

#pragma once

#include "direct_access/event_registry.h"
#include "direct_access/repository_factory.h"
#include "features/feature_event_registry.h"
#include "unit_of_work/unit_of_work.h"
#include "use_cases/close_work_uc/i_close_work_uow.h"

namespace Skribisto::WorkManagement
{
namespace SCE = Common::Entities;
namespace SCD = Common::DirectAccess;
namespace SCF = Common::Features;
namespace SCDRoot = Common::DirectAccess::Root;
namespace SCDSystem = Common::DirectAccess::System;

class CloseWorkUnitOfWork : public Common::UnitOfWork::UnitOfWorkBase, public ICloseWorkUnitOfWork
{
  public:
    CloseWorkUnitOfWork(SCDatabase::DbContext &db, QPointer<SCD::EventRegistry> eventRegistry,
                        QPointer<SCF::FeatureEventRegistry> featureEventRegistry)
        : UnitOfWorkBase(db, eventRegistry), m_featureEventRegistry(featureEventRegistry)
    {
    }

    UOW_ENTITY_GET_ALL(Root);
    UOW_ENTITY_RELATIONSHIPS(Root, SCDRoot::RootRelationshipField);

    UOW_ENTITY_RELATIONSHIPS(System, SCDSystem::SystemRelationshipField);

    UOW_ENTITY_REMOVE(Work);
    UOW_ENTITY_REMOVE(TrashInfo);

    UOW_ENTITY_GET(WorkInfo);
    UOW_ENTITY_UPDATE(WorkInfo);

    void publishCloseWorkSignal() override;

  private:
    QPointer<SCF::FeatureEventRegistry> m_featureEventRegistry;
};

inline void CloseWorkUnitOfWork::publishCloseWorkSignal()
{
    m_featureEventRegistry->workManagementEvents()->publishCloseWorkSignal();
}

} // namespace Skribisto::WorkManagement

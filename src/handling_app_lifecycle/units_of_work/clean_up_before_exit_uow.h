// Adapted for CleanUpBeforeExit use case

#pragma once

#include "direct_access/event_registry.h"
#include "direct_access/repository_factory.h"
#include "features/feature_event_registry.h"
#include "unit_of_work/unit_of_work.h"
#include "use_cases/clean_up_before_exit_uc/i_clean_up_before_exit_uow.h"

namespace Skribisto::HandlingAppLifecycle
{
namespace SCE = Common::Entities;
namespace SCD = Common::DirectAccess;
namespace SCF = Common::Features;

class CleanUpBeforeExitUnitOfWork : public Common::UnitOfWork::UnitOfWorkBase, public ICleanUpBeforeExitUnitOfWork
{
  public:
    CleanUpBeforeExitUnitOfWork(SCDatabase::DbContext &db, QPointer<SCD::EventRegistry> eventRegistry,
                                QPointer<SCF::FeatureEventRegistry> featureEventRegistry)
        : UnitOfWorkBase(db, eventRegistry), m_featureEventRegistry(featureEventRegistry)
    {
    }

    UOW_ENTITY_GET_ALL(Root);
    UOW_ENTITY_REMOVE(Root);

    void publishCleanUpBeforeExitSignal() override;

  private:
    QPointer<SCF::FeatureEventRegistry> m_featureEventRegistry;
};

inline void CleanUpBeforeExitUnitOfWork::publishCleanUpBeforeExitSignal()
{
    m_featureEventRegistry->handlingAppLifecycleEvents()->publishCleanUpBeforeExitSignal();
}

} // namespace Skribisto::HandlingAppLifecycle

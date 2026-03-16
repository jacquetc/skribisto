// Adapted for ExportWork use case

#pragma once

#include "direct_access/event_registry.h"
#include "direct_access/repository_factory.h"
#include "features/feature_event_registry.h"
#include "unit_of_work/unit_of_work.h"
#include "use_cases/export_work_uc/i_export_work_uow.h"

namespace Skribisto::ExportManagement
{
namespace SCDatabase = Common::Database;
namespace SCD = Common::DirectAccess;
namespace SCF = Common::Features;
namespace SCDWork = Common::DirectAccess::Work;
namespace SCDBinder = Common::DirectAccess::Binder;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

class ExportWorkUnitOfWork : public Common::UnitOfWork::UnitOfWorkBase, public IExportWorkUnitOfWork
{
  public:
    ExportWorkUnitOfWork(SCDatabase::DbContext &db, QPointer<SCD::EventRegistry> eventRegistry,
                         QPointer<SCF::FeatureEventRegistry> featureEventRegistry)
        : UnitOfWorkBase(db, eventRegistry), m_featureEventRegistry(featureEventRegistry)
    {
    }

    UOW_ENTITY_GET(Work);
    UOW_ENTITY_RELATIONSHIPS(Work, SCDWork::WorkRelationshipField);
    UOW_ENTITY_GET(Binder);
    UOW_ENTITY_RELATIONSHIPS(Binder, SCDBinder::BinderRelationshipField);
    UOW_ENTITY_GET(BinderItem);
    UOW_ENTITY_RELATIONSHIPS(BinderItem, SCDBinderItem::BinderItemRelationshipField);
    UOW_ENTITY_GET(BinderTag);
    UOW_ENTITY_GET(Content);

    void publishExportWorkSignal() override;

  private:
    QPointer<SCF::FeatureEventRegistry> m_featureEventRegistry;
};

inline void ExportWorkUnitOfWork::publishExportWorkSignal()
{
    m_featureEventRegistry->exportManagementEvents()->publishExportWorkSignal();
}

} // namespace Skribisto::ExportManagement

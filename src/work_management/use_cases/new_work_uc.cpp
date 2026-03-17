// Adapted for NewWork use case

#include "new_work_uc.h"

namespace Skribisto::WorkManagement
{
namespace SCE = Common::Entities;
namespace SCDRoot = Common::DirectAccess::Root;
namespace SCDSystem = Common::DirectAccess::System;
namespace SCDWork = Common::DirectAccess::Work;
namespace SCDBinder = Common::DirectAccess::Binder;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

NewWorkUseCase::NewWorkUseCase(std::unique_ptr<INewWorkUnitOfWork> uow) : m_uow(std::move(uow))
{
}

bool NewWorkUseCase::execute(const NewWorkDto &newWorkDto) const
{
    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        const auto now = QDateTime::currentDateTimeUtc();

        // Create Root
        SCE::Root root;
        auto roots = m_uow->createOrphanRoot({root});
        if (roots.isEmpty())
            throw std::runtime_error("Failed to create Root");
        int rootId = roots.first().id;

        // Create System
        SCE::System system;
        auto systems = m_uow->createOrphanSystem({system});
        if (systems.isEmpty())
            throw std::runtime_error("Failed to create System");
        int systemId = systems.first().id;
        m_uow->setRootRelationship(rootId, SCDRoot::RootRelationshipField::System, {systemId});

        // Create WorkInfo with fileName
        SCE::WorkInfo workInfo;
        workInfo.fileName = newWorkDto.fileName;
        workInfo.createdAt = now;
        workInfo.updatedAt = now;
        auto workInfos = m_uow->createOrphanWorkInfo({workInfo});
        if (!workInfos.isEmpty())
            m_uow->setSystemRelationship(systemId, SCDSystem::SystemRelationshipField::WorkInfo,
                                         {workInfos.first().id});

        // Create default Work
        SCE::Work work;
        work.createdAt = now;
        work.updatedAt = now;
        auto works = m_uow->createOrphanWork({work});
        if (works.isEmpty())
            throw std::runtime_error("Failed to create Work");
        int workId = works.first().id;
        m_uow->setRootRelationship(rootId, SCDRoot::RootRelationshipField::Works, {workId});

        // Create default Binder
        SCE::Binder binder;
        binder.createdAt = now;
        binder.updatedAt = now;
        binder.activated = true;
        auto binders = m_uow->createOrphanBinder({binder});
        if (binders.isEmpty())
            throw std::runtime_error("Failed to create Binder");
        int binderId = binders.first().id;
        m_uow->setWorkRelationship(workId, SCDWork::WorkRelationshipField::Binders, {binderId});

        // Create default BinderItem
        SCE::BinderItem binderItem;
        binderItem.createdAt = now;
        binderItem.updatedAt = now;
        binderItem.activated = true;
        binderItem.isPrintable = true;
        auto binderItems = m_uow->createOrphanBinderItem({binderItem});
        if (binderItems.isEmpty())
            throw std::runtime_error("Failed to create BinderItem");
        int binderItemId = binderItems.first().id;
        m_uow->setBinderRelationship(binderId, SCDBinder::BinderRelationshipField::BinderItems, {binderItemId});

        // Create default Content
        SCE::Content content;
        content.createdAt = now;
        content.updatedAt = now;
        content.activated = true;
        auto contents = m_uow->createOrphanContent({content});
        if (contents.isEmpty())
            throw std::runtime_error("Failed to create Content");
        m_uow->setBinderItemRelationship(binderItemId, SCDBinderItem::BinderItemRelationshipField::Contents,
                                         {contents.first().id});

        if (!m_uow->commit())
            throw std::runtime_error("Failed to commit transaction");
    }
    catch (...)
    {
        m_uow->rollback();
        throw;
    }

    m_uow->publishNewWorkSignal();
    return true;
}

} // namespace Skribisto::WorkManagement

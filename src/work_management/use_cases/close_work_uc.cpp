// Adapted for CloseWork use case

#include "close_work_uc.h"

namespace Skribisto::WorkManagement
{
namespace SCDRoot = Common::DirectAccess::Root;
namespace SCDSystem = Common::DirectAccess::System;

CloseWorkUseCase::CloseWorkUseCase(std::unique_ptr<ICloseWorkUnitOfWork> uow) : m_uow(std::move(uow))
{
}

bool CloseWorkUseCase::execute() const
{
    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        // Get Root to access Works and System
        auto roots = m_uow->getAllRoot();
        if (roots.isEmpty())
            throw std::runtime_error("Root entity not found");

        int rootId = roots.first().id;

        // Remove all Works (cascade deletes Binders, BinderItems, Content, Tags, DictWords)
        auto workIds = m_uow->getRootRelationship(rootId, SCDRoot::RootRelationshipField::Works);
        if (!workIds.isEmpty())
        {
            m_uow->removeWork(workIds);
            m_uow->setRootRelationship(rootId, SCDRoot::RootRelationshipField::Works, {});
        }

        // Get System to clear TrashInfos and WorkInfo
        auto systemIds = m_uow->getRootRelationship(rootId, SCDRoot::RootRelationshipField::System);
        if (!systemIds.isEmpty())
        {
            int systemId = systemIds.first();

            // Remove all TrashInfos
            auto trashIds =
                m_uow->getSystemRelationship(systemId, SCDSystem::SystemRelationshipField::TrashInfos);
            if (!trashIds.isEmpty())
            {
                m_uow->removeTrashInfo(trashIds);
                m_uow->setSystemRelationship(systemId, SCDSystem::SystemRelationshipField::TrashInfos, {});
            }

            // Clear WorkInfo fileName
            auto workInfoIds =
                m_uow->getSystemRelationship(systemId, SCDSystem::SystemRelationshipField::WorkInfo);
            if (!workInfoIds.isEmpty())
            {
                auto workInfos = m_uow->getWorkInfo(workInfoIds);
                if (!workInfos.isEmpty())
                {
                    auto workInfo = workInfos.first();
                    workInfo.fileName = std::nullopt;
                    workInfo.updatedAt = QDateTime::currentDateTimeUtc();
                    m_uow->updateWorkInfo({workInfo});
                }
            }
        }

        if (!m_uow->commit())
            throw std::runtime_error("Failed to commit transaction");
    }
    catch (...)
    {
        m_uow->rollback();
        throw;
    }

    m_uow->publishCloseWorkSignal();
    return true;
}

} // namespace Skribisto::WorkManagement

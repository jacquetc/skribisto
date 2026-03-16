// Adapted for TrashBinderItems use case

#include "trash_binder_items_uc.h"

namespace Skribisto::TrashManagement
{

TrashBinderItemsUseCase::TrashBinderItemsUseCase(std::unique_ptr<ITrashBinderItemsUnitOfWork> uow)
    : m_uow(std::move(uow))
{
}

bool TrashBinderItemsUseCase::execute(const TrashBinderItemsDto &dto) const
{
    if (dto.binderItemIds.isEmpty())
        return true;

    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        const auto now = QDateTime::currentDateTimeUtc();

        // Get all the system entity (there's only one)
        auto systems = m_uow->getSystem({1});
        if (systems.isEmpty())
            throw std::runtime_error("System entity not found");

        for (int itemId : dto.binderItemIds)
        {
            // Deactivate the BinderItem
            auto items = m_uow->getBinderItem({itemId});
            if (items.isEmpty())
                continue;

            auto item = items.first();
            item.activated = false;
            m_uow->updateBinderItem({item});

            // Deactivate its Contents
            auto contentIds =
                m_uow->getBinderItemRelationship(itemId, SCDBinderItem::BinderItemRelationshipField::Contents);
            if (!contentIds.isEmpty())
            {
                auto contents = m_uow->getContent(contentIds);
                for (auto &c : contents)
                    c.activated = false;
                m_uow->updateContent(contents);
            }

            // Create TrashInfo record
            SCE::TrashInfo trashInfo;
            trashInfo.trashedAt = now;
            trashInfo.originBinderId = dto.originBinderId;
            trashInfo.trashedBinderItem = itemId;
            auto trashInfos = m_uow->createOrphanTrashInfo({trashInfo});

            // Add TrashInfo to System's trashInfos relationship
            if (!trashInfos.isEmpty())
            {
                auto existingTrashInfoIds =
                    m_uow->getSystemRelationship(1, SCDSystem::SystemRelationshipField::TrashInfos);
                existingTrashInfoIds.append(trashInfos.first().id);
                m_uow->setSystemRelationship(1, SCDSystem::SystemRelationshipField::TrashInfos,
                                             existingTrashInfoIds);
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

    m_uow->publishTrashBinderItemsSignal();
    return true;
}

} // namespace Skribisto::TrashManagement

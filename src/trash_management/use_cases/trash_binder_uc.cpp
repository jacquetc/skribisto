// Adapted for TrashBinder use case

#include "trash_binder_uc.h"

namespace Skribisto::TrashManagement
{

TrashBinderUseCase::TrashBinderUseCase(std::unique_ptr<ITrashBinderUnitOfWork> uow) : m_uow(std::move(uow))
{
}

bool TrashBinderUseCase::execute(const TrashBinderDto &dto) const
{
    if (dto.binderId <= 0)
        throw std::runtime_error("Invalid binder ID");

    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        const auto now = QDateTime::currentDateTimeUtc();

        // Deactivate the Binder
        auto binders = m_uow->getBinder({dto.binderId});
        if (binders.isEmpty())
            throw std::runtime_error("Binder not found");

        auto binder = binders.first();
        binder.activated = false;
        m_uow->updateBinder({binder});

        // Cascade: deactivate all BinderItems in this Binder
        auto itemIds =
            m_uow->getBinderRelationship(dto.binderId, SCDBinder::BinderRelationshipField::BinderItems);
        if (!itemIds.isEmpty())
        {
            auto items = m_uow->getBinderItem(itemIds);
            for (auto &item : items)
                item.activated = false;
            m_uow->updateBinderItem(items);

            // Cascade: deactivate all Contents of those BinderItems
            for (int itemId : itemIds)
            {
                auto contentIds =
                    m_uow->getBinderItemRelationship(itemId, SCDBinderItem::BinderItemRelationshipField::Contents);
                if (!contentIds.isEmpty())
                {
                    auto contents = m_uow->getContent(contentIds);
                    for (auto &c : contents)
                        c.activated = false;
                    m_uow->updateContent(contents);
                }
            }
        }

        // Create TrashInfo record for the Binder
        SCE::TrashInfo trashInfo;
        trashInfo.trashedAt = now;
        trashInfo.originBinderId = 0; // Binder has no parent binder
        trashInfo.trashedBinder = dto.binderId;
        auto trashInfos = m_uow->createOrphanTrashInfo({trashInfo});

        // Add TrashInfo to System's trashInfos relationship
        if (!trashInfos.isEmpty())
        {
            auto existingTrashInfoIds =
                m_uow->getSystemRelationship(1, SCDSystem::SystemRelationshipField::TrashInfos);
            existingTrashInfoIds.append(trashInfos.first().id);
            m_uow->setSystemRelationship(1, SCDSystem::SystemRelationshipField::TrashInfos, existingTrashInfoIds);
        }

        if (!m_uow->commit())
            throw std::runtime_error("Failed to commit transaction");
    }
    catch (...)
    {
        m_uow->rollback();
        throw;
    }

    m_uow->publishTrashBinderSignal();
    return true;
}

} // namespace Skribisto::TrashManagement

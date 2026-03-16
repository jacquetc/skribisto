// Adapted for RestoreItems use case

#include "restore_items_uc.h"

namespace Skribisto::TrashManagement
{

RestoreItemsUseCase::RestoreItemsUseCase(std::unique_ptr<IRestoreItemsUnitOfWork> uow) : m_uow(std::move(uow))
{
}

RestoreResultDto RestoreItemsUseCase::execute(const RestoreItemsDto &dto) const
{
    RestoreResultDto result;

    if (dto.trashInfoIds.isEmpty())
        return result;

    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        auto trashInfos = m_uow->getTrashInfo(dto.trashInfoIds);

        for (const auto &ti : trashInfos)
        {
            if (ti.trashedBinderItem.has_value())
            {
                // Restore a BinderItem
                int itemId = ti.trashedBinderItem.value();
                auto items = m_uow->getBinderItem({itemId});
                if (!items.isEmpty())
                {
                    auto item = items.first();
                    item.activated = true;
                    m_uow->updateBinderItem({item});

                    // Reactivate its Contents
                    auto contentIds = m_uow->getBinderItemRelationship(
                        itemId, SCDBinderItem::BinderItemRelationshipField::Contents);
                    if (!contentIds.isEmpty())
                    {
                        auto contents = m_uow->getContent(contentIds);
                        for (auto &c : contents)
                            c.activated = true;
                        m_uow->updateContent(contents);
                    }

                    // Check if origin binder still exists and is active
                    if (ti.originBinderId > 0)
                    {
                        auto binders = m_uow->getBinder({ti.originBinderId});
                        if (binders.isEmpty() || !binders.first().activated)
                            result.orphaned = true;
                    }

                    result.restoredCount++;
                }
            }
            else if (ti.trashedBinder.has_value())
            {
                // Restore a Binder and all its items
                int binderId = ti.trashedBinder.value();
                auto binders = m_uow->getBinder({binderId});
                if (!binders.isEmpty())
                {
                    auto binder = binders.first();
                    binder.activated = true;
                    m_uow->updateBinder({binder});

                    // Reactivate all BinderItems
                    auto itemIds = m_uow->getBinderRelationship(
                        binderId, SCDBinder::BinderRelationshipField::BinderItems);
                    if (!itemIds.isEmpty())
                    {
                        auto items = m_uow->getBinderItem(itemIds);
                        for (auto &item : items)
                            item.activated = true;
                        m_uow->updateBinderItem(items);

                        // Reactivate all Contents
                        for (int itemId : itemIds)
                        {
                            auto contentIds = m_uow->getBinderItemRelationship(
                                itemId, SCDBinderItem::BinderItemRelationshipField::Contents);
                            if (!contentIds.isEmpty())
                            {
                                auto contents = m_uow->getContent(contentIds);
                                for (auto &c : contents)
                                    c.activated = true;
                                m_uow->updateContent(contents);
                            }
                        }
                    }

                    result.restoredCount++;
                }
            }
        }

        // Remove the TrashInfo records
        m_uow->removeTrashInfo(dto.trashInfoIds);

        // Update System's trashInfos relationship
        auto allTrashInfoIds = m_uow->getSystemRelationship(1, SCDSystem::SystemRelationshipField::TrashInfos);
        for (int id : dto.trashInfoIds)
            allTrashInfoIds.removeAll(id);
        m_uow->setSystemRelationship(1, SCDSystem::SystemRelationshipField::TrashInfos, allTrashInfoIds);

        if (!m_uow->commit())
            throw std::runtime_error("Failed to commit transaction");
    }
    catch (...)
    {
        m_uow->rollback();
        throw;
    }

    m_uow->publishRestoreItemsSignal();
    return result;
}

} // namespace Skribisto::TrashManagement

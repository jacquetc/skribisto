// Adapted for EmptyTrash use case

#include "empty_trash_uc.h"

namespace Skribisto::TrashManagement
{

EmptyTrashUseCase::EmptyTrashUseCase(std::unique_ptr<IEmptyTrashUnitOfWork> uow) : m_uow(std::move(uow))
{
}

bool EmptyTrashUseCase::execute() const
{
    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        // Get all TrashInfo records from System
        auto trashInfoIds = m_uow->getSystemRelationship(1, SCDSystem::SystemRelationshipField::TrashInfos);
        if (trashInfoIds.isEmpty())
        {
            m_uow->commit();
            return true; // nothing to empty
        }

        auto trashInfos = m_uow->getTrashInfo(trashInfoIds);

        for (const auto &ti : trashInfos)
        {
            if (ti.trashedBinderItem.has_value())
            {
                int itemId = ti.trashedBinderItem.value();

                // Remove Contents of this BinderItem
                auto contentIds =
                    m_uow->getBinderItemRelationship(itemId, SCDBinderItem::BinderItemRelationshipField::Contents);
                if (!contentIds.isEmpty())
                    m_uow->removeContent(contentIds);

                // Remove the BinderItem itself
                m_uow->removeBinderItem({itemId});
            }
            else if (ti.trashedBinder.has_value())
            {
                int binderId = ti.trashedBinder.value();

                // Remove all BinderItems and their Contents
                auto itemIds =
                    m_uow->getBinderRelationship(binderId, SCDBinder::BinderRelationshipField::BinderItems);
                for (int itemId : itemIds)
                {
                    auto contentIds = m_uow->getBinderItemRelationship(
                        itemId, SCDBinderItem::BinderItemRelationshipField::Contents);
                    if (!contentIds.isEmpty())
                        m_uow->removeContent(contentIds);
                }
                if (!itemIds.isEmpty())
                    m_uow->removeBinderItem(itemIds);

                // Remove the Binder itself
                m_uow->removeBinder({binderId});
            }
        }

        // Remove all TrashInfo records
        m_uow->removeTrashInfo(trashInfoIds);

        // Clear System's trashInfos relationship
        m_uow->setSystemRelationship(1, SCDSystem::SystemRelationshipField::TrashInfos, {});

        if (!m_uow->commit())
            throw std::runtime_error("Failed to commit transaction");
    }
    catch (...)
    {
        m_uow->rollback();
        throw;
    }

    m_uow->publishEmptyTrashSignal();
    return true;
}

} // namespace Skribisto::TrashManagement

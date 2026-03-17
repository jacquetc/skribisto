// Adapted for MoveItems use case

#include "move_items_uc.h"
#include <algorithm>
#include <limits>

namespace Skribisto::BinderItemManagement
{
namespace SCE = Common::Entities;
namespace SCDBinder = Common::DirectAccess::Binder;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

// Given a flat ordered list of IDs with indent levels, expand the requested IDs
// to include their implicit subtrees (consecutive items with higher indent).
// Returns the expanded list in binder order, with duplicates removed.
static QList<int> expandWithSubtrees(const QList<int> &binderItemIds, const QHash<int, int> &indentMap,
                                     const QList<int> &requestedIds)
{
    QSet<int> requestedSet(requestedIds.begin(), requestedIds.end());
    QSet<int> covered;
    QList<int> expanded;

    for (int i = 0; i < binderItemIds.size(); ++i)
    {
        int id = binderItemIds[i];
        if (!requestedSet.contains(id) || covered.contains(id))
            continue;

        expanded.append(id);
        covered.insert(id);

        int rootIndent = indentMap.value(id, 0);
        for (int j = i + 1; j < binderItemIds.size(); ++j)
        {
            int childId = binderItemIds[j];
            if (indentMap.value(childId, 0) > rootIndent)
            {
                expanded.append(childId);
                covered.insert(childId);
            }
            else
                break;
        }
    }

    return expanded;
}

// Find the end of an item's subtree in the flat list (exclusive index).
static int findSubtreeEnd(const QList<int> &binderItemIds, const QHash<int, int> &indentMap, int itemPos)
{
    int rootIndent = indentMap.value(binderItemIds[itemPos], 0);
    int end = itemPos + 1;
    while (end < binderItemIds.size() && indentMap.value(binderItemIds[end], 0) > rootIndent)
        ++end;
    return end;
}

MoveItemsUseCase::MoveItemsUseCase(std::unique_ptr<IMoveItemsUnitOfWork> uow) : m_uow(std::move(uow))
{
}

bool MoveItemsUseCase::execute(const MoveDto &moveDto)
{
    m_moveDto = moveDto;

    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        QList<int> requestedIds;
        for (uint id : moveDto.itemIds)
            requestedIds.append(static_cast<int>(id));

        if (requestedIds.isEmpty())
        {
            if (!m_uow->commit())
                throw std::runtime_error("Failed to commit transaction");
            m_uow->publishMoveItemsSignal();
            return true;
        }

        int targetId = moveDto.targetId.has_value() ? static_cast<int>(moveDto.targetId.value()) : -1;

        // Find source binder
        auto allBinders = m_uow->getAllBinder();
        int sourceBinder = -1;
        QList<int> sourceItemIds;

        for (const auto &binder : allBinders)
        {
            auto binderItems =
                m_uow->getBinderRelationship(binder.id, SCDBinder::BinderRelationshipField::BinderItems);
            if (binderItems.contains(requestedIds.first()))
            {
                sourceBinder = binder.id;
                sourceItemIds = binderItems;
                break;
            }
        }

        if (sourceBinder < 0)
            throw std::runtime_error("Could not find source binder for items");

        // Find target binder (may differ from source)
        int targetBinder = sourceBinder;
        QList<int> targetItemIds = sourceItemIds;

        if (targetId >= 0 && !sourceItemIds.contains(targetId))
        {
            for (const auto &binder : allBinders)
            {
                auto binderItems =
                    m_uow->getBinderRelationship(binder.id, SCDBinder::BinderRelationshipField::BinderItems);
                if (binderItems.contains(targetId))
                {
                    targetBinder = binder.id;
                    targetItemIds = binderItems;
                    break;
                }
            }

            if (targetBinder == sourceBinder)
                throw std::runtime_error("Could not find target binder");
        }

        // Snapshot affected binders for undo
        QList<int> binderIdsToSnapshot = {sourceBinder};
        if (targetBinder != sourceBinder)
            binderIdsToSnapshot.append(targetBinder);
        m_undoSnapshot = m_uow->snapshotBinder(binderIdsToSnapshot);

        // Fetch source items to get indents
        auto sourceEntities = m_uow->getBinderItem(sourceItemIds);
        QHash<int, int> sourceIndentMap;
        for (const auto &item : sourceEntities)
            sourceIndentMap[item.id] = item.indent;

        // Expand selection to include subtrees
        QList<int> expandedIds = expandWithSubtrees(sourceItemIds, sourceIndentMap, requestedIds);

        // Validate: target must not be inside the moved subtree
        if (targetId >= 0)
        {
            QSet<int> expandedSet(expandedIds.begin(), expandedIds.end());
            if (expandedSet.contains(targetId))
                throw std::runtime_error("Cannot move items relative to an item within the moved subtree");
        }

        // Build target indent map (may be same as source for same-binder)
        QHash<int, int> targetIndentMap;
        if (sourceBinder == targetBinder)
        {
            targetIndentMap = sourceIndentMap;
        }
        else
        {
            auto targetEntities = m_uow->getBinderItem(targetItemIds);
            for (const auto &item : targetEntities)
                targetIndentMap[item.id] = item.indent;
        }

        // Determine insertion position and new root indent
        int insertPos = 0;
        int newRootIndent = 0;

        if (targetId < 0)
        {
            if (sourceBinder == targetBinder)
                insertPos = (moveDto.movePlace == MovePlace::Before) ? 0 : sourceItemIds.size();
            else
                insertPos = (moveDto.movePlace == MovePlace::Before) ? 0 : targetItemIds.size();
        }
        else
        {
            int targetIndent = targetIndentMap.value(targetId, 0);
            const QList<int> &targetList = (sourceBinder == targetBinder) ? sourceItemIds : targetItemIds;
            int targetPos = targetList.indexOf(targetId);
            int subtreeEnd = findSubtreeEnd(targetList, targetIndentMap, targetPos);

            switch (moveDto.movePlace)
            {
            case MovePlace::Before:
                insertPos = targetPos;
                newRootIndent = targetIndent;
                break;
            case MovePlace::After:
                insertPos = subtreeEnd;
                newRootIndent = targetIndent;
                break;
            case MovePlace::Into:
                insertPos = subtreeEnd;
                newRootIndent = targetIndent + 1;
                break;
            }
        }

        // Calculate indent delta from the root items of the moved set
        int minRootIndent = std::numeric_limits<int>::max();
        for (int id : requestedIds)
        {
            if (sourceIndentMap.contains(id))
                minRootIndent = std::min(minRootIndent, sourceIndentMap.value(id));
        }
        int indentDelta = newRootIndent - minRootIndent;

        // Perform the list reordering
        // Remove expanded items from source list
        QList<int> newSourceItems = sourceItemIds;
        for (int id : expandedIds)
            newSourceItems.removeAll(id);

        if (sourceBinder == targetBinder)
        {
            // Adjust insert position since items were removed from the same list
            int adjustedPos = insertPos;
            for (int id : expandedIds)
            {
                int origPos = sourceItemIds.indexOf(id);
                if (origPos < insertPos)
                    --adjustedPos;
            }

            for (int i = 0; i < expandedIds.size(); ++i)
                newSourceItems.insert(adjustedPos + i, expandedIds[i]);

            m_uow->setBinderRelationship(
                sourceBinder, SCDBinder::BinderRelationshipField::BinderItems, newSourceItems);
        }
        else
        {
            // Update source
            m_uow->setBinderRelationship(
                sourceBinder, SCDBinder::BinderRelationshipField::BinderItems, newSourceItems);

            // Insert into target
            QList<int> newTargetItems = targetItemIds;
            for (int i = 0; i < expandedIds.size(); ++i)
                newTargetItems.insert(insertPos + i, expandedIds[i]);

            m_uow->setBinderRelationship(
                targetBinder, SCDBinder::BinderRelationshipField::BinderItems, newTargetItems);
        }

        // Adjust indents for all moved items
        if (indentDelta != 0)
        {
            auto movedEntities = m_uow->getBinderItem(expandedIds);
            const auto now = QDateTime::currentDateTimeUtc();
            for (auto &item : movedEntities)
            {
                item.indent += indentDelta;
                item.updatedAt = now;
            }
            m_uow->updateBinderItem(movedEntities);
        }

        if (!m_uow->commit())
            throw std::runtime_error("Failed to commit transaction");
    }
    catch (...)
    {
        m_uow->rollback();
        throw;
    }

    m_uow->publishMoveItemsSignal();
    return true;
}

Skribisto::Common::UndoRedo::Result<void> MoveItemsUseCase::undo()
{
    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        // Restore binder snapshot (restores ordering and item fields including indent)
        m_uow->restoreBinder(m_undoSnapshot);

        if (!m_uow->commit())
            throw std::runtime_error("Failed to commit transaction");
    }
    catch (...)
    {
        m_uow->rollback();
        throw;
    }

    m_uow->publishMoveItemsSignal();
    return Skribisto::Common::UndoRedo::Result<void>{};
}

Skribisto::Common::UndoRedo::Result<void> MoveItemsUseCase::redo()
{
    try
    {
        execute(m_moveDto);
    }
    catch (const std::exception &e)
    {
        return Skribisto::Common::UndoRedo::Result<void>{QString::fromUtf8(e.what())};
    }
    return Skribisto::Common::UndoRedo::Result<void>{};
}

} // namespace Skribisto::BinderItemManagement

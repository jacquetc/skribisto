// Adapted for Duplicate use case

#include "duplicate_uc.h"
#include <algorithm>

namespace Skribisto::BinderItemManagement
{
namespace SCE = Common::Entities;
namespace SCDBinder = Common::DirectAccess::Binder;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

// A subtree group: a root item and its implicit children (consecutive items with higher indent).
struct SubtreeGroup
{
    QList<int> originalIds; // ordered list of item IDs in this subtree
    int insertionPos;       // position to insert duplicates (right after the original subtree)
};

// Walk the binder's ordered list and build subtree groups for each requested item.
// Items already covered by a previous group's subtree are skipped.
static QList<SubtreeGroup> buildSubtreeGroups(const QList<int> &binderItemIds, const QHash<int, int> &indentMap,
                                              const QList<int> &requestedIds)
{
    QSet<int> requestedSet(requestedIds.begin(), requestedIds.end());
    QSet<int> covered;
    QList<SubtreeGroup> groups;

    for (int i = 0; i < binderItemIds.size(); ++i)
    {
        int id = binderItemIds[i];
        if (!requestedSet.contains(id) || covered.contains(id))
            continue;

        SubtreeGroup group;
        group.originalIds.append(id);
        covered.insert(id);

        int rootIndent = indentMap.value(id, 0);
        int j = i + 1;
        while (j < binderItemIds.size() && indentMap.value(binderItemIds[j], 0) > rootIndent)
        {
            group.originalIds.append(binderItemIds[j]);
            covered.insert(binderItemIds[j]);
            ++j;
        }
        group.insertionPos = j; // right after the subtree
        groups.append(group);
    }

    return groups;
}

DuplicateUseCase::DuplicateUseCase(std::unique_ptr<IDuplicateUnitOfWork> uow) : m_uow(std::move(uow))
{
}

DuplicateReturnDto DuplicateUseCase::execute(const DuplicateDto &duplicateDto)
{
    m_duplicateDto = duplicateDto;
    m_createdItemIds.clear();
    m_createdContentIds.clear();

    DuplicateReturnDto result;

    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        QList<int> requestedIds;
        for (uint id : duplicateDto.itemIds)
            requestedIds.append(static_cast<int>(id));

        if (!requestedIds.isEmpty())
        {
            // Find which binder owns each item and group by binder
            auto allBinders = m_uow->getAllBinder();
            QHash<int, QList<int>> binderToRequested;  // binderId -> requested items in this binder
            QHash<int, QList<int>> binderItemLists;    // binderId -> full ordered item list

            for (const auto &binder : allBinders)
            {
                auto binderItems =
                    m_uow->getBinderRelationship(binder.id, SCDBinder::BinderRelationshipField::BinderItems);
                for (int itemId : requestedIds)
                {
                    if (binderItems.contains(itemId))
                    {
                        binderToRequested[binder.id].append(itemId);
                        if (!binderItemLists.contains(binder.id))
                            binderItemLists[binder.id] = binderItems;
                    }
                }
            }

            // Snapshot all affected binders for undo
            QList<int> binderIds = binderToRequested.keys();
            m_undoSnapshot = m_uow->snapshotBinder(binderIds);

            const auto now = QDateTime::currentDateTimeUtc();
            QHash<int, int> origToNew; // original root ID -> new root ID (for result)

            for (int binderId : binderIds)
            {
                const auto &binderItemList = binderItemLists[binderId];
                const auto &requested = binderToRequested[binderId];

                // Fetch all items in this binder to get indents
                auto allItems = m_uow->getBinderItem(binderItemList);
                QHash<int, int> indentMap;
                QHash<int, SCE::BinderItem> itemMap;
                for (const auto &item : allItems)
                {
                    indentMap[item.id] = item.indent;
                    itemMap[item.id] = item;
                }

                // Build subtree groups
                auto groups = buildSubtreeGroups(binderItemList, indentMap, requested);

                // Process from last to first so insertions don't shift earlier positions
                for (int g = groups.size() - 1; g >= 0; --g)
                {
                    const auto &group = groups[g];

                    // Build duplicate items preserving order and indent structure
                    QList<SCE::BinderItem> duplicates;
                    for (int origId : group.originalIds)
                    {
                        SCE::BinderItem dup = itemMap.value(origId);
                        dup.id = 0;
                        dup.createdAt = now;
                        dup.updatedAt = now;
                        dup.contents = {};
                        dup.references = {};
                        dup.tags = {};
                        duplicates.append(dup);
                    }

                    auto created = m_uow->createBinderItem(duplicates, binderId, group.insertionPos);
                    if (created.size() != group.originalIds.size())
                        throw std::runtime_error("Failed to create duplicate items");

                    // Process each original-duplicate pair
                    for (int k = 0; k < group.originalIds.size(); ++k)
                    {
                        int origId = group.originalIds[k];
                        int newId = created[k].id;
                        m_createdItemIds.append(newId);

                        // Track root items for result
                        if (requested.contains(origId))
                            origToNew[origId] = newId;

                        // Copy contents (deep copy)
                        auto contentIds = m_uow->getBinderItemRelationship(
                            origId, SCDBinderItem::BinderItemRelationshipField::Contents);
                        if (!contentIds.isEmpty())
                        {
                            auto contents = m_uow->getContent(contentIds);
                            for (auto &c : contents)
                            {
                                c.id = 0;
                                c.createdAt = now;
                                c.updatedAt = now;
                            }
                            auto createdContents = m_uow->createContent(contents, newId, 0);
                            for (const auto &c : createdContents)
                                m_createdContentIds.append(c.id);
                        }

                        // Copy references (weak relationship — same target IDs)
                        auto refIds = m_uow->getBinderItemRelationship(
                            origId, SCDBinderItem::BinderItemRelationshipField::References);
                        if (!refIds.isEmpty())
                            m_uow->setBinderItemRelationship(
                                newId, SCDBinderItem::BinderItemRelationshipField::References, refIds);

                        // Copy tags (weak relationship — same target IDs)
                        auto tagIds = m_uow->getBinderItemRelationship(
                            origId, SCDBinderItem::BinderItemRelationshipField::Tags);
                        if (!tagIds.isEmpty())
                            m_uow->setBinderItemRelationship(
                                newId, SCDBinderItem::BinderItemRelationshipField::Tags, tagIds);
                    }
                }
            }

            // Build result in input order (only root IDs, not subtree children)
            for (uint origId : duplicateDto.itemIds)
            {
                int newId = origToNew.value(static_cast<int>(origId), 0);
                if (newId > 0)
                    result.newItemIds.append(static_cast<uint>(newId));
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

    m_uow->publishDuplicateSignal();
    return result;
}

Skribisto::Common::UndoRedo::Result<void> DuplicateUseCase::undo()
{
    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        // Remove created contents first (children before parents)
        if (!m_createdContentIds.isEmpty())
            m_uow->removeContent(m_createdContentIds);

        // Remove created items
        if (!m_createdItemIds.isEmpty())
            m_uow->removeBinderItem(m_createdItemIds);

        // Restore binder snapshot (restores junction table ordering)
        m_uow->restoreBinder(m_undoSnapshot);

        if (!m_uow->commit())
            throw std::runtime_error("Failed to commit transaction");
    }
    catch (...)
    {
        m_uow->rollback();
        throw;
    }

    m_uow->publishDuplicateSignal();
    return Skribisto::Common::UndoRedo::Result<void>{};
}

Skribisto::Common::UndoRedo::Result<void> DuplicateUseCase::redo()
{
    try
    {
        [[maybe_unused]] auto _ = execute(m_duplicateDto);
    }
    catch (const std::exception &e)
    {
        return Skribisto::Common::UndoRedo::Result<void>{QString::fromUtf8(e.what())};
    }
    return Skribisto::Common::UndoRedo::Result<void>{};
}

} // namespace Skribisto::BinderItemManagement

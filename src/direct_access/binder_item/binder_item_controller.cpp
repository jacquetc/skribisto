/******************************************************************************
 Copyright (C) 2025 by Cyril Jacquet                                          *
 cyril.jacquet@skribisto.eu                                                   *
                                                                              *
 This file is part of Skribisto.                                              *
                                                                              *
 Skribisto is free software: you can redistribute it and/or modify            *
 it under the terms of the GNU General Public License as published by         *
 the Free Software Foundation, either version 3 of the License, or            *
 (at your option) any later version.                                          *
                                                                              *
 Skribisto is distributed in the hope that it will be useful,                 *
 but WITHOUT ANY WARRANTY; without even the implied warranty of               *
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                *
 GNU General Public License for more details.                                 *
                                                                              *
 You should have received a copy of the GNU General Public License            *
 along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.           *
 ******************************************************************************/

#include "binder_item_controller.h"

#include "binder_item_unit_of_work.h"
#include "service_locator.h"
#include "use_cases/common/dto_mapper.h"
#include "use_cases/create_uc.h"
#include "use_cases/get_relationship_ids_count_uc.h"
#include "use_cases/get_relationship_ids_in_range_uc.h"
#include "use_cases/get_relationship_ids_many_uc.h"
#include "use_cases/get_relationship_ids_uc.h"
#include "use_cases/get_uc.h"
#include "use_cases/remove_uc.h"
#include "use_cases/set_relationship_ids_uc.h"
#include "use_cases/update_uc.h"
#include <QCoro/QCoroTask>
#include <QCoro/QCoroTimer>

#include <memory>

namespace Skribisto::DirectAccess::BinderItem
{
namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;

BinderItemController::BinderItemController(QObject *parent) : QObject(parent)
{
    resolveDependencies();
}
void BinderItemController::resolveDependencies()
{
    auto *locator = Common::ServiceLocator::instance(); // set by provider
    if (!locator)
    {
        qCritical() << "ServiceLocator not initialized";
        return;
    }
    m_dbContext = locator->dbContext();
    m_eventRegistry = locator->eventRegistry();
    m_undoRedoSystem = locator->undoRedoSystem();
}

QCoro::Task<QList<BinderItemDto>> BinderItemController::create(const QList<CreateBinderItemDto> &binderItems)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<BinderItemDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IBinderItemUnitOfWork> uow = std::make_unique<BinderItemUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<CreateBinderItemUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Create BinderItems Command"_L1);
    QList<BinderItemDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<CreateBinderItemUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, binderItems, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(binderItems);
        }
    });

    // Prepare lambda for redo - use weak_ptr to avoid circular reference
    command->setRedoFunction([weakUseCase]() -> Common::UndoRedo::Result<void> {
        if (auto useCase = weakUseCase.lock())
        {
            return useCase->redo();
        }
        return Common::UndoRedo::Result<void>("UseCase no longer available"_L1,
                                              Common::UndoRedo::ErrorCategory::ExecutionError);
    });

    // Prepare lambda for undo - use weak_ptr to avoid circular reference
    command->setUndoFunction([weakUseCase]() -> Common::UndoRedo::Result<void> {
        if (auto useCase = weakUseCase.lock())
        {
            return useCase->undo();
        }
        return Common::UndoRedo::Result<void>("UseCase no longer available"_L1,
                                              Common::UndoRedo::ErrorCategory::ExecutionError);
    });

    // Store the useCase in the command to maintain ownership
    // This ensures the useCase stays alive as long as the command exists
    command->setProperty("useCase", QVariant::fromValue(useCase));

    // Execute command asynchronously using QCoro integration
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, 0);

    if (!success.has_value())
    {
        qWarning() << "Create binderItem command execution timed out";
        co_return QList<BinderItemDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute create binderItem command";
        co_return QList<BinderItemDto>();
    }

    co_return result;
}
QCoro::Task<QList<BinderItemDto>> BinderItemController::get(const QList<int> &binderItemIds)
{
    // Use undo/redo query system with QCoro integration
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<BinderItemDto>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<BinderItemDto>>("Get BinderItems Query"_L1);
    query->setQueryFunction([this, binderItemIds]() -> QList<BinderItemDto> {
        std::unique_ptr<IBinderItemUnitOfWork> uow =
            std::make_unique<BinderItemUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetBinderItemUseCase>(std::move(uow));
        return useCase->execute(binderItemIds);
    });

    // Execute query asynchronously using QCoro integration
    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
QCoro::Task<QList<BinderItemDto>> BinderItemController::update(const QList<BinderItemDto> &binderItems)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<BinderItemDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IBinderItemUnitOfWork> uow = std::make_unique<BinderItemUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<UpdateBinderItemUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Update BinderItems Command"_L1);
    QList<BinderItemDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<UpdateBinderItemUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, binderItems, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(binderItems);
        }
    });

    // Prepare lambda for redo - use weak_ptr to avoid circular reference
    command->setRedoFunction([weakUseCase]() -> Common::UndoRedo::Result<void> {
        if (auto useCase = weakUseCase.lock())
        {
            return useCase->redo();
        }
        return Common::UndoRedo::Result<void>("UseCase no longer available"_L1,
                                              Common::UndoRedo::ErrorCategory::ExecutionError);
    });

    // Prepare lambda for undo - use weak_ptr to avoid circular reference
    command->setUndoFunction([weakUseCase]() -> Common::UndoRedo::Result<void> {
        if (auto useCase = weakUseCase.lock())
        {
            return useCase->undo();
        }
        return Common::UndoRedo::Result<void>("UseCase no longer available"_L1,
                                              Common::UndoRedo::ErrorCategory::ExecutionError);
    });

    // Store the useCase in the command to maintain ownership
    // This ensures the useCase stays alive as long as the command exists
    command->setProperty("useCase", QVariant::fromValue(useCase));

    // Execute command asynchronously using QCoro integration
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, 0);

    if (!success.has_value())
    {
        qWarning() << "Update binderItem command execution timed out";
        co_return QList<BinderItemDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute update binderItem command";
        co_return QList<BinderItemDto>();
    }

    co_return result;
}
QCoro::Task<QList<int>> BinderItemController::remove(const QList<int> &binderItemIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IBinderItemUnitOfWork> uow = std::make_unique<BinderItemUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<RemoveBinderItemUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Remove BinderItems Command"_L1);
    QList<int> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<RemoveBinderItemUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, binderItemIds, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(binderItemIds);
        }
    });

    // Prepare lambda for redo - use weak_ptr to avoid circular reference
    command->setRedoFunction([weakUseCase]() -> Common::UndoRedo::Result<void> {
        if (auto useCase = weakUseCase.lock())
        {
            return useCase->redo();
        }
        return Common::UndoRedo::Result<void>("UseCase no longer available"_L1,
                                              Common::UndoRedo::ErrorCategory::ExecutionError);
    });

    // Prepare lambda for undo - use weak_ptr to avoid circular reference
    command->setUndoFunction([weakUseCase]() -> Common::UndoRedo::Result<void> {
        if (auto useCase = weakUseCase.lock())
        {
            return useCase->undo();
        }
        return Common::UndoRedo::Result<void>("UseCase no longer available"_L1,
                                              Common::UndoRedo::ErrorCategory::ExecutionError);
    });

    // Store the useCase in the command to maintain ownership
    // This ensures the useCase stays alive as long as the command exists
    command->setProperty("useCase", QVariant::fromValue(useCase));

    // Execute command asynchronously using QCoro integration
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, 0);

    if (!success.has_value())
    {
        qWarning() << "Remove binderItem command execution timed out";
        co_return QList<int>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute remove binderItem command";
        co_return QList<int>();
    }

    co_return result;
}

QCoro::Task<QList<int>> BinderItemController::getRelationshipIds(int binderItemId,
                                                                 BinderItemRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<int>>("Get BinderItem Relationship IDs Query"_L1);
    query->setQueryFunction([this, binderItemId, relationship]() -> QList<int> {
        std::unique_ptr<IBinderItemUnitOfWork> uow =
            std::make_unique<BinderItemUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsUseCase>(std::move(uow));
        return useCase->execute(binderItemId, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<void> BinderItemController::setRelationshipIds(int binderItemId, BinderItemRelationshipField relationship,
                                                           QList<int> relatedIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return;
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IBinderItemUnitOfWork> uow = std::make_unique<BinderItemUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<SetRelationshipIdsUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Set BinderItem Relationship IDs Command"_L1);

    // Create weak_ptr to break circular reference
    std::weak_ptr<SetRelationshipIdsUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, binderItemId, relationship, relatedIds](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            useCase->execute(binderItemId, relationship, relatedIds);
        }
    });

    // Prepare lambda for redo - use weak_ptr to avoid circular reference
    command->setRedoFunction([weakUseCase]() -> Common::UndoRedo::Result<void> {
        if (auto useCase = weakUseCase.lock())
        {
            return useCase->redo();
        }
        return Common::UndoRedo::Result<void>("UseCase no longer available"_L1,
                                              Common::UndoRedo::ErrorCategory::ExecutionError);
    });

    // Prepare lambda for undo - use weak_ptr to avoid circular reference
    command->setUndoFunction([weakUseCase]() -> Common::UndoRedo::Result<void> {
        if (auto useCase = weakUseCase.lock())
        {
            return useCase->undo();
        }
        return Common::UndoRedo::Result<void>("UseCase no longer available"_L1,
                                              Common::UndoRedo::ErrorCategory::ExecutionError);
    });

    // Store the useCase in the command to maintain ownership
    // This ensures the useCase stays alive as long as the command exists
    command->setProperty("useCase", QVariant::fromValue(useCase));

    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, 0);

    if (!success.has_value())
    {
        qWarning() << "Set binderItem relationship command execution timed out";
        co_return;
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute set binderItem relationship command";
        co_return;
    }
}

QCoro::Task<QHash<int, QList<int>>> BinderItemController::getRelationshipIdsMany(
    const QList<int> &binderItemIds, BinderItemRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QHash<int, QList<int>>();
    }

    auto query = m_undoRedoSystem->createQuery<QHash<int, QList<int>>>("Get BinderItem Relationship IDs Many Query"_L1);
    query->setQueryFunction([this, binderItemIds, relationship]() -> QHash<int, QList<int>> {
        std::unique_ptr<IBinderItemUnitOfWork> uow =
            std::make_unique<BinderItemUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsManyUseCase>(std::move(uow));
        return useCase->execute(binderItemIds, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<int> BinderItemController::getRelationshipIdsCount(int binderItemId,
                                                               BinderItemRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return 0;
    }

    auto query = m_undoRedoSystem->createQuery<int>("Get BinderItem Relationship IDs Count Query"_L1);
    query->setQueryFunction([this, binderItemId, relationship]() -> int {
        std::unique_ptr<IBinderItemUnitOfWork> uow =
            std::make_unique<BinderItemUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsCountUseCase>(std::move(uow));
        return useCase->execute(binderItemId, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<QList<int>> BinderItemController::getRelationshipIdsInRange(int binderItemId,
                                                                        BinderItemRelationshipField relationship,
                                                                        int offset, int limit)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<int>>("Get BinderItem Relationship IDs In Range Query"_L1);
    query->setQueryFunction([this, binderItemId, relationship, offset, limit]() -> QList<int> {
        std::unique_ptr<IBinderItemUnitOfWork> uow =
            std::make_unique<BinderItemUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsInRangeUseCase>(std::move(uow));
        return useCase->execute(binderItemId, relationship, offset, limit);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
} // namespace Skribisto::DirectAccess::BinderItem
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

#include "root_controller.h"

#include "root_unit_of_work.h"
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

namespace Skribisto::DirectAccess::Root
{
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;

RootController::RootController(QObject *parent) : QObject(parent)
{
    resolveDependencies();
}
void RootController::resolveDependencies()
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

QCoro::Task<QList<RootDto>> RootController::create(const QList<CreateRootDto> &roots)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<RootDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IRootUnitOfWork> uow = std::make_unique<RootUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<CreateRootUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Create Roots Command"_L1);
    QList<RootDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<CreateRootUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, roots, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(roots);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "root_create"_L1);

    if (!success.has_value())
    {
        qWarning() << "Create root command execution timed out";
        co_return QList<RootDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute create root command";
        co_return QList<RootDto>();
    }

    co_return result;
}
QCoro::Task<QList<RootDto>> RootController::get(const QList<int> &rootIds)
{
    // Use undo/redo query system with QCoro integration
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<RootDto>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<RootDto>>("Get Roots Query"_L1);
    query->setQueryFunction([this, rootIds]() -> QList<RootDto> {
        std::unique_ptr<IRootUnitOfWork> uow = std::make_unique<RootUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRootUseCase>(std::move(uow));
        return useCase->execute(rootIds);
    });

    // Execute query asynchronously using QCoro integration
    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
QCoro::Task<QList<RootDto>> RootController::update(const QList<RootDto> &roots)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<RootDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IRootUnitOfWork> uow = std::make_unique<RootUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<UpdateRootUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Update Roots Command"_L1);
    QList<RootDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<UpdateRootUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, roots, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(roots);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "root_update"_L1);

    if (!success.has_value())
    {
        qWarning() << "Update root command execution timed out";
        co_return QList<RootDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute update root command";
        co_return QList<RootDto>();
    }

    co_return result;
}
QCoro::Task<QList<int>> RootController::remove(const QList<int> &rootIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IRootUnitOfWork> uow = std::make_unique<RootUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<RemoveRootUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Remove Roots Command"_L1);
    QList<int> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<RemoveRootUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, rootIds, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(rootIds);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "root_remove"_L1);

    if (!success.has_value())
    {
        qWarning() << "Remove root command execution timed out";
        co_return QList<int>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute remove root command";
        co_return QList<int>();
    }

    co_return result;
}

QCoro::Task<QList<int>> RootController::getRelationshipIds(int rootId, RootRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<int>>("Get Root Relationship IDs Query"_L1);
    query->setQueryFunction([this, rootId, relationship]() -> QList<int> {
        std::unique_ptr<IRootUnitOfWork> uow = std::make_unique<RootUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsUseCase>(std::move(uow));
        return useCase->execute(rootId, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<void> RootController::setRelationshipIds(int rootId, RootRelationshipField relationship,
                                                     QList<int> relatedIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return;
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IRootUnitOfWork> uow = std::make_unique<RootUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<SetRelationshipIdsUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Set Root Relationship IDs Command"_L1);

    // Create weak_ptr to break circular reference
    std::weak_ptr<SetRelationshipIdsUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, rootId, relationship, relatedIds](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            useCase->execute(rootId, relationship, relatedIds);
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

    std::optional<bool> success =
        co_await m_undoRedoSystem->executeCommandAsync(command, 500, "root_set_relationship"_L1);

    if (!success.has_value())
    {
        qWarning() << "Set root relationship command execution timed out";
        co_return;
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute set root relationship command";
        co_return;
    }
}

QCoro::Task<QHash<int, QList<int>>> RootController::getRelationshipIdsMany(const QList<int> &rootIds,
                                                                           RootRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QHash<int, QList<int>>();
    }

    auto query = m_undoRedoSystem->createQuery<QHash<int, QList<int>>>("Get Root Relationship IDs Many Query"_L1);
    query->setQueryFunction([this, rootIds, relationship]() -> QHash<int, QList<int>> {
        std::unique_ptr<IRootUnitOfWork> uow = std::make_unique<RootUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsManyUseCase>(std::move(uow));
        return useCase->execute(rootIds, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<int> RootController::getRelationshipIdsCount(int rootId, RootRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return 0;
    }

    auto query = m_undoRedoSystem->createQuery<int>("Get Root Relationship IDs Count Query"_L1);
    query->setQueryFunction([this, rootId, relationship]() -> int {
        std::unique_ptr<IRootUnitOfWork> uow = std::make_unique<RootUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsCountUseCase>(std::move(uow));
        return useCase->execute(rootId, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<QList<int>> RootController::getRelationshipIdsInRange(int rootId, RootRelationshipField relationship,
                                                                  int offset, int limit)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<int>>("Get Root Relationship IDs In Range Query"_L1);
    query->setQueryFunction([this, rootId, relationship, offset, limit]() -> QList<int> {
        std::unique_ptr<IRootUnitOfWork> uow = std::make_unique<RootUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsInRangeUseCase>(std::move(uow));
        return useCase->execute(rootId, relationship, offset, limit);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
} // namespace Skribisto::DirectAccess::Root
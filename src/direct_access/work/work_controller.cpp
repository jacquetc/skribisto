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

#include "work_controller.h"

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
#include "work_unit_of_work.h"
#include <QCoro/QCoroTask>
#include <QCoro/QCoroTimer>

#include <memory>

namespace Skribisto::DirectAccess::Work
{
namespace SCDWork = Skribisto::Common::DirectAccess::Work;

WorkController::WorkController(QObject *parent) : QObject(parent)
{
    resolveDependencies();
}
void WorkController::resolveDependencies()
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

QCoro::Task<QList<WorkDto>> WorkController::create(const QList<CreateWorkDto> &works)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<WorkDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IWorkUnitOfWork> uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<CreateWorkUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Create Works Command"_L1);
    QList<WorkDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<CreateWorkUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, works, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(works);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "work_create"_L1);

    if (!success.has_value())
    {
        qWarning() << "Create work command execution timed out";
        co_return QList<WorkDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute create work command";
        co_return QList<WorkDto>();
    }

    co_return result;
}
QCoro::Task<QList<WorkDto>> WorkController::get(const QList<int> &workIds)
{
    // Use undo/redo query system with QCoro integration
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<WorkDto>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<WorkDto>>("Get Works Query"_L1);
    query->setQueryFunction([this, workIds]() -> QList<WorkDto> {
        std::unique_ptr<IWorkUnitOfWork> uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetWorkUseCase>(std::move(uow));
        return useCase->execute(workIds);
    });

    // Execute query asynchronously using QCoro integration
    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
QCoro::Task<QList<WorkDto>> WorkController::update(const QList<WorkDto> &works)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<WorkDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IWorkUnitOfWork> uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<UpdateWorkUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Update Works Command"_L1);
    QList<WorkDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<UpdateWorkUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, works, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(works);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "work_update"_L1);

    if (!success.has_value())
    {
        qWarning() << "Update work command execution timed out";
        co_return QList<WorkDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute update work command";
        co_return QList<WorkDto>();
    }

    co_return result;
}
QCoro::Task<QList<int>> WorkController::remove(const QList<int> &workIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IWorkUnitOfWork> uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<RemoveWorkUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Remove Works Command"_L1);
    QList<int> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<RemoveWorkUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, workIds, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(workIds);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "work_remove"_L1);

    if (!success.has_value())
    {
        qWarning() << "Remove work command execution timed out";
        co_return QList<int>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute remove work command";
        co_return QList<int>();
    }

    co_return result;
}

QCoro::Task<QList<int>> WorkController::getRelationshipIds(int workId, WorkRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<int>>("Get Work Relationship IDs Query"_L1);
    query->setQueryFunction([this, workId, relationship]() -> QList<int> {
        std::unique_ptr<IWorkUnitOfWork> uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsUseCase>(std::move(uow));
        return useCase->execute(workId, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<void> WorkController::setRelationshipIds(int workId, WorkRelationshipField relationship,
                                                     QList<int> relatedIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return;
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IWorkUnitOfWork> uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<SetRelationshipIdsUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Set Work Relationship IDs Command"_L1);

    // Create weak_ptr to break circular reference
    std::weak_ptr<SetRelationshipIdsUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, workId, relationship, relatedIds](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            useCase->execute(workId, relationship, relatedIds);
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
        co_await m_undoRedoSystem->executeCommandAsync(command, 500, "work_set_relationship"_L1);

    if (!success.has_value())
    {
        qWarning() << "Set work relationship command execution timed out";
        co_return;
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute set work relationship command";
        co_return;
    }
}

QCoro::Task<QHash<int, QList<int>>> WorkController::getRelationshipIdsMany(const QList<int> &workIds,
                                                                           WorkRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QHash<int, QList<int>>();
    }

    auto query = m_undoRedoSystem->createQuery<QHash<int, QList<int>>>("Get Work Relationship IDs Many Query"_L1);
    query->setQueryFunction([this, workIds, relationship]() -> QHash<int, QList<int>> {
        std::unique_ptr<IWorkUnitOfWork> uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsManyUseCase>(std::move(uow));
        return useCase->execute(workIds, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<int> WorkController::getRelationshipIdsCount(int workId, WorkRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return 0;
    }

    auto query = m_undoRedoSystem->createQuery<int>("Get Work Relationship IDs Count Query"_L1);
    query->setQueryFunction([this, workId, relationship]() -> int {
        std::unique_ptr<IWorkUnitOfWork> uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsCountUseCase>(std::move(uow));
        return useCase->execute(workId, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<QList<int>> WorkController::getRelationshipIdsInRange(int workId, WorkRelationshipField relationship,
                                                                  int offset, int limit)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<int>>("Get Work Relationship IDs In Range Query"_L1);
    query->setQueryFunction([this, workId, relationship, offset, limit]() -> QList<int> {
        std::unique_ptr<IWorkUnitOfWork> uow = std::make_unique<WorkUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsInRangeUseCase>(std::move(uow));
        return useCase->execute(workId, relationship, offset, limit);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
} // namespace Skribisto::DirectAccess::Work
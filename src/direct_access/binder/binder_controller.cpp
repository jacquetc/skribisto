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

#include "binder_controller.h"

#include "binder_unit_of_work.h"
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

namespace Skribisto::DirectAccess::Binder
{
namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;

BinderController::BinderController(QObject *parent) : QObject(parent)
{
    resolveDependencies();
}
void BinderController::resolveDependencies()
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

QCoro::Task<QList<BinderDto>> BinderController::create(const QList<CreateBinderDto> &binders)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<BinderDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IBinderUnitOfWork> uow = std::make_unique<BinderUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<CreateBinderUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Create Binders Command"_L1);
    QList<BinderDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<CreateBinderUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, binders, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(binders);
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
        qWarning() << "Create binder command execution timed out";
        co_return QList<BinderDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute create binder command";
        co_return QList<BinderDto>();
    }

    co_return result;
}
QCoro::Task<QList<BinderDto>> BinderController::get(const QList<int> &binderIds)
{
    // Use undo/redo query system with QCoro integration
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<BinderDto>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<BinderDto>>("Get Binders Query"_L1);
    query->setQueryFunction([this, binderIds]() -> QList<BinderDto> {
        std::unique_ptr<IBinderUnitOfWork> uow = std::make_unique<BinderUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetBinderUseCase>(std::move(uow));
        return useCase->execute(binderIds);
    });

    // Execute query asynchronously using QCoro integration
    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
QCoro::Task<QList<BinderDto>> BinderController::update(const QList<BinderDto> &binders)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<BinderDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IBinderUnitOfWork> uow = std::make_unique<BinderUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<UpdateBinderUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Update Binders Command"_L1);
    QList<BinderDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<UpdateBinderUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, binders, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(binders);
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
        qWarning() << "Update binder command execution timed out";
        co_return QList<BinderDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute update binder command";
        co_return QList<BinderDto>();
    }

    co_return result;
}
QCoro::Task<QList<int>> BinderController::remove(const QList<int> &binderIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IBinderUnitOfWork> uow = std::make_unique<BinderUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<RemoveBinderUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Remove Binders Command"_L1);
    QList<int> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<RemoveBinderUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, binderIds, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(binderIds);
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
        qWarning() << "Remove binder command execution timed out";
        co_return QList<int>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute remove binder command";
        co_return QList<int>();
    }

    co_return result;
}

QCoro::Task<QList<int>> BinderController::getRelationshipIds(int binderId, BinderRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<int>>("Get Binder Relationship IDs Query"_L1);
    query->setQueryFunction([this, binderId, relationship]() -> QList<int> {
        std::unique_ptr<IBinderUnitOfWork> uow = std::make_unique<BinderUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsUseCase>(std::move(uow));
        return useCase->execute(binderId, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<void> BinderController::setRelationshipIds(int binderId, BinderRelationshipField relationship,
                                                       QList<int> relatedIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return;
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IBinderUnitOfWork> uow = std::make_unique<BinderUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<SetRelationshipIdsUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Set Binder Relationship IDs Command"_L1);

    // Create weak_ptr to break circular reference
    std::weak_ptr<SetRelationshipIdsUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, binderId, relationship, relatedIds](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            useCase->execute(binderId, relationship, relatedIds);
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
        qWarning() << "Set binder relationship command execution timed out";
        co_return;
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute set binder relationship command";
        co_return;
    }
}

QCoro::Task<QHash<int, QList<int>>> BinderController::getRelationshipIdsMany(const QList<int> &binderIds,
                                                                             BinderRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QHash<int, QList<int>>();
    }

    auto query = m_undoRedoSystem->createQuery<QHash<int, QList<int>>>("Get Binder Relationship IDs Many Query"_L1);
    query->setQueryFunction([this, binderIds, relationship]() -> QHash<int, QList<int>> {
        std::unique_ptr<IBinderUnitOfWork> uow = std::make_unique<BinderUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsManyUseCase>(std::move(uow));
        return useCase->execute(binderIds, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<int> BinderController::getRelationshipIdsCount(int binderId, BinderRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return 0;
    }

    auto query = m_undoRedoSystem->createQuery<int>("Get Binder Relationship IDs Count Query"_L1);
    query->setQueryFunction([this, binderId, relationship]() -> int {
        std::unique_ptr<IBinderUnitOfWork> uow = std::make_unique<BinderUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsCountUseCase>(std::move(uow));
        return useCase->execute(binderId, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<QList<int>> BinderController::getRelationshipIdsInRange(int binderId, BinderRelationshipField relationship,
                                                                    int offset, int limit)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<int>>("Get Binder Relationship IDs In Range Query"_L1);
    query->setQueryFunction([this, binderId, relationship, offset, limit]() -> QList<int> {
        std::unique_ptr<IBinderUnitOfWork> uow = std::make_unique<BinderUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsInRangeUseCase>(std::move(uow));
        return useCase->execute(binderId, relationship, offset, limit);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
} // namespace Skribisto::DirectAccess::Binder
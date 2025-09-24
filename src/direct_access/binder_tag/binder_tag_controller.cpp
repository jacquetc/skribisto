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

#include "binder_tag_controller.h"

#include "binder_tag_unit_of_work.h"
#include "service_locator.h"
#include "use_cases/common/dto_mapper.h"
#include "use_cases/create_uc.h"
#include "use_cases/get_uc.h"
#include "use_cases/remove_uc.h"
#include "use_cases/update_uc.h"
#include <QCoro/QCoroTask>
#include <QCoro/QCoroTimer>

#include <memory>

namespace Skribisto::DirectAccess::BinderTag
{
namespace SCDBinderTag = Skribisto::Common::DirectAccess::BinderTag;

BinderTagController::BinderTagController(QObject *parent) : QObject(parent)
{
    resolveDependencies();
}
void BinderTagController::resolveDependencies()
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

QCoro::Task<QList<BinderTagDto>> BinderTagController::create(const QList<CreateBinderTagDto> &binderTags)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<BinderTagDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IBinderTagUnitOfWork> uow = std::make_unique<BinderTagUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<CreateBinderTagUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Create BinderTags Command"_L1);
    QList<BinderTagDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<CreateBinderTagUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, binderTags, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(binderTags);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "binderTag_create"_L1);

    if (!success.has_value())
    {
        qWarning() << "Create binderTag command execution timed out";
        co_return QList<BinderTagDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute create binderTag command";
        co_return QList<BinderTagDto>();
    }

    co_return result;
}
QCoro::Task<QList<BinderTagDto>> BinderTagController::get(const QList<int> &binderTagIds)
{
    // Use undo/redo query system with QCoro integration
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<BinderTagDto>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<BinderTagDto>>("Get BinderTags Query"_L1);
    query->setQueryFunction([this, binderTagIds]() -> QList<BinderTagDto> {
        std::unique_ptr<IBinderTagUnitOfWork> uow =
            std::make_unique<BinderTagUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetBinderTagUseCase>(std::move(uow));
        return useCase->execute(binderTagIds);
    });

    // Execute query asynchronously using QCoro integration
    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
QCoro::Task<QList<BinderTagDto>> BinderTagController::update(const QList<BinderTagDto> &binderTags)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<BinderTagDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IBinderTagUnitOfWork> uow = std::make_unique<BinderTagUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<UpdateBinderTagUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Update BinderTags Command"_L1);
    QList<BinderTagDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<UpdateBinderTagUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, binderTags, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(binderTags);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "binderTag_update"_L1);

    if (!success.has_value())
    {
        qWarning() << "Update binderTag command execution timed out";
        co_return QList<BinderTagDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute update binderTag command";
        co_return QList<BinderTagDto>();
    }

    co_return result;
}
QCoro::Task<QList<int>> BinderTagController::remove(const QList<int> &binderTagIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IBinderTagUnitOfWork> uow = std::make_unique<BinderTagUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<RemoveBinderTagUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Remove BinderTags Command"_L1);
    QList<int> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<RemoveBinderTagUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, binderTagIds, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(binderTagIds);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "binderTag_remove"_L1);

    if (!success.has_value())
    {
        qWarning() << "Remove binderTag command execution timed out";
        co_return QList<int>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute remove binderTag command";
        co_return QList<int>();
    }

    co_return result;
}

} // namespace Skribisto::DirectAccess::BinderTag
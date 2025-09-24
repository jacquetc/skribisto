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

#include "content_controller.h"

#include "content_unit_of_work.h"
#include "service_locator.h"
#include "use_cases/common/dto_mapper.h"
#include "use_cases/create_uc.h"
#include "use_cases/get_uc.h"
#include "use_cases/remove_uc.h"
#include "use_cases/update_uc.h"
#include <QCoro/QCoroTask>
#include <QCoro/QCoroTimer>

#include <memory>

namespace Skribisto::DirectAccess::Content
{
namespace SCDContent = Skribisto::Common::DirectAccess::Content;

ContentController::ContentController(QObject *parent) : QObject(parent)
{
    resolveDependencies();
}
void ContentController::resolveDependencies()
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

QCoro::Task<QList<ContentDto>> ContentController::create(const QList<CreateContentDto> &contents)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<ContentDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IContentUnitOfWork> uow = std::make_unique<ContentUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<CreateContentUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Create Contents Command"_L1);
    QList<ContentDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<CreateContentUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, contents, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(contents);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "content_create"_L1);

    if (!success.has_value())
    {
        qWarning() << "Create content command execution timed out";
        co_return QList<ContentDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute create content command";
        co_return QList<ContentDto>();
    }

    co_return result;
}
QCoro::Task<QList<ContentDto>> ContentController::get(const QList<int> &contentIds)
{
    // Use undo/redo query system with QCoro integration
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<ContentDto>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<ContentDto>>("Get Contents Query"_L1);
    query->setQueryFunction([this, contentIds]() -> QList<ContentDto> {
        std::unique_ptr<IContentUnitOfWork> uow = std::make_unique<ContentUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetContentUseCase>(std::move(uow));
        return useCase->execute(contentIds);
    });

    // Execute query asynchronously using QCoro integration
    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
QCoro::Task<QList<ContentDto>> ContentController::update(const QList<ContentDto> &contents)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<ContentDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IContentUnitOfWork> uow = std::make_unique<ContentUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<UpdateContentUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Update Contents Command"_L1);
    QList<ContentDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<UpdateContentUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, contents, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(contents);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "content_update"_L1);

    if (!success.has_value())
    {
        qWarning() << "Update content command execution timed out";
        co_return QList<ContentDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute update content command";
        co_return QList<ContentDto>();
    }

    co_return result;
}
QCoro::Task<QList<int>> ContentController::remove(const QList<int> &contentIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IContentUnitOfWork> uow = std::make_unique<ContentUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<RemoveContentUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Remove Contents Command"_L1);
    QList<int> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<RemoveContentUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, contentIds, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(contentIds);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "content_remove"_L1);

    if (!success.has_value())
    {
        qWarning() << "Remove content command execution timed out";
        co_return QList<int>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute remove content command";
        co_return QList<int>();
    }

    co_return result;
}

} // namespace Skribisto::DirectAccess::Content
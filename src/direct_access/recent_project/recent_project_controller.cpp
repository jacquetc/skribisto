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

#include "recent_project_controller.h"

#include "recent_project_unit_of_work.h"
#include "service_locator.h"
#include "use_cases/common/dto_mapper.h"
#include "use_cases/create_uc.h"
#include "use_cases/get_uc.h"
#include "use_cases/remove_uc.h"
#include "use_cases/update_uc.h"
#include <QCoro/QCoroTask>
#include <QCoro/QCoroTimer>

#include <memory>

namespace Skribisto::DirectAccess::RecentProject
{
namespace SCDRecentProject = Skribisto::Common::DirectAccess::RecentProject;

RecentProjectController::RecentProjectController(QObject *parent) : QObject(parent)
{
    resolveDependencies();
}
void RecentProjectController::resolveDependencies()
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

QCoro::Task<QList<RecentProjectDto>> RecentProjectController::create(
    const QList<CreateRecentProjectDto> &recentProjects)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<RecentProjectDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IRecentProjectUnitOfWork> uow =
        std::make_unique<RecentProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<CreateRecentProjectUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Create RecentProjects Command"_L1);
    QList<RecentProjectDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<CreateRecentProjectUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, recentProjects, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(recentProjects);
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
    std::optional<bool> success =
        co_await m_undoRedoSystem->executeCommandAsync(command, 500, "recentProject_create"_L1);

    if (!success.has_value())
    {
        qWarning() << "Create recentProject command execution timed out";
        co_return QList<RecentProjectDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute create recentProject command";
        co_return QList<RecentProjectDto>();
    }

    co_return result;
}
QCoro::Task<QList<RecentProjectDto>> RecentProjectController::get(const QList<int> &recentProjectIds)
{
    // Use undo/redo query system with QCoro integration
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<RecentProjectDto>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<RecentProjectDto>>("Get RecentProjects Query"_L1);
    query->setQueryFunction([this, recentProjectIds]() -> QList<RecentProjectDto> {
        std::unique_ptr<IRecentProjectUnitOfWork> uow =
            std::make_unique<RecentProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRecentProjectUseCase>(std::move(uow));
        return useCase->execute(recentProjectIds);
    });

    // Execute query asynchronously using QCoro integration
    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
QCoro::Task<QList<RecentProjectDto>> RecentProjectController::update(const QList<RecentProjectDto> &recentProjects)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<RecentProjectDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IRecentProjectUnitOfWork> uow =
        std::make_unique<RecentProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<UpdateRecentProjectUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Update RecentProjects Command"_L1);
    QList<RecentProjectDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<UpdateRecentProjectUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, recentProjects, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(recentProjects);
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
    std::optional<bool> success =
        co_await m_undoRedoSystem->executeCommandAsync(command, 500, "recentProject_update"_L1);

    if (!success.has_value())
    {
        qWarning() << "Update recentProject command execution timed out";
        co_return QList<RecentProjectDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute update recentProject command";
        co_return QList<RecentProjectDto>();
    }

    co_return result;
}
QCoro::Task<QList<int>> RecentProjectController::remove(const QList<int> &recentProjectIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IRecentProjectUnitOfWork> uow =
        std::make_unique<RecentProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<RemoveRecentProjectUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Remove RecentProjects Command"_L1);
    QList<int> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<RemoveRecentProjectUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, recentProjectIds, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(recentProjectIds);
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
    std::optional<bool> success =
        co_await m_undoRedoSystem->executeCommandAsync(command, 500, "recentProject_remove"_L1);

    if (!success.has_value())
    {
        qWarning() << "Remove recentProject command execution timed out";
        co_return QList<int>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute remove recentProject command";
        co_return QList<int>();
    }

    co_return result;
}

} // namespace Skribisto::DirectAccess::RecentProject
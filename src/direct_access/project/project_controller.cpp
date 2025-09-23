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

#include "project_controller.h"

#include "project_unit_of_work.h"
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

namespace Skribisto::DirectAccess::Project
{
namespace SCDProject = Skribisto::Common::DirectAccess::Project;

ProjectController::ProjectController(QObject *parent) : QObject(parent)
{
    resolveDependencies();
}
void ProjectController::resolveDependencies()
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

QCoro::Task<QList<ProjectDto>> ProjectController::create(const QList<CreateProjectDto> &projects)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<ProjectDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IProjectUnitOfWork> uow = std::make_unique<ProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<CreateProjectUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Create Projects Command"_L1);
    QList<ProjectDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<CreateProjectUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, projects, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(projects);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "project_create"_L1);

    if (!success.has_value())
    {
        qWarning() << "Create project command execution timed out";
        co_return QList<ProjectDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute create project command";
        co_return QList<ProjectDto>();
    }

    co_return result;
}
QCoro::Task<QList<ProjectDto>> ProjectController::get(const QList<int> &projectIds)
{
    // Use undo/redo query system with QCoro integration
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<ProjectDto>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<ProjectDto>>("Get Projects Query"_L1);
    query->setQueryFunction([this, projectIds]() -> QList<ProjectDto> {
        std::unique_ptr<IProjectUnitOfWork> uow = std::make_unique<ProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetProjectUseCase>(std::move(uow));
        return useCase->execute(projectIds);
    });

    // Execute query asynchronously using QCoro integration
    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
QCoro::Task<QList<ProjectDto>> ProjectController::update(const QList<ProjectDto> &projects)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<ProjectDto>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IProjectUnitOfWork> uow = std::make_unique<ProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<UpdateProjectUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Update Projects Command"_L1);
    QList<ProjectDto> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<UpdateProjectUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, projects, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(projects);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "project_update"_L1);

    if (!success.has_value())
    {
        qWarning() << "Update project command execution timed out";
        co_return QList<ProjectDto>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute update project command";
        co_return QList<ProjectDto>();
    }

    co_return result;
}
QCoro::Task<QList<int>> ProjectController::remove(const QList<int> &projectIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IProjectUnitOfWork> uow = std::make_unique<ProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<RemoveProjectUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Remove Projects Command"_L1);
    QList<int> result;

    // Create weak_ptr to break circular reference
    std::weak_ptr<RemoveProjectUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, projectIds, &result](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            result = useCase->execute(projectIds);
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
    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "project_remove"_L1);

    if (!success.has_value())
    {
        qWarning() << "Remove project command execution timed out";
        co_return QList<int>();
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute remove project command";
        co_return QList<int>();
    }

    co_return result;
}

QCoro::Task<QList<int>> ProjectController::getRelationshipIds(int projectId, ProjectRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<int>>("Get Project Relationship IDs Query"_L1);
    query->setQueryFunction([this, projectId, relationship]() -> QList<int> {
        std::unique_ptr<IProjectUnitOfWork> uow = std::make_unique<ProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsUseCase>(std::move(uow));
        return useCase->execute(projectId, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<void> ProjectController::setRelationshipIds(int projectId, ProjectRelationshipField relationship,
                                                        QList<int> relatedIds)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return;
    }

    // Create use case that will be owned by the command
    std::unique_ptr<IProjectUnitOfWork> uow = std::make_unique<ProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<SetRelationshipIdsUseCase>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Set Project Relationship IDs Command"_L1);

    // Create weak_ptr to break circular reference
    std::weak_ptr<SetRelationshipIdsUseCase> weakUseCase = useCase;

    // Prepare lambda for execute - use weak_ptr to avoid circular reference
    command->setExecuteFunction([weakUseCase, projectId, relationship, relatedIds](auto &) {
        if (auto useCase = weakUseCase.lock())
        {
            useCase->execute(projectId, relationship, relatedIds);
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
        co_await m_undoRedoSystem->executeCommandAsync(command, 500, "project_set_relationship"_L1);

    if (!success.has_value())
    {
        qWarning() << "Set project relationship command execution timed out";
        co_return;
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute set project relationship command";
        co_return;
    }
}

QCoro::Task<QHash<int, QList<int>>> ProjectController::getRelationshipIdsMany(const QList<int> &projectIds,
                                                                              ProjectRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QHash<int, QList<int>>();
    }

    auto query = m_undoRedoSystem->createQuery<QHash<int, QList<int>>>("Get Project Relationship IDs Many Query"_L1);
    query->setQueryFunction([this, projectIds, relationship]() -> QHash<int, QList<int>> {
        std::unique_ptr<IProjectUnitOfWork> uow = std::make_unique<ProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsManyUseCase>(std::move(uow));
        return useCase->execute(projectIds, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<int> ProjectController::getRelationshipIdsCount(int projectId, ProjectRelationshipField relationship)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return 0;
    }

    auto query = m_undoRedoSystem->createQuery<int>("Get Project Relationship IDs Count Query"_L1);
    query->setQueryFunction([this, projectId, relationship]() -> int {
        std::unique_ptr<IProjectUnitOfWork> uow = std::make_unique<ProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsCountUseCase>(std::move(uow));
        return useCase->execute(projectId, relationship);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}

QCoro::Task<QList<int>> ProjectController::getRelationshipIdsInRange(int projectId,
                                                                     ProjectRelationshipField relationship, int offset,
                                                                     int limit)
{
    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return QList<int>();
    }

    auto query = m_undoRedoSystem->createQuery<QList<int>>("Get Project Relationship IDs In Range Query"_L1);
    query->setQueryFunction([this, projectId, relationship, offset, limit]() -> QList<int> {
        std::unique_ptr<IProjectUnitOfWork> uow = std::make_unique<ProjectUnitOfWork>(*m_dbContext, m_eventRegistry);
        auto useCase = std::make_unique<GetRelationshipIdsInRangeUseCase>(std::move(uow));
        return useCase->execute(projectId, relationship, offset, limit);
    });

    auto result = co_await m_undoRedoSystem->executeQueryAsync(query);
    co_return result;
}
} // namespace Skribisto::DirectAccess::Project
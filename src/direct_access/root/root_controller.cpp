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
#include "use_cases/create_uc.h"
#include "use_cases/get_uc.h"
#include "use_cases/remove_uc.h"
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
    // use case will live as long as the command lives thanks to shared_ptr ownership in the command lambdas
    // this is important for undo/redo to work correctly

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Create Roots Command"_L1);
    QList<RootDto> result;

    // Prepare lambda for execute
    command->setExecuteFunction([useCase, roots, &result](auto &) { result = useCase->execute(roots); });
    // Prepare lambda for redo
    command->setRedoFunction([useCase]() { return useCase->redo(); });
    // Prepare lambda for undo
    command->setUndoFunction([useCase]() -> Common::UndoRedo::Result<void> { return useCase->undo(); });

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
    // use case will live as long as the command lives thanks to shared_ptr ownership in the command lambdas
    // this is important for undo/redo to work correctly

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Update Roots Command"_L1);
    QList<RootDto> result;

    // Prepare lambda for execute
    command->setExecuteFunction([useCase, roots, &result](auto &) { result = useCase->execute(roots); });
    // Prepare lambda for redo
    command->setRedoFunction([useCase]() { return useCase->redo(); });
    // Prepare lambda for undo
    command->setUndoFunction([useCase]() -> Common::UndoRedo::Result<void> { return useCase->undo(); });

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
    // use case will live as long as the command lives thanks to shared_ptr ownership in the command lambdas
    // this is important for undo/redo to work correctly

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Remove Roots Command"_L1);
    QList<int> result;

    // Prepare lambda for execute
    command->setExecuteFunction([useCase, rootIds, &result](auto &) { result = useCase->execute(rootIds); });
    // Prepare lambda for redo
    command->setRedoFunction([useCase]() { return useCase->redo(); });
    // Prepare lambda for undo
    command->setUndoFunction([useCase]() -> Common::UndoRedo::Result<void> { return useCase->undo(); });

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
// QCoro::Task<QList<int>> RootController::getRelationship(int rootId, Common::DirectAccess::Root::RootRelationshipField
// relationship)
// {
// }
// QCoro::Task<void> RootController::setRelationship(int rootId, Common::DirectAccess::Root::RootRelationshipField
// relationship,
//                                      QList<int> relatedIds)
// {
// }
} // namespace Skribisto::DirectAccess::Root
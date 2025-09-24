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

#include "work_management_controller.h"

#include "service_locator.h"
#include "units_of_work/load_work_unit_of_work.h"
#include "use_cases/load_work.h"

#include <QCoro/QCoroTask>

#include <memory>

namespace Skribisto::WorkManagement
{
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;

WorkManagementController::WorkManagementController(QObject *parent) : QObject(parent)
{
    resolveDependencies();
}
QCoro::Task<bool> WorkManagementController::loadWork(const LoadWorkDto &loadWorkDto)
{

    if (!m_undoRedoSystem)
    {
        qCritical() << "UndoRedo system not available";
        co_return false;
    }
    // Create use case that will be owned by the command
    std::unique_ptr<ILoadWorkUnitOfWork> uow = std::make_unique<LoadWorkUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_shared<LoadWork>(std::move(uow));

    // Create command that owns the use case
    auto command = std::make_shared<Common::UndoRedo::UndoRedoCommand>("Load Work Command"_L1);

    bool result = false;

    command->setExecuteFunction([useCase, &result, loadWorkDto](auto &) { result = useCase->execute(loadWorkDto); });

    std::optional<bool> success = co_await m_undoRedoSystem->executeCommandAsync(command, 500, "load_work"_L1);

    if (!success.has_value())
    {
        qWarning() << "Load work command execution timed out";
        co_return false;
    }

    if (!success.value())
    {
        qWarning() << "Failed to execute load work command";
        co_return false;
    }

    co_return result;
}
void WorkManagementController::resolveDependencies()
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

} // namespace Skribisto::WorkManagement
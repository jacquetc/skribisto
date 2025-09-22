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
#include <QCoroTimer>

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
    // TODO: add undo redo
}

QCoro::Task<QList<RootDto>> RootController::create(const QList<CreateRootDto> &roots)
{
    // TODO: add undo redo support
    std::unique_ptr<IRootUnitOfWork> uow = std::make_unique<RootUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_unique<CreateRootUseCase>(std::move(uow));
    auto result = useCase->execute(roots);

    // placeholder for async
    co_await QCoro::sleepFor(std::chrono::milliseconds(100));

    co_return result;
}
QCoro::Task<QList<RootDto>> RootController::get(const QList<int> &rootIds)
{
    // TODO: add undo redo support, but for a query
    std::unique_ptr<IRootUnitOfWork> uow = std::make_unique<RootUnitOfWork>(*m_dbContext, m_eventRegistry);
    auto useCase = std::make_unique<GetRootUseCase>(std::move(uow));
    auto result = useCase->execute(rootIds);

    // placeholder for async
    co_await QCoro::sleepFor(std::chrono::milliseconds(100));

    co_return result;
}
// QList<RootDto> RootController::update(const QList<RootDto> &roots)
// {
// }
// QList<int> RootController::remove(const QList<int> &rootIds)
// {
// }
// QList<int> RootController::getRelationship(int rootId, Common::DirectAccess::Root::RootRelationshipField
// relationship)
// {
// }
// void RootController::setRelationship(int rootId, Common::DirectAccess::Root::RootRelationshipField relationship,
//                                      QList<int> relatedIds)
// {
// }
} // namespace Skribisto::DirectAccess::Root
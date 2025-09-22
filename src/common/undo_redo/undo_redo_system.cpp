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

#include "undo_redo_system.h"
#include <QCoro/QCoroSignal>

namespace Skribisto::Common::UndoRedo
{

UndoRedoSystem::UndoRedoSystem(QObject *parent)
    : QObject(parent), m_manager(std::make_unique<UndoRedoManager>(this)),
      m_queryHandler(std::make_unique<QueryHandler>(this))
{
    // Connect signals for forwarding
    connect(m_manager.get(), &UndoRedoManager::commandFinished, this,
            [this](bool success) { Q_EMIT commandExecuted(m_manager->currentScope().name(), success); });
    connect(m_queryHandler.get(), &QueryHandler::queryFinished, this, &UndoRedoSystem::onQueryFinished);
}

UndoRedoManager *UndoRedoSystem::manager() const
{
    return m_manager.get();
}

QueryHandler *UndoRedoSystem::queryHandler() const
{
    return m_queryHandler.get();
}

/** DO NOT USE THIS METHOD ! Use executeCommandAsync instead. This Undo Redo system is async by nature.
 *
 * @param command
 * @param scope
 */
void UndoRedoSystem::executeCommand(std::shared_ptr<UndoRedoCommand> command, const QString &scope)
{
    auto undoRedoScope = UndoRedoScope::customScope(scope);
    m_manager->pushCommand(command, undoRedoScope);
    m_manager->execute(undoRedoScope);
}

/** DO NOT USE THIS METHOD ! Use executeQueryAsync instead. This Undo Redo system is async by nature.
 *
 * @param query
 */
void UndoRedoSystem::executeQuery(std::shared_ptr<QueryBase> query)
{
    m_queryHandler->executeQuery(query);
}

QCoro::Task<std::optional<bool>> UndoRedoSystem::executeCommandAsync(std::shared_ptr<UndoRedoCommand> command,
                                                                     int millisecondsTimeout, const QString &scope)

{
    if (!command)
    {
        co_return false;
    }

    // ensure millisecondsTimeout range between 1 and 20000 ms
    if (millisecondsTimeout < 1)
    {
        millisecondsTimeout = 1;
    }
    else if (millisecondsTimeout > 20000)
    {
        millisecondsTimeout = 20000;
    }

    auto undoRedoScope = UndoRedoScope::customScope(scope);
    m_manager->pushCommand(command, undoRedoScope);
    m_manager->execute(undoRedoScope);

    // Wait for the specific command to finish using QCoro
    auto success =
        co_await qCoro(command.get(), &UndoRedoCommand::finished, std::chrono::milliseconds(millisecondsTimeout));
    if (!success)
    {
        qWarning() << "Timeout reached while waiting for command to finish:" << command->text();

        qDebug() << "The command could not be completed in the allotted time.";
    }

    co_return success;
}

void UndoRedoSystem::onCommandFinished(const QString &scope, bool success)
{
    Q_EMIT commandExecuted(scope, success);
}

void UndoRedoSystem::onQueryFinished(std::shared_ptr<QueryBase> query, bool success)
{
    Q_EMIT queryExecuted(query, success);
}

} // namespace Skribisto::Common::UndoRedo

#include "undo_redo_system.moc"
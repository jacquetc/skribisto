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
#include <QElapsedTimer>
#include <QScopeGuard>
#include <QThread>
#include <atomic>

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

void UndoRedoSystem::shutdown()
{
    qDebug() << "UndoRedoSystem: Starting graceful shutdown. Active operations:" << m_activeOperations.load();

    // Step 1: Block all new queries and commands
    m_isShuttingDown = true;

    // Step 2: Clear all undo/redo stacks to release stored commands and their database connections
    if (m_manager)
    {
        m_manager->clearAllScopes();
        qDebug() << "UndoRedoSystem: Cleared all undo/redo stacks";
    }

    // Step 3: Wait for active operations to complete (if any)
    if (m_activeOperations.load() == 0)
    {
        qDebug() << "UndoRedoSystem: No active operations, shutdown complete.";
        return;
    }

    // Wait for existing operations with detailed progress
    const int maxWaitTime = 2000;
    const int pollInterval = 100;
    int elapsed = 0;
    int lastOperationCount = m_activeOperations.load();

    qDebug() << "UndoRedoSystem: Waiting for" << lastOperationCount << "operations to complete...";

    while (m_activeOperations.load() > 0 && elapsed < maxWaitTime)
    {
        QThread::msleep(pollInterval);
        elapsed += pollInterval;

        int currentCount = m_activeOperations.load();
        if (currentCount != lastOperationCount)
        {
            qDebug() << "UndoRedoSystem: Operations remaining:" << currentCount << "("
                     << (lastOperationCount - currentCount) << "completed)";
            lastOperationCount = currentCount;
        }
    }

    if (m_activeOperations.load() > 0)
    {
        qWarning() << "UndoRedoSystem: Timeout after" << elapsed << "ms. Forcefully cancelling"
                   << m_activeOperations.load() << "remaining operations.";

        // Brutal cancellation
        if (m_queryHandler)
        {
            m_queryHandler->cancelAllQueries();
        }
        if (m_manager)
        {
            m_manager->cancelAllCommands();
        }
    }
    else
    {
        qDebug() << "UndoRedoSystem: All operations completed gracefully in" << elapsed << "ms.";
    }
}

QCoro::Task<std::optional<bool>> UndoRedoSystem::executeCommandAsync(std::shared_ptr<UndoRedoCommand> command,
                                                                     int millisecondsTimeout, const QString &scope)

{
    // Check if shutting down - block new commands
    if (m_isShuttingDown.load())
    {
        qDebug() << "UndoRedoSystem: Rejecting new command during shutdown:" << command->text();
        co_return false;
    }

    if (!command)
    {
        co_return false;
    }

    // Increment active operations counter
    ++m_activeOperations;

    // Ensure we decrement the counter when done
    auto decrementOnExit = qScopeGuard([this]() { --m_activeOperations; });

    // ... rest of the existing method implementation stays the same until the end

    // ensure millisecondsTimeout range between 1 and 20000 ms
    if (millisecondsTimeout < 1)
    {
        millisecondsTimeout = 1;
    }
    else if (millisecondsTimeout > 20000)
    {
        millisecondsTimeout = 20000;
    }

    // Start timing for performance monitoring
    QElapsedTimer timer;
    timer.start();

    auto undoRedoScope = UndoRedoScope::customScope(scope);
    m_manager->pushCommand(command, undoRedoScope);

    // Emit stack size change signal
    Q_EMIT stackSizeChanged(scope, m_manager->undoCount(undoRedoScope), m_manager->redoCount(undoRedoScope));

    m_manager->execute(undoRedoScope);

    // Wait for the specific command to finish using QCoro
    auto success =
        co_await qCoro(command.get(), &UndoRedoCommand::finished, std::chrono::milliseconds(millisecondsTimeout));

    // Emit execution time signal
    qint64 executionTime = timer.elapsed();
    Q_EMIT commandExecutionTime(command->text(), executionTime);

    if (!success)
    {
        qWarning() << "Timeout reached while waiting for command to finish:" << command->text();
        qDebug() << "The command could not be completed in the allotted time.";
    }

    // decrementOnExit will automatically decrement m_activeOperations
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

void UndoRedoSystem::onCommandFinishedWithResult(const QString &scope, const Result<void> &result)
{
    Q_EMIT commandExecutedWithResult(scope, result);
    
    if (!result.isSuccess())
    {
        Q_EMIT commandErrorOccurred(scope, result.error(), result.category(), result.severity());
    }
}

} // namespace Skribisto::Common::UndoRedo
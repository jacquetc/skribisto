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

#pragma once

#include "query_handler.h"
#include "undo_redo_manager.h"
#include "undo_redo_command.h"
#include <QCoro/QCoroSignal>
#include <QCoro/QCoroTask>
#include <QObject>
#include <memory>

using namespace Qt::StringLiterals;

namespace Skribisto::Common::UndoRedo
{

class UndoRedoSystem : public QObject
{
    Q_OBJECT

  public:
    explicit UndoRedoSystem(QObject *parent = nullptr);

    // Manager access
    UndoRedoManager *manager() const;

    // Query handler access
    QueryHandler *queryHandler() const;

    // Convenience methods for command operations
    void executeCommand(std::shared_ptr<UndoRedoCommand> command, const QString &scope = "default"_L1);
    QCoro::Task<std::optional<bool>> executeCommandAsync(std::shared_ptr<UndoRedoCommand> command,
                                                         int millisecondsTimeout = 500,
                                                         const QString &scope = "default"_L1);

    // Convenience methods for query operations
    template <typename T> std::shared_ptr<Query<T>> createQuery(const QString &description);

    void executeQuery(std::shared_ptr<QueryBase> query);

    template <typename T> QCoro::Task<T> executeQueryAsync(std::shared_ptr<Query<T>> query);

    // Stack size management (affects current scope)
    void setMaxStackSize(int maxSize);
    int maxStackSize() const;
    void setAutoCleanupEnabled(bool enabled);
    bool isAutoCleanupEnabled() const;

    // Shutdown method to cancel all pending operations
    void shutdown();

  Q_SIGNALS:
    void commandExecuted(const QString &scope, bool success);
    void queryExecuted(std::shared_ptr<QueryBase> query, bool success);
    
    // Enhanced signals with detailed error information
    void commandExecutedWithResult(const QString &scope, const Result<void> &result);
    void commandErrorOccurred(const QString &scope, const QString &error, ErrorCategory category, ErrorSeverity severity);

    // Performance monitoring signals
    void commandExecutionTime(const QString &commandName, qint64 milliseconds);
    void stackSizeChanged(const QString &scope, int undoCount, int redoCount);

  private Q_SLOTS:
    void onCommandFinished(const QString &scope, bool success);
    void onCommandFinishedWithResult(const QString &scope, const Result<void> &result);
    void onQueryFinished(std::shared_ptr<QueryBase> query, bool success);

  private:
    std::unique_ptr<UndoRedoManager> m_manager;
    std::unique_ptr<QueryHandler> m_queryHandler;
    std::atomic<bool> m_isShuttingDown{false};
    std::atomic<int> m_activeOperations{0};
};
// Template implementation
template <typename T> std::shared_ptr<Query<T>> UndoRedoSystem::createQuery(const QString &description)
{
    return m_queryHandler->createQuery<T>(description);
}

template <typename T> QCoro::Task<T> UndoRedoSystem::executeQueryAsync(std::shared_ptr<Query<T>> query)
{
    // Check if shutting down - block new queries
    if (m_isShuttingDown.load())
    {
        qDebug() << "UndoRedoSystem: Rejecting new query during shutdown:" << query->description();
        co_return T{};
    }

    if (!query)
    {
        co_return T{};
    }

    // Increment active operations counter
    ++m_activeOperations;
    // Ensure we decrement the counter when done
    auto decrementOnExit = qScopeGuard([this]() { --m_activeOperations; });

    // Execute query asynchronously
    m_queryHandler->executeQuery(query);

    // Wait for query finished signal using QCoro
    co_await qCoro(m_queryHandler.get(), &QueryHandler::queryFinished);

    // decrementOnExit will automatically decrement m_activeOperations
    co_return query->typedResult();
}

} // namespace Skribisto::Common::UndoRedo
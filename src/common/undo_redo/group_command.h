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

#include "undo_redo_command.h"
#include <QList>
#include <QObject>
#include <memory>

using namespace Qt::StringLiterals;

namespace Skribisto::Common::UndoRedo
{

enum class FailureStrategy
{
    ContinueOnFailure,  // Continue executing remaining commands even if one fails
    StopOnFailure,      // Stop execution on first failure (default behavior)
    RollbackAll,        // Rollback all successfully executed commands on any failure
    RollbackPartial     // Rollback only the partially executed commands
};

class GroupCommand : public UndoRedoCommand
{
    Q_OBJECT

  public:
    explicit GroupCommand(const QString &text, QObject *parent = nullptr);

    void addCommand(std::shared_ptr<UndoRedoCommand> command);
    void insertCommand(int index, std::shared_ptr<UndoRedoCommand> command);
    void removeCommand(int index);
    void clearCommands();

    int commandCount() const;
    std::shared_ptr<UndoRedoCommand> command(int index) const;
    QList<std::shared_ptr<UndoRedoCommand>> commands() const;

    // Failure strategy
    void setFailureStrategy(FailureStrategy strategy);
    FailureStrategy failureStrategy() const;

    // Override base class methods
    void asyncExecute() override;
    void asyncUndo() override;
    void asyncRedo() override;

  private Q_SLOTS:
    void onChildCommandFinished(bool success);

  private:
    enum class ExecutionState
    {
        Idle,
        Executing,
        Undoing,
        Redoing
    };

    void executeNextExecuteCommand();
    void executeNextRedoCommand();
    void executeNextUndoCommand();
    void finishExecution(bool success);
    void handleFailureCleanup();
    void rollbackSuccessfulCommands();
    void rollbackPartialCommands();

    QList<std::shared_ptr<UndoRedoCommand>> m_commands;
    int m_currentCommandIndex;
    bool m_executionInProgress;
    int m_successfulCommands;
    ExecutionState m_executionState = ExecutionState::Idle;
    FailureStrategy m_failureStrategy = FailureStrategy::StopOnFailure;
};

} // namespace Skribisto::Common::UndoRedo
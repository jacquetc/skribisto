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

#include "group_command.h"
#include <QDebug>

namespace Skribisto::Common::UndoRedo
{

GroupCommand::GroupCommand(const QString &text, QObject *parent)
    : UndoRedoCommand(text, parent), m_currentCommandIndex(-1), m_executionInProgress(false), m_successfulCommands(0)
{
}

void GroupCommand::addCommand(std::shared_ptr<UndoRedoCommand> command)
{
    if (command && !m_executionInProgress)
    {
        m_commands.append(command);
    }
}

void GroupCommand::insertCommand(int index, std::shared_ptr<UndoRedoCommand> command)
{
    if (command && !m_executionInProgress && index >= 0 && index <= m_commands.size())
    {
        m_commands.insert(index, command);
    }
}

void GroupCommand::removeCommand(int index)
{
    if (!m_executionInProgress && index >= 0 && index < m_commands.size())
    {
        m_commands.removeAt(index);
    }
}

void GroupCommand::clearCommands()
{
    if (!m_executionInProgress)
    {
        m_commands.clear();
    }
}

int GroupCommand::commandCount() const
{
    return m_commands.size();
}

std::shared_ptr<UndoRedoCommand> GroupCommand::command(int index) const
{
    if (index >= 0 && index < m_commands.size())
    {
        return m_commands.at(index);
    }
    return nullptr;
}

QList<std::shared_ptr<UndoRedoCommand>> GroupCommand::commands() const
{
    return m_commands;
}
void GroupCommand::asyncExecute()
{
    m_executionState = ExecutionState::Executing;
    m_executionInProgress = true;
    m_currentCommandIndex = 0; // Start from first command for execute
    m_successfulCommands = 0;

    executeNextExecuteCommand();
}

void GroupCommand::asyncUndo()
{
    if (m_executionInProgress || m_commands.isEmpty())
    {
        Q_EMIT finished(m_commands.isEmpty());
        return;
    }

    m_executionState = ExecutionState::Undoing;
    m_executionInProgress = true;
    m_currentCommandIndex = m_commands.size() - 1; // Start from last command for undo
    m_successfulCommands = 0;

    executeNextUndoCommand();
}

void GroupCommand::asyncRedo()
{
    if (m_executionInProgress || m_commands.isEmpty())
    {
        Q_EMIT finished(m_commands.isEmpty());
        return;
    }

    m_executionState = ExecutionState::Redoing;
    m_executionInProgress = true;
    m_currentCommandIndex = 0; // Start from first command for redo
    m_successfulCommands = 0;

    executeNextRedoCommand();
}

void GroupCommand::onChildCommandFinished(bool success)
{
    // Disconnect from the finished command
    auto *command = qobject_cast<UndoRedoCommand *>(sender());
    if (command)
    {
        disconnect(command, &UndoRedoCommand::finished, this, &GroupCommand::onChildCommandFinished);
    }

    if (success)
    {
        m_successfulCommands++;
    }
    switch (m_executionState)
    {
    case ExecutionState::Executing:
        if (success && m_currentCommandIndex < m_commands.size() - 1)
        {
            m_currentCommandIndex++;
            executeNextExecuteCommand();
        }
        else
        {
            // Finished or failed
            finishExecution(success);
        }

        break;
    case ExecutionState::Undoing:
        if (success && m_currentCommandIndex > 0)
        {
            m_currentCommandIndex--;
            executeNextUndoCommand();
        }
        else
        {
            // Finished or failed
            finishExecution(success);
        }

        break;
    case ExecutionState::Redoing:

        if (success && m_currentCommandIndex < m_commands.size() - 1)
        {
            m_currentCommandIndex++;
            executeNextRedoCommand();
        }
        else
        {
            // Finished or failed
            finishExecution(success);
        }
        break;
    default:
        qCritical() << "GroupCommand::onChildCommandFinished: Unknown execution state.";
        break;
    }
}

void GroupCommand::executeNextExecuteCommand()
{
    if (m_currentCommandIndex >= 0 && m_currentCommandIndex < m_commands.size())
    {
        auto command = m_commands.at(m_currentCommandIndex);
        connect(command.get(), &UndoRedoCommand::finished, this, &GroupCommand::onChildCommandFinished,
                Qt::UniqueConnection);
        command->asyncExecute();
    }
    else
    {
        finishExecution(false);
    }
}

void GroupCommand::executeNextRedoCommand()
{
    if (m_currentCommandIndex >= 0 && m_currentCommandIndex < m_commands.size())
    {
        auto command = m_commands.at(m_currentCommandIndex);
        connect(command.get(), &UndoRedoCommand::finished, this, &GroupCommand::onChildCommandFinished,
                Qt::UniqueConnection);
        command->asyncRedo();
    }
    else
    {
        finishExecution(false);
    }
}

void GroupCommand::executeNextUndoCommand()
{
    if (m_currentCommandIndex >= 0 && m_currentCommandIndex < m_commands.size())
    {
        auto command = m_commands.at(m_currentCommandIndex);
        connect(command.get(), &UndoRedoCommand::finished, this, &GroupCommand::onChildCommandFinished,
                Qt::UniqueConnection);
        command->asyncUndo();
    }
    else
    {
        finishExecution(false);
    }
}

void GroupCommand::finishExecution(bool success)
{
    m_executionInProgress = false;
    m_currentCommandIndex = -1;

    // Consider success if we executed all commands successfully
    bool allSuccess = success && (m_successfulCommands == m_commands.size());

    Q_EMIT finished(allSuccess);
}

} // namespace Skribisto::Common::UndoRedo

#include "group_command.moc"
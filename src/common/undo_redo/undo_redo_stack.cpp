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

#include "undo_redo_stack.h"
#include <QMutexLocker>

namespace Skribisto::Common::UndoRedo
{

UndoRedoStack::UndoRedoStack(QObject *parent) : QObject(parent)
{
}

void UndoRedoStack::push(std::shared_ptr<UndoRedoCommand> command)
{
    if (!command)
    {
        return;
    }

    QMutexLocker locker(&m_mutex);

    // Clear redo stack when pushing a new command
    m_redoStack.clear();

    // Try to merge with the top command if possible
    if (!m_undoStack.isEmpty())
    {
        auto topCommand = m_undoStack.top();
        if (topCommand->canMergeWith(command))
        {
            topCommand->mergeWith(command);
            updateState(); // Update to reflect the merged command text
            return;
        }
    }

    // If merging is not possible, push the new command
    m_undoStack.push(command);

    updateState();
}

bool UndoRedoStack::canUndo() const
{
    QMutexLocker locker(&m_mutex);
    return !m_undoStack.isEmpty();
}

bool UndoRedoStack::canRedo() const
{
    QMutexLocker locker(&m_mutex);
    return !m_redoStack.isEmpty();
}
void UndoRedoStack::execute()
{
    QMutexLocker locker(&m_mutex);

    if (m_undoStack.isEmpty())
    {
        return;
    }

    auto command = m_undoStack.top();
    m_currentCommand = command;

    // Connect to command finished signal
    connect(command.get(), &UndoRedoCommand::finished, this, &UndoRedoStack::onCommandFinished, Qt::UniqueConnection);

    updateState();

    // Execute the command asynchronously (for newly pushed commands)
    command->asyncExecute();
}

void UndoRedoStack::undo()
{
    QMutexLocker locker(&m_mutex);

    if (m_undoStack.isEmpty())
    {
        return;
    }

    auto command = m_undoStack.pop();
    m_redoStack.push(command);
    m_currentCommand = command;

    // Connect to command finished signal
    connect(command.get(), &UndoRedoCommand::finished, this, &UndoRedoStack::onCommandFinished, Qt::UniqueConnection);

    updateState();

    // Execute undo asynchronously
    command->asyncUndo();
}

void UndoRedoStack::redo()
{
    QMutexLocker locker(&m_mutex);

    if (m_redoStack.isEmpty())
    {
        return;
    }

    auto command = m_redoStack.pop();
    m_undoStack.push(command);
    m_currentCommand = command;

    // Connect to command finished signal
    connect(command.get(), &UndoRedoCommand::finished, this, &UndoRedoStack::onCommandFinished, Qt::UniqueConnection);

    updateState();

    // Execute redo asynchronously
    command->asyncRedo();
}

void UndoRedoStack::clear()
{
    QMutexLocker locker(&m_mutex);

    m_undoStack.clear();
    m_redoStack.clear();
    m_currentCommand.reset();

    updateState();
}

int UndoRedoStack::undoCount() const
{
    QMutexLocker locker(&m_mutex);
    return m_undoStack.size();
}

int UndoRedoStack::redoCount() const
{
    QMutexLocker locker(&m_mutex);
    return m_redoStack.size();
}

QString UndoRedoStack::undoText() const
{
    QMutexLocker locker(&m_mutex);

    if (m_undoStack.isEmpty())
    {
        return QString();
    }

    return m_undoStack.top()->text();
}

QString UndoRedoStack::redoText() const
{
    QMutexLocker locker(&m_mutex);

    if (m_redoStack.isEmpty())
    {
        return QString();
    }

    return m_redoStack.top()->text();
}

void UndoRedoStack::onCommandFinished(bool success)
{
    m_currentCommand.reset();
    Q_EMIT commandFinished(success);
}

void UndoRedoStack::updateState()
{
    // This method assumes mutex is already locked
    Q_EMIT canUndoChanged(!m_undoStack.isEmpty());
    Q_EMIT canRedoChanged(!m_redoStack.isEmpty());

    QString undoText = m_undoStack.isEmpty() ? QString() : m_undoStack.top()->text();
    QString redoText = m_redoStack.isEmpty() ? QString() : m_redoStack.top()->text();

    Q_EMIT undoTextChanged(undoText);
    Q_EMIT redoTextChanged(redoText);
}

} // namespace Skribisto::Common::UndoRedo

#include "undo_redo_stack.moc"
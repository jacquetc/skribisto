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

#include "undo_redo_command.h"
#include <QDebug>
#include <QFutureWatcher>
#include <QtConcurrent/QtConcurrentRun>

namespace Skribisto::Common::UndoRedo
{

UndoRedoCommand::UndoRedoCommand(const QString &text, QObject *parent)
    : QObject(parent), m_text(text), m_executeWatcher(new QFutureWatcher<Result<void>>(this)),
      m_redoWatcher(new QFutureWatcher<Result<void>>(this)), m_undoWatcher(new QFutureWatcher<Result<void>>(this))
{
    connect(m_executeWatcher, &QFutureWatcher<Result<void>>::finished, this, &UndoRedoCommand::onExecuteFinished);
    connect(m_redoWatcher, &QFutureWatcher<Result<void>>::finished, this, &UndoRedoCommand::onRedoFinished);
    connect(m_undoWatcher, &QFutureWatcher<Result<void>>::finished, this, &UndoRedoCommand::onUndoFinished);
}

void UndoRedoCommand::setExecuteFunction(const std::function<void(QPromise<Result<void>> &promise)> &function)
{
    m_executeFunction = function;
}

void UndoRedoCommand::setUndoFunction(const std::function<Result<void>()> &function)
{
    m_undoFunction = function;
}

void UndoRedoCommand::setRedoFunction(const std::function<Result<void>()> &function)
{
    m_redoFunction = function;
}

void UndoRedoCommand::asyncExecute()
{
    if (m_wasAlreadyExecuted)
    {
        qCritical() << "Command already executed, calling redo instead.";
        asyncRedo();
        return;
    }

    if (m_executeFunction)
    {
        auto future = QtConcurrent::run([this]() {
            QPromise<Result<void>> promise;
            m_executeFunction(promise);
            return Result<void>(); // Success by default
        });
        m_executeWatcher->setFuture(future);
        m_wasAlreadyExecuted = true;
    }
    else
    {
        // No execute function, emit finished signal immediately
        m_wasAlreadyExecuted = true;
        Q_EMIT finished(true);
    }
}

void UndoRedoCommand::asyncUndo()
{
    if (!m_wasAlreadyExecuted)
    {
        qCritical() << "Command not yet executed, cannot undo.";
        return;
    }

    if (m_undoFunction)
    {
        auto future = QtConcurrent::run([this]() { return m_undoFunction(); });
        m_undoWatcher->setFuture(future);
    }
    else
    {
        // No undo function, emit finished signal immediately
        Q_EMIT finished(true);
    }
}

void UndoRedoCommand::asyncRedo()
{
    if (!m_wasAlreadyExecuted)
    {
        qCritical() << "Command not yet executed, calling execute instead.";
        asyncExecute();
        return;
    }

    if (m_redoFunction)
    {
        auto future = QtConcurrent::run([this]() { return m_redoFunction(); });
        m_redoWatcher->setFuture(future);
    }
    else
    {
        // No redo function, emit finished signal immediately
        Q_EMIT finished(true);
    }
}

QString UndoRedoCommand::text() const
{
    return m_text;
}

void UndoRedoCommand::setText(const QString &newText)
{
    m_text = newText;
}

bool UndoRedoCommand::canMergeWith(const std::shared_ptr<UndoRedoCommand> &other) const
{
    Q_UNUSED(other)
    return false; // Default implementation: no merging
}

void UndoRedoCommand::mergeWith(const std::shared_ptr<UndoRedoCommand> &other)
{
    Q_UNUSED(other)
    // Default implementation: do nothing
}

void UndoRedoCommand::onExecuteFinished()
{
    if (m_executeWatcher->isFinished())
    {
        auto result = m_executeWatcher->result();
        Q_EMIT finished(result.isSuccess());
    }
}

void UndoRedoCommand::onRedoFinished()
{
    if (m_redoWatcher->isFinished())
    {
        auto result = m_redoWatcher->result();
        Q_EMIT finished(result.isSuccess());
    }
}

void UndoRedoCommand::onUndoFinished()
{
    if (m_undoWatcher->isFinished())
    {
        auto result = m_undoWatcher->result();
        Q_EMIT finished(result.isSuccess());
    }
}

} // namespace Skribisto::Common::UndoRedo

#include "undo_redo_command.moc"
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

#include "undo_redo_manager.h"
#include <QMutexLocker>

namespace Skribisto::Common::UndoRedo
{

UndoRedoManager::UndoRedoManager(QObject *parent) : QObject(parent), m_currentScope(UndoRedoScope::rootScope())
{
}

void UndoRedoManager::setCurrentScope(const UndoRedoScope &scope)
{
    QMutexLocker locker(&m_mutex);

    if (m_currentScope == scope)
    {
        return;
    }

    m_currentScope = scope;
    updateCurrentScopeSignals();
    Q_EMIT currentScopeChanged(scope);
}

UndoRedoScope UndoRedoManager::currentScope() const
{
    QMutexLocker locker(&m_mutex);
    return m_currentScope;
}

void UndoRedoManager::pushCommand(std::shared_ptr<UndoRedoCommand> command)
{
    QMutexLocker locker(&m_mutex);
    if (!command)
    {
        return;
    }

    auto *stack = getOrCreateStack(m_currentScope);
    stack->push(command);
}

void UndoRedoManager::pushCommand(std::shared_ptr<UndoRedoCommand> command, const UndoRedoScope &scope)
{
    if (!command)
    {
        return;
    }

    QMutexLocker locker(&m_mutex);
    auto *stack = getOrCreateStack(scope);
    stack->push(command);
}

bool UndoRedoManager::canUndo() const
{
    QMutexLocker locker(&m_mutex);
    return canUndo(m_currentScope);
}

bool UndoRedoManager::canRedo() const
{
    QMutexLocker locker(&m_mutex);
    return canRedo(m_currentScope);
}

void UndoRedoManager::execute()
{
    QMutexLocker locker(&m_mutex);
    execute(m_currentScope);
}

void UndoRedoManager::undo()
{
    QMutexLocker locker(&m_mutex);
    undo(m_currentScope);
}

void UndoRedoManager::redo()
{
    QMutexLocker locker(&m_mutex);
    redo(m_currentScope);
}

QString UndoRedoManager::undoText() const
{
    QMutexLocker locker(&m_mutex);
    return undoText(m_currentScope);
}

QString UndoRedoManager::redoText() const
{
    QMutexLocker locker(&m_mutex);
    return redoText(m_currentScope);
}

bool UndoRedoManager::canUndo(const UndoRedoScope &scope) const
{
    QMutexLocker locker(&m_mutex);
    auto it = m_stacks.find(scope);
    return it != m_stacks.end() ? it.value()->canUndo() : false;
}

bool UndoRedoManager::canRedo(const UndoRedoScope &scope) const
{
    QMutexLocker locker(&m_mutex);
    auto it = m_stacks.find(scope);
    return it != m_stacks.end() ? it.value()->canRedo() : false;
}

void UndoRedoManager::execute(const UndoRedoScope &scope)
{
    QMutexLocker locker(&m_mutex);
    auto it = m_stacks.find(scope);
    if (it != m_stacks.end())
    {
        it.value()->execute();
    }
}

void UndoRedoManager::undo(const UndoRedoScope &scope)
{
    QMutexLocker locker(&m_mutex);
    auto it = m_stacks.find(scope);
    if (it != m_stacks.end())
    {
        it.value()->undo();
    }
}

void UndoRedoManager::redo(const UndoRedoScope &scope)
{
    QMutexLocker locker(&m_mutex);
    auto it = m_stacks.find(scope);
    if (it != m_stacks.end())
    {
        it.value()->redo();
    }
}

QString UndoRedoManager::undoText(const UndoRedoScope &scope) const
{
    QMutexLocker locker(&m_mutex);
    auto it = m_stacks.find(scope);
    return it != m_stacks.end() ? it.value()->undoText() : QString();
}

QString UndoRedoManager::redoText(const UndoRedoScope &scope) const
{
    QMutexLocker locker(&m_mutex);
    auto it = m_stacks.find(scope);
    return it != m_stacks.end() ? it.value()->redoText() : QString();
}

void UndoRedoManager::clearScope(const UndoRedoScope &scope)
{
    QMutexLocker locker(&m_mutex);
    auto it = m_stacks.find(scope);
    if (it != m_stacks.end())
    {
        it.value()->clear();
        m_stacks.erase(it);

        if (scope == m_currentScope)
        {
            updateCurrentScopeSignals();
        }
    }
}

void UndoRedoManager::clearAllScopes()
{
    QMutexLocker locker(&m_mutex);
    m_stacks.clear();
    updateCurrentScopeSignals();
}

QList<UndoRedoScope> UndoRedoManager::activeScopes() const
{
    QMutexLocker locker(&m_mutex);
    return m_stacks.keys();
}

int UndoRedoManager::undoCount(const UndoRedoScope &scope) const
{
    QMutexLocker locker(&m_mutex);
    auto it = m_stacks.find(scope);
    return it != m_stacks.end() ? it.value()->undoCount() : 0;
}

int UndoRedoManager::redoCount(const UndoRedoScope &scope) const
{
    QMutexLocker locker(&m_mutex);
    auto it = m_stacks.find(scope);
    return it != m_stacks.end() ? it.value()->redoCount() : 0;
}

void UndoRedoManager::onStackCanUndoChanged(bool canUndo)
{
    auto *stack = qobject_cast<UndoRedoStack *>(sender());
    if (!stack)
        return;

    QMutexLocker locker(&m_mutex);
    for (auto it = m_stacks.begin(); it != m_stacks.end(); ++it)
    {
        if (it.value().get() == stack && it.key() == m_currentScope)
        {
            Q_EMIT canUndoChanged(canUndo);
            break;
        }
    }
}

void UndoRedoManager::onStackCanRedoChanged(bool canRedo)
{
    auto *stack = qobject_cast<UndoRedoStack *>(sender());
    if (!stack)
        return;

    QMutexLocker locker(&m_mutex);
    for (auto it = m_stacks.begin(); it != m_stacks.end(); ++it)
    {
        if (it.value().get() == stack && it.key() == m_currentScope)
        {
            Q_EMIT canRedoChanged(canRedo);
            break;
        }
    }
}

void UndoRedoManager::onStackUndoTextChanged(const QString &undoText)
{
    auto *stack = qobject_cast<UndoRedoStack *>(sender());
    if (!stack)
        return;

    QMutexLocker locker(&m_mutex);
    for (auto it = m_stacks.begin(); it != m_stacks.end(); ++it)
    {
        if (it.value().get() == stack && it.key() == m_currentScope)
        {
            Q_EMIT undoTextChanged(undoText);
            break;
        }
    }
}

void UndoRedoManager::onStackRedoTextChanged(const QString &redoText)
{
    auto *stack = qobject_cast<UndoRedoStack *>(sender());
    if (!stack)
        return;

    QMutexLocker locker(&m_mutex);
    for (auto it = m_stacks.begin(); it != m_stacks.end(); ++it)
    {
        if (it.value().get() == stack && it.key() == m_currentScope)
        {
            Q_EMIT redoTextChanged(redoText);
            break;
        }
    }
}

void UndoRedoManager::onStackCommandFinished(bool success)
{
    Q_EMIT commandFinished(success);
}

UndoRedoStack *UndoRedoManager::getOrCreateStack(const UndoRedoScope &scope)
{
    // Assumes mutex is already locked
    auto it = m_stacks.find(scope);
    if (it == m_stacks.end())
    {
        auto stack = std::make_shared<UndoRedoStack>(this);
        connectStackSignals(stack.get());
        m_stacks[scope] = stack;
        return stack.get();
    }
    return it.value().get();
}

void UndoRedoManager::connectStackSignals(UndoRedoStack *stack)
{
    connect(stack, &UndoRedoStack::canUndoChanged, this, &UndoRedoManager::onStackCanUndoChanged, Qt::QueuedConnection);
    connect(stack, &UndoRedoStack::canRedoChanged, this, &UndoRedoManager::onStackCanRedoChanged, Qt::QueuedConnection);
    connect(stack, &UndoRedoStack::undoTextChanged, this, &UndoRedoManager::onStackUndoTextChanged,
            Qt::QueuedConnection);
    connect(stack, &UndoRedoStack::redoTextChanged, this, &UndoRedoManager::onStackRedoTextChanged,
            Qt::QueuedConnection);
    connect(stack, &UndoRedoStack::commandFinished, this, &UndoRedoManager::onStackCommandFinished,
            Qt::QueuedConnection);
}

void UndoRedoManager::updateCurrentScopeSignals()
{
    // Assumes mutex is already locked
    auto it = m_stacks.find(m_currentScope);
    if (it != m_stacks.end())
    {
        Q_EMIT canUndoChanged(it.value()->canUndo());
        Q_EMIT canRedoChanged(it.value()->canRedo());
        Q_EMIT undoTextChanged(it.value()->undoText());
        Q_EMIT redoTextChanged(it.value()->redoText());
    }
    else
    {
        Q_EMIT canUndoChanged(false);
        Q_EMIT canRedoChanged(false);
        Q_EMIT undoTextChanged(QString());
        Q_EMIT redoTextChanged(QString());
    }
}

} // namespace Skribisto::Common::UndoRedo

#include "undo_redo_manager.moc"
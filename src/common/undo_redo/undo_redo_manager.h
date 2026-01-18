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

#include "undo_redo_stack.h"
#include <QHash>
#include <QMutex>
#include <QObject>
#include <memory>

using namespace Qt::StringLiterals;

namespace Skribisto::Common::UndoRedo
{

class UndoRedoManager : public QObject
{
    Q_OBJECT

  public:
    explicit UndoRedoManager(QObject *parent = nullptr);

    // Stack management
    int createStack();
    void removeStack(int stackId);
    void setCurrentStackId(int stackId);
    int currentStackId() const;

    // Command operations
    void pushCommand(std::shared_ptr<UndoRedoCommand> command);
    void pushCommand(std::shared_ptr<UndoRedoCommand> command, int stackId);

    // Undo/Redo operations for current stack
    bool canUndo() const;
    bool canRedo() const;
    void execute();
    void undo();
    void redo();
    QString undoText() const;
    QString redoText() const;

    // Undo/Redo operations for specific stack
    bool canUndo(int stackId) const;
    bool canRedo(int stackId) const;
    void execute(int stackId);
    void undo(int stackId);
    void redo(int stackId);
    QString undoText(int stackId) const;
    QString redoText(int stackId) const;

    // Stack management
    void clearStack(int stackId);
    void clearAllStacks();

    // Information
    QList<int> activeStackIds() const;
    int undoCount(int stackId) const;
    int redoCount(int stackId) const;

    // Stack size management for current stack
    void setMaxStackSize(int maxSize);
    int maxStackSize() const;
    void setAutoCleanupEnabled(bool enabled);
    bool isAutoCleanupEnabled() const;

    // Stack size management for specific stack
    void setMaxStackSize(int stackId, int maxSize);
    int maxStackSize(int stackId) const;
    void setAutoCleanupEnabled(int stackId, bool enabled);
    bool isAutoCleanupEnabled(int stackId) const;

    // Cancel all running commands in all stacks
    void cancelAllCommands();

  Q_SIGNALS:
    void currentStackIdChanged(int stackId);
    void canUndoChanged(bool canUndo);
    void canRedoChanged(bool canRedo);
    void undoTextChanged(const QString &undoText);
    void redoTextChanged(const QString &redoText);
    void commandFinished(bool success);

  private Q_SLOTS:
    void onStackCanUndoChanged(bool canUndo);
    void onStackCanRedoChanged(bool canRedo);
    void onStackUndoTextChanged(const QString &undoText);
    void onStackRedoTextChanged(const QString &redoText);
    void onStackCommandFinished(bool success);

  private:
    UndoRedoStack *getOrCreateStack(int stackId);
    void connectStackSignals(UndoRedoStack *stack);
    void updateCurrentStackSignals();

    mutable QRecursiveMutex m_mutex;
    QHash<int, std::shared_ptr<UndoRedoStack>> m_stacks;
    int m_currentStackId = 0; // Stack 0 is the global stack (default)
    int m_nextStackId = 1;    // Next available stack ID for createStack()
};

} // namespace Skribisto::Common::UndoRedo

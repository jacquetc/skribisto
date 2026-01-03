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

#include "undo_redo_scopes.h"
#include "undo_redo_stack.h"
#include <QHash>
#include <QMutex>
#include <QObject>
#include <memory>
#include <pstl/glue_execution_defs.h>

using namespace Qt::StringLiterals;

namespace Skribisto::Common::UndoRedo
{

class UndoRedoManager : public QObject
{
    Q_OBJECT

  public:
    explicit UndoRedoManager(QObject *parent = nullptr);

    // Scope management
    void setCurrentScope(const UndoRedoScope &scope);
    UndoRedoScope currentScope() const;

    // Command operations
    void pushCommand(std::shared_ptr<UndoRedoCommand> command);
    void pushCommand(std::shared_ptr<UndoRedoCommand> command, const UndoRedoScope &scope);

    // Undo/Redo operations for current scope
    bool canUndo() const;
    bool canRedo() const;
    void execute();
    void undo();
    void redo();
    QString undoText() const;
    QString redoText() const;

    // Undo/Redo operations for specific scope
    bool canUndo(const UndoRedoScope &scope) const;
    bool canRedo(const UndoRedoScope &scope) const;
    void execute(const UndoRedoScope &scope);
    void undo(const UndoRedoScope &scope);
    void redo(const UndoRedoScope &scope);
    QString undoText(const UndoRedoScope &scope) const;
    QString redoText(const UndoRedoScope &scope) const;

    // Stack management
    void clearScope(const UndoRedoScope &scope);
    void clearAllScopes();

    // Information
    QList<UndoRedoScope> activeScopes() const;
    int undoCount(const UndoRedoScope &scope) const;
    int redoCount(const UndoRedoScope &scope) const;

    // Stack size management for current scope
    void setMaxStackSize(int maxSize);
    int maxStackSize() const;
    void setAutoCleanupEnabled(bool enabled);
    bool isAutoCleanupEnabled() const;

    // Stack size management for specific scope
    void setMaxStackSize(const UndoRedoScope &scope, int maxSize);
    int maxStackSize(const UndoRedoScope &scope) const;
    void setAutoCleanupEnabled(const UndoRedoScope &scope, bool enabled);
    bool isAutoCleanupEnabled(const UndoRedoScope &scope) const;

    // Cancel all running commands in all stacks
    void cancelAllCommands();

  Q_SIGNALS:
    void currentScopeChanged(const UndoRedoScope &scope);
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
    UndoRedoStack *getOrCreateStack(const UndoRedoScope &scope);
    void connectStackSignals(UndoRedoStack *stack);
    void updateCurrentScopeSignals();

    mutable QRecursiveMutex m_mutex;
    QHash<UndoRedoScope, std::shared_ptr<UndoRedoStack>> m_stacks;
    UndoRedoScope m_currentScope;
};

} // namespace Skribisto::Common::UndoRedo
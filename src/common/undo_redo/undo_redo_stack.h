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
#include <QRecursiveMutex>
#include <QObject>
#include <QStack>
#include <memory>

using namespace Qt::StringLiterals;

namespace Skribisto::Common::UndoRedo
{

class UndoRedoStack : public QObject
{
    Q_OBJECT

  public:
    explicit UndoRedoStack(QObject *parent = nullptr);

    void push(std::shared_ptr<UndoRedoCommand> command);
    bool canUndo() const;
    bool canRedo() const;
    void execute();
    void undo();
    void redo();
    void clear();

    int undoCount() const;
    int redoCount() const;
    QString undoText() const;
    QString redoText() const;

    // Stack size management
    void setMaxStackSize(int maxSize);
    int maxStackSize() const;
    void setAutoCleanupEnabled(bool enabled);
    bool isAutoCleanupEnabled() const;

  Q_SIGNALS:
    void canUndoChanged(bool canUndo);
    void canRedoChanged(bool canRedo);
    void undoTextChanged(const QString &undoText);
    void redoTextChanged(const QString &redoText);
    void commandFinished(bool success);

  private Q_SLOTS:
    void onCommandFinished(bool success);

  private:
    void updateState();

    mutable QRecursiveMutex m_mutex;
    QStack<std::shared_ptr<UndoRedoCommand>> m_undoStack;
    QStack<std::shared_ptr<UndoRedoCommand>> m_redoStack;
    std::shared_ptr<UndoRedoCommand> m_currentCommand;
    int m_maxStackSize = -1; // -1 means unlimited
    bool m_autoCleanupEnabled = false;
    friend class UndoRedoManager;
};

} // namespace Skribisto::Common::UndoRedo
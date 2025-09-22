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

#include "group_command.h"
#include "undo_redo_command.h"
#include <QString>
#include <memory>

using namespace Qt::StringLiterals;

namespace Skribisto::Common::UndoRedo
{

class GroupCommandBuilder
{
public:
    explicit GroupCommandBuilder(const QString &text);
    
    // Fluent API methods
    GroupCommandBuilder& addCommand(std::shared_ptr<UndoRedoCommand> command);
    GroupCommandBuilder& insertCommand(int index, std::shared_ptr<UndoRedoCommand> command);
    GroupCommandBuilder& onFailure(FailureStrategy strategy);
    GroupCommandBuilder& setParent(QObject *parent);
    
    // Build method
    std::shared_ptr<GroupCommand> build();
    
private:
    QString m_text;
    QList<std::shared_ptr<UndoRedoCommand>> m_commands;
    FailureStrategy m_failureStrategy = FailureStrategy::StopOnFailure;
    QObject *m_parent = nullptr;
};

} // namespace Skribisto::Common::UndoRedo
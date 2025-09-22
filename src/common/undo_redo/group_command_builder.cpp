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

#include "group_command_builder.h"

namespace Skribisto::Common::UndoRedo
{

GroupCommandBuilder::GroupCommandBuilder(const QString &text)
    : m_text(text)
{
}

GroupCommandBuilder& GroupCommandBuilder::addCommand(std::shared_ptr<UndoRedoCommand> command)
{
    if (command) {
        m_commands.append(command);
    }
    return *this;
}

GroupCommandBuilder& GroupCommandBuilder::insertCommand(int index, std::shared_ptr<UndoRedoCommand> command)
{
    if (command && index >= 0 && index <= m_commands.size()) {
        m_commands.insert(index, command);
    }
    return *this;
}

GroupCommandBuilder& GroupCommandBuilder::onFailure(FailureStrategy strategy)
{
    m_failureStrategy = strategy;
    return *this;
}

GroupCommandBuilder& GroupCommandBuilder::setParent(QObject *parent)
{
    m_parent = parent;
    return *this;
}

std::shared_ptr<GroupCommand> GroupCommandBuilder::build()
{
    auto groupCommand = std::make_shared<GroupCommand>(m_text, m_parent);
    
    // Add all commands to the group
    for (const auto& command : m_commands) {
        groupCommand->addCommand(command);
    }
    
    // Set failure strategy (we'll need to extend GroupCommand to support this)
    groupCommand->setFailureStrategy(m_failureStrategy);
    
    return groupCommand;
}

} // namespace Skribisto::Common::UndoRedo
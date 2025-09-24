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

#include "undo_redo_scopes.h"

namespace Skribisto::Common::UndoRedo
{

UndoRedoScope::UndoRedoScope(UndoRedoScopeType type, const QString &name, int id) : m_type(type), m_name(name), m_id(id)
{
}

UndoRedoScopeType UndoRedoScope::type() const
{
    return m_type;
}

QString UndoRedoScope::name() const
{
    return m_name;
}

int UndoRedoScope::id() const
{
    return m_id;
}

QString UndoRedoScope::scopeKey() const
{
    QString typeStr;
    switch (m_type)
    {
    case UndoRedoScopeType::Root:
        typeStr = "Root"_L1;
        break;
    case UndoRedoScopeType::Work:
        typeStr = "Work"_L1;
        break;
    case UndoRedoScopeType::Content:
        typeStr = "Content"_L1;
        break;
    case UndoRedoScopeType::Settings:
        typeStr = "Settings"_L1;
        break;
    case UndoRedoScopeType::Custom:
        typeStr = "Custom"_L1;
        break;
    }

    if (m_id >= 0)
    {
        return QString("%1_%2"_L1).arg(typeStr, QString::number(m_id));
    }
    else if (!m_name.isEmpty())
    {
        return QString("%1_%2"_L1).arg(typeStr, m_name);
    }
    else
    {
        return typeStr;
    }
}

bool UndoRedoScope::operator==(const UndoRedoScope &other) const
{
    return m_type == other.m_type && m_name == other.m_name && m_id == other.m_id;
}

bool UndoRedoScope::operator!=(const UndoRedoScope &other) const
{
    return !(*this == other);
}

UndoRedoScope UndoRedoScope::rootScope()
{
    return UndoRedoScope(UndoRedoScopeType::Root);
}

UndoRedoScope UndoRedoScope::workScope(int workId)
{
    return UndoRedoScope(UndoRedoScopeType::Work, QString(), workId);
}

UndoRedoScope UndoRedoScope::contentScope(int contentId)
{
    return UndoRedoScope(UndoRedoScopeType::Content, QString(), contentId);
}

UndoRedoScope UndoRedoScope::settingsScope()
{
    return UndoRedoScope(UndoRedoScopeType::Settings);
}

UndoRedoScope UndoRedoScope::customScope(const QString &name)
{
    return UndoRedoScope(UndoRedoScopeType::Custom, name);
}

} // namespace Skribisto::Common::UndoRedo
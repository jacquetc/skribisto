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

#include <QHash>
#include <QString>

using namespace Qt::StringLiterals;

namespace Skribisto::Common::UndoRedo
{

enum class UndoRedoScopeType
{
    Root,
    Project,
    Content,
    Settings,
    Custom
};

class UndoRedoScope
{
  public:
    explicit UndoRedoScope(UndoRedoScopeType type, const QString &name = QString(), int id = -1);

    UndoRedoScopeType type() const;
    QString name() const;
    int id() const;
    QString scopeKey() const;

    bool operator==(const UndoRedoScope &other) const;
    bool operator!=(const UndoRedoScope &other) const;

    // Predefined scopes
    static UndoRedoScope rootScope();
    static UndoRedoScope projectScope(int projectId);
    static UndoRedoScope contentScope(int contentId);
    static UndoRedoScope settingsScope();
    static UndoRedoScope customScope(const QString &name);

  private:
    UndoRedoScopeType m_type;
    QString m_name;
    int m_id;
};

} // namespace Skribisto::Common::UndoRedo

// Make UndoRedoScope hashable for QHash
namespace std
{
template <> struct hash<Skribisto::Common::UndoRedo::UndoRedoScope>
{
    std::size_t operator()(const Skribisto::Common::UndoRedo::UndoRedoScope &scope) const noexcept
    {
        return qHash(scope.scopeKey());
    }
};
} // namespace std

// Qt hash function
inline uint qHash(const Skribisto::Common::UndoRedo::UndoRedoScope &scope, uint seed = 0)
{
    return qHash(scope.scopeKey(), seed);
}
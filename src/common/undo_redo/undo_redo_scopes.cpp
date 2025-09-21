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

#include "undo_redo/undo_redo_scopes.h"
#include <QRegularExpression>

Skribisto::Common::UndoRedo::Scopes::Scopes(const QStringList &scopeList)
{
    // Initialize the bit flags to 0x01, which is equivalent to 1
    int n = 0x01;

    // Loop through the list of scopes
    for (const auto &scope : scopeList)
    {
        // If the scope is "all", and exit the loop
        if (scope.toLower() == QString::fromLatin1("all"))
        {
            qFatal("do not add All to scopes");
        }

        // Add the scope to the list and map its flag value
        m_scopeList.append(scope);
        m_scopeHash.insert(scope, n);
        m_scopeMap.insert(n, scope);
        m_flags += n;

        // Increment the bit flag to the next power of 2
        n <<= 1;
    }
}

Skribisto::Common::UndoRedo::Scopes::Scopes(const QString &scopeList)
    : Scopes(scopeList.split(QRegularExpression(QString::fromLatin1("[\\s|,]+")), Qt::SkipEmptyParts))
{
}

Skribisto::Common::UndoRedo::Scope Skribisto::Common::UndoRedo::Scopes::createScopeFromString(
    const QString &scopeString)
{
    static auto expr = QRegularExpression(QString::fromLatin1("[\\s|,]+"));
    return createScopeFromString(scopeString.split(expr, Qt::SkipEmptyParts));
}
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

#include "query_handler.h"

namespace Skribisto::Common::UndoRedo
{

QueryBase::QueryBase(const QString &description, QObject *parent) : QObject(parent), m_description(description)
{
}

QString QueryBase::description() const
{
    return m_description;
}

QueryHandler::QueryHandler(QObject *parent) : QObject(parent)
{
}

void QueryHandler::executeQuery(std::shared_ptr<QueryBase> query)
{
    if (!query || m_isShuttingDown.load())
    {
        return;
    }

    m_currentQuery = query;

    // Connect to query finished signal
    connect(query.get(), &QueryBase::finished, this, &QueryHandler::onQueryFinished, Qt::UniqueConnection);

    // Execute query asynchronously
    query->asyncExecute();
}
void QueryHandler::cancelAllQueries()
{
    m_isShuttingDown = true;
    if (m_currentQuery)
    {
        // Try to cancel the current query if it's still running
        if (auto query = std::dynamic_pointer_cast<Query<QVariant>>(m_currentQuery))
        {
            if (query->m_watcher && query->m_watcher->isRunning())
            {
                query->m_watcher->cancel();
            }
        }
        m_currentQuery.reset();
    }
}

void QueryHandler::onQueryFinished(bool success)
{
    if (m_currentQuery)
    {
        auto query = m_currentQuery;
        m_currentQuery.reset();
        Q_EMIT queryFinished(query, success);
    }
}

} // namespace Skribisto::Common::UndoRedo
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

#include <QFuture>
#include <QFutureWatcher>
#include <QObject>
#include <QVariant>
#include <QtConcurrent/QtConcurrentRun>
#include <functional>
#include <memory>

using namespace Qt::StringLiterals;

namespace Skribisto::Common::UndoRedo
{

class QueryBase : public QObject
{
    Q_OBJECT

  public:
    explicit QueryBase(const QString &description, QObject *parent = nullptr);

    virtual void asyncExecute() = 0;
    virtual QVariant result() const = 0;

    QString description() const;

  Q_SIGNALS:
    void finished(bool isSuccessful);

  protected:
    QString m_description;
};

template <typename T> class Query : public QueryBase
{
  public:
    explicit Query(const QString &description, QObject *parent = nullptr);

    void setQueryFunction(const std::function<T()> &function);

    void asyncExecute() override;
    QVariant result() const override;

    T typedResult() const;

  private:
    void onQueryFinished();

    std::function<T()> m_queryFunction;
    QFutureWatcher<T> *m_watcher;
    T m_result;
    bool m_hasResult = false;
    friend class QueryHandler;
};

class QueryHandler : public QObject
{
    Q_OBJECT

  public:
    explicit QueryHandler(QObject *parent = nullptr);

    template <typename T> std::shared_ptr<Query<T>> createQuery(const QString &description);

    void executeQuery(std::shared_ptr<QueryBase> query);
    void cancelAllQueries();
  Q_SIGNALS:
    void queryFinished(std::shared_ptr<QueryBase> query, bool success);

  private Q_SLOTS:
    void onQueryFinished(bool success);

  private:
    std::shared_ptr<QueryBase> m_currentQuery;
    std::atomic<bool> m_isShuttingDown{false};
};

// Template implementation
template <typename T>
Query<T>::Query(const QString &description, QObject *parent)
    : QueryBase(description, parent), m_watcher(new QFutureWatcher<T>(this))
{
    connect(m_watcher, &QFutureWatcher<T>::finished, this, [this]() { this->onQueryFinished(); });
}

template <typename T> void Query<T>::setQueryFunction(const std::function<T()> &function)
{
    m_queryFunction = function;
}

template <typename T> void Query<T>::asyncExecute()
{
    if (m_queryFunction)
    {
        auto future = QtConcurrent::run(m_queryFunction);
        m_watcher->setFuture(future);
    }
}

template <typename T> QVariant Query<T>::result() const
{
    if (m_hasResult)
    {
        return QVariant::fromValue(m_result);
    }
    return QVariant();
}

template <typename T> T Query<T>::typedResult() const
{
    return m_result;
}

template <typename T> void Query<T>::onQueryFinished()
{
    if (m_watcher->isFinished())
    {
        try
        {
            m_result = m_watcher->result();
            m_hasResult = true;
            Q_EMIT finished(true);
        }
        catch (const std::exception &e)
        {
            qCritical() << "Exception in query execution:" << QString::fromStdString(e.what());
            Q_EMIT finished(false);
        }
        catch (...)
        {
            qCritical() << "Unknown exception in query execution";
            Q_EMIT finished(false);
        }
    }
}

template <typename T> std::shared_ptr<Query<T>> QueryHandler::createQuery(const QString &description)
{
    return std::make_shared<Query<T>>(description, this);
}

} // namespace Skribisto::Common::UndoRedo
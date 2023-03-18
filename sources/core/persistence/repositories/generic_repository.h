#pragma once

#include "QtConcurrent/qtconcurrenttask.h"
#include "database/interface_database.h"
#include "persistence/interface_generic_repository.h"
#include "persistence_global.h"
#include "result.h"

#include <QFuture>
#include <QObject>
#include <QReadWriteLock>
#include <QUuid>

using namespace Contracts::Database;

namespace Repository
{
// -------------------------------------------------

template <class T>
class SKR_PERSISTENCE_EXPORT GenericRepository : public virtual Contracts::Persistence::InterfaceGenericRepository<T>
{

  public:
    // InterfaceGenericRepository interface

  public:
    GenericRepository(InterfaceDatabase<T> *database) : m_database(database)
    {
    }

    Result<T> get(const QUuid &uuid) override;

    Result<QList<T>> getAll() override;

    Result<T> remove(T &&entity) override;

    Result<T> add(T &&entity) override;

    Result<T> update(T &&entity) override;

    Result<bool> exists(const QUuid &uuid) override;

  private:
    InterfaceDatabase<T> *m_database;
    QReadWriteLock m_lock;
};

template <class T> Result<T> GenericRepository<T>::get(const QUuid &uuid)
{
    QReadLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabase<T> *database, QUuid uuid) { return database->get(uuid); })
        .withArguments(m_database, uuid)
        .spawn()
        .result();
}

template <class T> Result<QList<T>> GenericRepository<T>::getAll()
{
    QReadLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabase<T> *database) { return database->getAll(); })
        .withArguments(m_database)
        .spawn()
        .result();
}

template <class T> Result<T> GenericRepository<T>::remove(T &&entity)
{
    QWriteLocker locker(&m_lock);
    return QtConcurrent::task(
               [](InterfaceDatabase<T> *database, T entity) { return database->remove(std::move(entity)); })
        .withArguments(m_database, std::move(entity))
        .spawn()
        .result();
}

template <class T> Result<T> GenericRepository<T>::add(T &&entity)
{
    QWriteLocker locker(&m_lock);

    return QtConcurrent::task([](InterfaceDatabase<T> *database, T entity) { return database->add(std::move(entity)); })
        .withArguments(m_database, std::move(entity))
        .spawn()
        .result();
}

template <class T> Result<T> GenericRepository<T>::update(T &&entity)
{
    QWriteLocker locker(&m_lock);
    return QtConcurrent::task(
               [](InterfaceDatabase<T> *database, T entity) { return database->update(std::move(entity)); })
        .withArguments(m_database, std::move(entity))
        .spawn()
        .result();
}

template <class T> Result<bool> GenericRepository<T>::exists(const QUuid &uuid)
{
    QReadLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabase<T> *database, QUuid uuid) { return database->exists(uuid); })
        .withArguments(m_database, uuid)
        .spawn()
        .result();
}

} // namespace Repository

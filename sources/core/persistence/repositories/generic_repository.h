#pragma once

#include "QtConcurrent/qtconcurrenttask.h"
#include "database/interface_database_table.h"
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
    GenericRepository(InterfaceDatabaseTable<T> *database) : m_database(database)
    {
    }

    virtual Result<T> get(const QUuid &uuid) override;
    virtual Result<T> get(const int &id) override;

    virtual Result<QList<T>> getAll() override;
    Result<QList<T>> getAll(const QHash<QString, QVariant> &filters) override;

    virtual Result<T> remove(T &&entity) override;

    virtual Result<T> add(T &&entity) override;

    virtual Result<T> update(T &&entity) override;

    virtual Result<bool> exists(const QUuid &uuid) override;
    virtual Result<bool> exists(int id) override;
    Result<void> clear() override;

    Result<SaveData> save(const QList<int> &idList) override;
    Result<void> restore(const SaveData &saveData) override;
    virtual Result<void> beginChanges() override;
    virtual Result<void> saveChanges() override;
    virtual Result<void> cancelChanges() override;

  protected:
    InterfaceDatabaseTable<T> *database() const;

  private:
    InterfaceDatabaseTable<T> *m_database;
    QReadWriteLock m_lock;

  public:
};

template <class T> Result<T> GenericRepository<T>::get(const QUuid &uuid)
{
    QReadLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabaseTable<T> *database, QUuid uuid) { return database->get(uuid); })
        .withArguments(m_database, uuid)
        .spawn()
        .result();
}

template <class T> Result<T> GenericRepository<T>::get(const int &id)
{
    QReadLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabaseTable<T> *database, int id) { return database->get(id); })
        .withArguments(m_database, id)
        .spawn()
        .result();
}

template <class T> Result<QList<T>> GenericRepository<T>::getAll()
{
    QReadLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabaseTable<T> *database) { return database->getAll(); })
        .withArguments(m_database)
        .spawn()
        .result();
}

template <class T> Result<QList<T>> GenericRepository<T>::getAll(const QHash<QString, QVariant> &filters)
{
    QReadLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabaseTable<T> *database, const QHash<QString, QVariant> &filters) {
               return database->getAll(filters);
           })
        .withArguments(m_database, filters)
        .spawn()
        .result();
}

template <class T> Result<T> GenericRepository<T>::remove(T &&entity)
{
    QWriteLocker locker(&m_lock);
    return QtConcurrent::task(
               [](InterfaceDatabaseTable<T> *database, T entity) { return database->remove(std::move(entity)); })
        .withArguments(m_database, std::move(entity))
        .spawn()
        .result();
}

template <class T> Result<T> GenericRepository<T>::add(T &&entity)
{
    QWriteLocker locker(&m_lock);

    return QtConcurrent::task(
               [](InterfaceDatabaseTable<T> *database, T entity) { return database->add(std::move(entity)); })
        .withArguments(m_database, std::move(entity))
        .spawn()
        .result();
}

template <class T> Result<T> GenericRepository<T>::update(T &&entity)
{
    QWriteLocker locker(&m_lock);
    return QtConcurrent::task(
               [](InterfaceDatabaseTable<T> *database, T entity) { return database->update(std::move(entity)); })
        .withArguments(m_database, std::move(entity))
        .spawn()
        .result();
}

template <class T> Result<bool> GenericRepository<T>::exists(const QUuid &uuid)
{
    QReadLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabaseTable<T> *database, QUuid uuid) { return database->exists(uuid); })
        .withArguments(m_database, uuid)
        .spawn()
        .result();
}

template <class T> Result<bool> GenericRepository<T>::exists(int id)
{

    QReadLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabaseTable<T> *database, int id) { return database->exists(id); })
        .withArguments(m_database, id)
        .spawn()
        .result();
}

template <class T> Result<void> GenericRepository<T>::clear()
{
    QReadLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabaseTable<T> *database) { return database->clear(); })
        .withArguments(m_database)
        .spawn()
        .result();
}

template <class T> Result<SaveData> GenericRepository<T>::save(const QList<int> &idList)
{
    QWriteLocker locker(&m_lock);
    return QtConcurrent::task(
               [](InterfaceDatabaseTable<T> *database, const QList<int> &idList) { return database->save(idList); })
        .withArguments(m_database, idList)
        .spawn()
        .result();
}

template <class T> Result<void> GenericRepository<T>::restore(const SaveData &saveData)

{
    QWriteLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabaseTable<T> *database, const SaveData &saveData) {
               return database->restore(saveData);
           })
        .withArguments(m_database, saveData)
        .spawn()
        .result();
}

template <class T> Result<void> GenericRepository<T>::beginChanges()
{
    QWriteLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabaseTable<T> *database) { return database->beginTransaction(); })
        .withArguments(m_database)
        .spawn()
        .result();
}

template <class T> Result<void> GenericRepository<T>::saveChanges()
{
    QWriteLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabaseTable<T> *database) { return database->commit(); })
        .withArguments(m_database)
        .spawn()
        .result();
}

template <class T> Result<void> GenericRepository<T>::cancelChanges()
{
    QWriteLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceDatabaseTable<T> *database) { return database->rollback(); })
        .withArguments(m_database)
        .spawn()
        .result();
}

template <class T> InterfaceDatabaseTable<T> *GenericRepository<T>::database() const
{
    return m_database;
}

} // namespace Repository

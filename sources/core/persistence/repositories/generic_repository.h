#pragma once

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
    GenericRepository(InterfaceDatabaseTable<T> *databaseTable) : m_databaseTable(databaseTable)
    {
    }

    virtual Result<T> get(const QUuid &uuid) override;
    virtual Result<T> get(const int &id) override;

    virtual Result<QList<T>> getAll() override;
    Result<QList<T>> getAll(const QHash<QString, QVariant> &filters) override;

    virtual Result<int> remove(int id) override;

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
    InterfaceDatabaseTable<T> *databaseTable() const;

  private:
    InterfaceDatabaseTable<T> *m_databaseTable;
    QReadWriteLock m_lock;

  public:
};

template <class T> Result<T> GenericRepository<T>::get(const QUuid &uuid)
{
    QReadLocker locker(&m_lock);
    return m_databaseTable->get(uuid);
}

template <class T> Result<T> GenericRepository<T>::get(const int &id)
{
    QReadLocker locker(&m_lock);
    return m_databaseTable->get(id);
}

template <class T> Result<QList<T>> GenericRepository<T>::getAll()
{
    QReadLocker locker(&m_lock);
    return m_databaseTable->getAll();
}

template <class T> Result<QList<T>> GenericRepository<T>::getAll(const QHash<QString, QVariant> &filters)
{
    QReadLocker locker(&m_lock);
    return m_databaseTable->getAll(filters);
}

template <class T> Result<int> GenericRepository<T>::remove(int id)
{
    QWriteLocker locker(&m_lock);
    return m_databaseTable->remove(id);
}

template <class T> Result<T> GenericRepository<T>::add(T &&entity)
{
    QWriteLocker locker(&m_lock);

    return m_databaseTable->add(std::move(entity));
}

template <class T> Result<T> GenericRepository<T>::update(T &&entity)
{
    QWriteLocker locker(&m_lock);

    return m_databaseTable->update(std::move(entity));
}

template <class T> Result<bool> GenericRepository<T>::exists(const QUuid &uuid)
{
    QReadLocker locker(&m_lock);
    return m_databaseTable->exists(uuid);
}

template <class T> Result<bool> GenericRepository<T>::exists(int id)
{

    QReadLocker locker(&m_lock);
    return m_databaseTable->exists(id);
}

template <class T> Result<void> GenericRepository<T>::clear()
{
    QReadLocker locker(&m_lock);

    return m_databaseTable->clear();
}

template <class T> Result<SaveData> GenericRepository<T>::save(const QList<int> &idList)
{
    QWriteLocker locker(&m_lock);
    return m_databaseTable->save(idList);
}

template <class T> Result<void> GenericRepository<T>::restore(const SaveData &saveData)

{
    QWriteLocker locker(&m_lock);
    return m_databaseTable->restore(saveData);
}

template <class T> Result<void> GenericRepository<T>::beginChanges()
{
    QWriteLocker locker(&m_lock);
    return m_databaseTable->beginTransaction();
}

template <class T> Result<void> GenericRepository<T>::saveChanges()
{
    QWriteLocker locker(&m_lock);
    return m_databaseTable->commit();
}

template <class T> Result<void> GenericRepository<T>::cancelChanges()
{
    QWriteLocker locker(&m_lock);
    return m_databaseTable->rollback();
}

template <class T> InterfaceDatabaseTable<T> *GenericRepository<T>::databaseTable() const
{
    return m_databaseTable;
}

} // namespace Repository

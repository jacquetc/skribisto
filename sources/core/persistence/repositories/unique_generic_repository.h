#pragma once
#include "database/interface_database.h"
#include "generic_repository.h"
#include "persistence/interface_unique_generic_repository.h"
#include "persistence_global.h"
#include "result.h"

#include <QObject>
#include <QReadWriteLock>
#include <QUuid>

using namespace Contracts::Database;

namespace Repository
{
// -------------------------------------------------

template <class T>
class SKR_PERSISTENCE_EXPORT UniqueGenericRepository
    : public GenericRepository<T>,
      public virtual Contracts::Persistence::InterfaceUniqueGenericRepository<T>
{

  public:
    // InterfaceGenericRepository interface

  public:
    UniqueGenericRepository(InterfaceDatabaseTable<T> *database) : GenericRepository<T>(database)
    {
    }
    Result<T> get();
    Result<T> add(T &&entity) override;
    Result<T> update(T &&entity) override;
    Result<bool> exists();

  private:
    Result<T> get(const QUuid &uuid) override = delete;
    Result<bool> exists(const QUuid &uuid) override = delete;
    Result<T> remove(T &&entity) override = delete;
    Result<QList<T>> getAll() override = delete;

  private:
    QReadWriteLock m_lock;
};

template <class T> Result<T> UniqueGenericRepository<T>::get()
{
    QReadLocker locker(&m_lock);
    return GenericRepository<T>::get(QUuid());
}

template <class T> Result<T> UniqueGenericRepository<T>::add(T &&entity)
{
    QWriteLocker locker(&m_lock);

    return GenericRepository<T>::add(entity);
}

template <class T> Result<T> UniqueGenericRepository<T>::update(T &&entity)
{
    QWriteLocker locker(&m_lock);
    return GenericRepository<T>::update(entity);
}

template <class T> Result<bool> UniqueGenericRepository<T>::exists()
{
    QReadLocker locker(&m_lock);
    return GenericRepository<T>::exists(QUuid());
}

} // namespace Repository

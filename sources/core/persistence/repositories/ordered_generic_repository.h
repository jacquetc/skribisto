#pragma once
#include "database/interface_ordered_database_table.h"
#include "generic_repository.h"
#include "persistence/interface_ordered_generic_repository.h"
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
class SKR_PERSISTENCE_EXPORT OrderedGenericRepository
    : public GenericRepository<T>,
      public virtual Contracts::Persistence::InterfaceOrderedGenericRepository<T>
{

  public:
    // InterfaceGenericRepository interface

  public:
    OrderedGenericRepository(InterfaceOrderedDatabaseTable<T> *database) : GenericRepository<T>(database){};

    Result<T> remove(T &&entity) override;

    Result<T> insert(T &&entity, int position) override;
    Result<T> insert(T &&entity, int position, const QHash<QString, QVariant> &filter) override;
    //    Result<QList<T>> getAll() override;
    //    Result<QList<T>> getAll(const QHash<QString, QVariant> &filter) override;
    //    Result<QMap<QString, QList<QVariantHash>>> save(const QList<QUuid> &uuidList) override;
    //    Result<void> restore(const QMap<QString, QList<QVariantHash>> &tablesData) override;

  private:
    Result<T> add(T &&entity) override = delete;
    QReadWriteLock m_lock;
};

template <class T> Result<T> OrderedGenericRepository<T>::remove(T &&entity)
{
    QWriteLocker locker(&m_lock);
    return QtConcurrent::task(
               [](InterfaceOrderedDatabaseTable<T> *database, T entity) { return database->remove(std::move(entity)); })
        .withArguments(GenericRepository<T>::database(), std::move(entity))
        .spawn()
        .result();
}

template <class T> Result<T> OrderedGenericRepository<T>::insert(T &&entity, int position)
{
    QWriteLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceOrderedDatabaseTable<T> *database, T entity, int position) {
               return database->insert(std::move(entity), position);
           })
        .withArguments(GenericRepository<T>::database(), std::move(entity), position)
        .spawn()
        .result();
}

template <class T>
Result<T> OrderedGenericRepository<T>::insert(T &&entity, int position, const QHash<QString, QVariant> &filter)
{
    QWriteLocker locker(&m_lock);
    return QtConcurrent::task([](InterfaceOrderedDatabaseTable<T> *database, T entity, int position,
                                 const QHash<QString, QVariant> &filter) {
               return database->insert(std::move(entity), position, filter);
           })
        .withArguments(GenericRepository<T>::database(), std::move(entity), position, filter)
        .spawn()
        .result();
}

} // namespace Repository

#pragma once
#include "contracts_global.h"
#include "result.h"
#include "types.h"
#include <QCoreApplication>
#include <QFuture>
#include <QUuid>

namespace Contracts::Persistence
{
template <class T> class SKR_CONTRACTS_EXPORT InterfaceGenericRepository
{
  public:
    virtual ~InterfaceGenericRepository()
    {
    }

    // classes can clean up

    virtual Result<T> get(const int &id) = 0;
    virtual Result<T> get(const QUuid &uuid) = 0;

    virtual Result<QList<T>> getAll() = 0;
    virtual Result<QList<T>> getAll(const QHash<QString, QVariant> &filters) = 0;

    virtual Result<int> remove(int id) = 0;
    virtual Result<QList<int>> remove(QList<int> ids) = 0;
    virtual Result<T> add(T &&entity) = 0;
    virtual Result<T> update(T &&entity) = 0;
    virtual Result<bool> exists(int id) = 0;
    virtual Result<bool> exists(const QUuid &uuid) = 0;
    virtual Result<void> clear() = 0;
    virtual Result<SaveData> save(const QList<int> &idList) = 0;
    virtual Result<void> restore(const SaveData &saveData) = 0;
    virtual Result<void> beginChanges() = 0;
    virtual Result<void> saveChanges() = 0;
    virtual Result<void> cancelChanges() = 0;
};
} // namespace Contracts::Persistence

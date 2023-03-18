#pragma once

#include <QCoreApplication>
#include <QFuture>
#include <QUuid>

#include "contracts_global.h"
#include "result.h"

namespace Contracts::Persistence
{
template <class T> class SKR_CONTRACTS_EXPORT InterfaceGenericRepository
{
  public:
    virtual ~InterfaceGenericRepository()
    {
    }

    // classes can clean up

    virtual Result<T> get(const QUuid &uuid) = 0;

    virtual Result<QList<T>> getAll() = 0;

    virtual Result<T> remove(T &&entity) = 0;
    virtual Result<T> add(T &&entity) = 0;
    virtual Result<T> update(T &&entity) = 0;
    virtual Result<bool> exists(const QUuid &uuid) = 0;
};
} // namespace Contracts::Persistence

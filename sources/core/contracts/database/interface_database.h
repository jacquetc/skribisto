#pragma once

#include "contracts_global.h"
#include "database/interface_database_context.h"
#include "result.h"
#include <QFuture>
#include <QUuid>

namespace Contracts::Database
{
template <class T> class SKR_CONTRACTS_EXPORT InterfaceDatabase
{
  public:
    virtual ~InterfaceDatabase()
    {
    }

    virtual Result<T> get(const QUuid &uuid) = 0;
    virtual Result<QList<T>> getAll() = 0;
    virtual Result<T> remove(T &&entity) = 0;
    virtual Result<T> add(T &&entity) = 0;
    virtual Result<T> update(T &&entity) = 0;
    virtual Result<bool> exists(const QUuid &uuid) = 0;

  private:
};

} // namespace Contracts::Database

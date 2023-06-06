#pragma once

#include "contracts_global.h"
#include "database/interface_foreign_entity.h"
#include "result.h"
#include "types.h"
#include <QHash>
#include <QString>
#include <QUuid>

namespace Contracts::Database
{

template <class T> class SKR_CONTRACTS_EXPORT InterfaceDatabaseTable : public virtual InterfaceForeignEntity<T>
{
  public:
    virtual ~InterfaceDatabaseTable()
    {
    }

    virtual Result<T> get(const QUuid &uuid) = 0;
    virtual Result<T> get(const int &id) = 0;
    virtual Result<QList<T>> getAll() = 0;
    virtual Result<QList<T>> getAll(const QHash<QString, QVariant> &filters) = 0;
    virtual Result<T> remove(T &&entity) = 0;
    virtual Result<T> add(T &&entity) = 0;
    virtual Result<T> update(T &&entity) = 0;
    virtual Result<bool> exists(const QUuid &uuid) = 0;
    virtual Result<bool> exists(int id) = 0;
    virtual Result<void> clear() = 0;
    virtual Result<SaveData> save(const QList<int> &idList) = 0;
    virtual Result<void> restore(const SaveData &saveData) = 0;
    virtual Result<void> beginTransaction() = 0;
    virtual Result<void> commit() = 0;
    virtual Result<void> rollback() = 0;

  private:
};

} // namespace Contracts::Database

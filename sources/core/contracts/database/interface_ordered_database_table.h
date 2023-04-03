#pragma once

#include "contracts_global.h"
#include "interface_database_table.h"
#include <QHash>
#include <QString>
#include <QUuid>
#include <QVariant>

namespace Contracts::Database
{
template <class T>
class SKR_CONTRACTS_EXPORT InterfaceOrderedDatabaseTable : public virtual Contracts::Database::InterfaceDatabaseTable<T>
{
  public:
    virtual ~InterfaceOrderedDatabaseTable()
    {
    }

    virtual Result<T> insert(T &&entity, int position) = 0;
    virtual Result<T> remove(T &&entity) = 0;
    virtual Result<T> insert(T &&entity, int position, const QHash<QString, QVariant> &filter) = 0;
};

} // namespace Contracts::Database

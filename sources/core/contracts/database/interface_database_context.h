#pragma once
#include "contracts_global.h"
#include <QSqlDatabase>
#include <QThreadPool>

namespace Contracts::Database
{
class SKR_CONTRACTS_EXPORT InterfaceDatabaseContext
{
  public:
    virtual ~InterfaceDatabaseContext()
    {
    }

    virtual QSqlDatabase getConnection() = 0;
    virtual bool beginTransaction(QSqlDatabase &database) = 0;
    virtual bool commitTransaction(QSqlDatabase &database) = 0;
    virtual bool rollbackTransaction(QSqlDatabase &database) = 0;
};
} // namespace Contracts::Database

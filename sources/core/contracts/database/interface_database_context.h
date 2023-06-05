#pragma once
#include "contracts_global.h"
#include "result.h"
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

    virtual QStringList entityClassNames() const = 0;
    virtual Result<void> init() = 0;
    virtual void setEntityClassNames(const QStringList &newEntityClassNames) = 0;

    virtual QSqlDatabase getConnection() = 0;
};
} // namespace Contracts::Database

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

    QStringList entityClassNames() const;
    void init();
    void setEntityClassNames(const QStringList &newEntityClassNames);

    virtual QSqlDatabase getConnection() = 0;
};
} // namespace Contracts::Database

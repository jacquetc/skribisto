#pragma once

#include "contracts_global.h"
#include "result.h"
#include "types.h"
#include <QList>
#include <QString>

namespace Contracts::Database
{

template <class T> class SKR_CONTRACTS_EXPORT InterfaceForeignEntity
{
  public:
    virtual ~InterfaceForeignEntity()
    {
    }

    virtual Result<QList<int>> getRelatedForeignIds(const T &entity, const QString &propertyName) = 0;
    virtual Result<QList<int>> getRelatedForeignIds(int entityId, const QString &propertyName) = 0;
    virtual Result<SaveData> save(const QList<int> &idList) = 0;
    virtual Result<void> restore(const SaveData &saveData) = 0;

  private:
};

} // namespace Contracts::Database

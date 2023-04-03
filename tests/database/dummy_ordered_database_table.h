#pragma once
#include "database/ordered_database_table.h"
#include "dummy_entity.h"

namespace Database
{

class DummyOrderedDatabaseTable : public Database::OrderedDatabaseTable<Domain::DummyEntity>
{
  public:
    explicit DummyOrderedDatabaseTable(InterfaceDatabaseContext *context) : OrderedDatabaseTable(context)
    {
    }

    Result<QPair<int, int>> getFuturePreviousAndNextOrderedIds(int position)
    {
        return Database::OrderedDatabaseTable<Domain::DummyEntity>::getFuturePreviousAndNextOrderedIds(position);
    }
    Result<QPair<int, int>> getPreviousAndNextOrderedIds(int entityId)
    {
        return Database::OrderedDatabaseTable<Domain::DummyEntity>::getPreviousAndNextOrderedIds(entityId);
    }
    Result<QList<QVariantMap>> getOrderingTableData() const
    {
        return Database::OrderedDatabaseTable<Domain::DummyEntity>::getOrderingTableData();
    }
};
} // namespace Database

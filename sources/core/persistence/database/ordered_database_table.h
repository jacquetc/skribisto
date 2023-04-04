#pragma once

#include "database/database_table.h"
#include "database/interface_ordered_database_table.h"
#include "ordered_entity.h"

#include <QSqlRecord>

using namespace Contracts::Database;

namespace Database
{

template <class T>
class SKR_PERSISTENCE_EXPORT OrderedDatabaseTable : public DatabaseTable<T>, public InterfaceOrderedDatabaseTable<T>
{
  public:
    explicit OrderedDatabaseTable(InterfaceDatabaseContext *context);

    OrderedDatabaseTable(const OrderedDatabaseTable &other);

    Result<T> add(T &&entity) override;
    Result<T> remove(T &&entity) override;
    Result<T> insert(T &&entity, int position) override;
    Result<T> insert(T &&entity, int position, const QHash<QString, QVariant> &filter) override;
    Result<QList<T>> getAll() override;
    Result<QList<T>> getAll(const QHash<QString, QVariant> &filter) override;
    Result<QList<QUuid>> getAllOrderedUuid();
    Result<QList<QUuid>> getAllOrderedUuid(const QHash<QString, QVariant> &filter);
    Result<QList<int>> getAllOrderedId();
    Result<QList<int>> getAllOrderedId(const QHash<QString, QVariant> &filter);

    Result<void> clear() override;
    Result<SaveData> save(const QList<int> &idList) override;
    Result<void> restore(const SaveData &tablesData) override;

  protected:
    Result<QPair<int, int>> getFuturePreviousAndNextOrderedIds(int position);
    Result<QPair<int, int>> getPreviousAndNextOrderedIds(int entityId);
    QString orderingTableName() const;
    Result<QList<QVariantMap>> getOrderingTableData() const;

  private:
    const QString m_orderingTableName = Tools<T>::getEntityOrderingTableName();

    int numberOfRows();
    Result<int> getOrderingId(int entity_id);
};

template <class T>
OrderedDatabaseTable<T>::OrderedDatabaseTable(InterfaceDatabaseContext *context) : DatabaseTable<T>(context)
{
    static_assert(std::is_base_of<Domain::OrderedEntity, T>::value, "T must inherit from Domain::OrderedEntity");
}

//----------------------------------

template <class T>
OrderedDatabaseTable<T>::OrderedDatabaseTable(const OrderedDatabaseTable &other) : DatabaseTable<T>(other)
{
    static_assert(std::is_base_of<Domain::OrderedEntity, T>::value, "T must inherit from Domain::OrderedEntity");
}

template <class T> Result<T> OrderedDatabaseTable<T>::add(T &&entity)
{

    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    const QString &entityTableName = this->tableName();
    const QString &entityOrderingTableName = this->orderingTableName();

    QSqlQuery query(database);

    // Insert the new entity using the add method
    Result<T> addResult = DatabaseTable<T>::add(std::move(entity));
    if (!addResult)
    {
        return addResult;
    }
    T insertedEntity = addResult.value();

    // Get the last ordering_id with a NULL next value
    QString getLastOrderingIdStr =
        QString("SELECT ordering_id FROM %1 WHERE next IS NULL").arg(entityOrderingTableName);
    if (!query.exec(getLastOrderingIdStr))
    {

        return Result<T>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), getLastOrderingIdStr));
    }

    bool isEmpty = false;
    QString insertOrderingStr;
    int lastOrderingId = -1;
    if (!query.first())
    {
        isEmpty = true;

        insertOrderingStr = QString("INSERT INTO %1 (previous, next) VALUES (NULL, NULL)").arg(entityOrderingTableName);

        query.prepare(insertOrderingStr);
    }
    else
    {
        lastOrderingId = query.value(0).toInt();
        // Create a new row in the ordering table
        insertOrderingStr =
            QString("INSERT INTO %1 (previous, next) VALUES (:lastOrderingId, NULL)").arg(entityOrderingTableName);
        if (!query.prepare(insertOrderingStr))
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), insertOrderingStr));
        }

        query.bindValue(":lastOrderingId", lastOrderingId);
    }
    if (!query.exec())
    {
        return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), insertOrderingStr));
    }

    // Get the new entity's ordering_id
    int newOrderingId = query.lastInsertId().toInt();
    {
        QString updateEntityStr = "UPDATE " + entityTableName + " SET ordering_id = :ordering_id WHERE id = :id";
        query.prepare(updateEntityStr);
        query.bindValue(":ordering_id", newOrderingId);
        query.bindValue(":id", insertedEntity.id());
        if (!query.exec())
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updateEntityStr));
        }
    }
    if (!isEmpty)
    {

        // Update the previous values for the affected entities
        QString updatePrevNextStr;

        updatePrevNextStr = QString("UPDATE %1 SET next = %2 WHERE ordering_id = :lastOrderingId")
                                .arg(entityOrderingTableName)
                                .arg(lastOrderingId);
        if (!query.prepare(updatePrevNextStr))
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updatePrevNextStr));
        }
        query.bindValue(":lastOrderingId", lastOrderingId);

        if (!query.exec())
        {

            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updatePrevNextStr));
        }
    }

    return Result<T>(insertedEntity);
}

//----------------------------------

template <class T> Result<T> OrderedDatabaseTable<T>::insert(T &&entity, int position)
{

    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    const QString &entityTableName = this->tableName();
    const QString &entityOrderingTableName = this->orderingTableName();

    QSqlQuery query(database);

    // dertermine previous row id ad next row id:

    auto futurePreviousAndNextRowIds = getFuturePreviousAndNextOrderedIds(position);
    if (futurePreviousAndNextRowIds.isError())
    {
        return Result<T>(futurePreviousAndNextRowIds.error());
    }

    int previousRowId = futurePreviousAndNextRowIds.value().first;
    int nextRowId = futurePreviousAndNextRowIds.value().second;

    qDebug() << "previousRowId:" << previousRowId << ", nextRowId:" << nextRowId;

    // Insert the new entity using the add method
    Result<T> addResult = DatabaseTable<T>::add(std::move(entity));
    if (!addResult)
    {
        return addResult;
    }
    T insertedEntity = addResult.value();

    // Create a new row in the ordering table
    QString insertOrderingStr =
        QString("INSERT INTO %1 (previous, next) VALUES (NULL, NULL)").arg(entityOrderingTableName);
    if (!query.exec(insertOrderingStr))
    {
        return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), insertOrderingStr));
    }
    qDebug() << "Executed SQL:" << insertOrderingStr;

    // Get the new entity's ordering_id
    int newOrderingId = query.lastInsertId().toInt();

    // update entity table ordering_id
    {
        QString updateEntityStr = "UPDATE " + entityTableName + " SET ordering_id = :ordering_id WHERE id = :id";
        query.prepare(updateEntityStr);
        query.bindValue(":ordering_id", newOrderingId);
        query.bindValue(":id", insertedEntity.id());
        if (!query.exec())
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updateEntityStr));
        }
        qDebug() << "Executed SQL:" << updateEntityStr;
    }

    // Update the previous and next values for the affected entities
    QString updateNewRowStr, updateNextRowStr, updatePrevRowStr;

    if (previousRowId >= 0 || (previousRowId == -1 && numberOfRows() > 0))
    {

        updatePrevRowStr = QString("UPDATE %1 SET next = %2 WHERE ordering_id = %3")
                               .arg(entityOrderingTableName)
                               .arg(newOrderingId)
                               .arg(previousRowId);

        if (!query.exec(updatePrevRowStr))
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updatePrevRowStr));
        }
        qDebug() << "Executed SQL:" << updatePrevRowStr;
    }
    if (nextRowId >= 0)
    {

        updateNextRowStr = QString("UPDATE %1 SET previous = %2 WHERE ordering_id = %3")
                               .arg(entityOrderingTableName)
                               .arg(newOrderingId)
                               .arg(nextRowId);

        if (!query.exec(updateNextRowStr))
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updateNextRowStr));
        }
        qDebug() << "Executed SQL:" << updateNextRowStr;
    }

    // Update the new ordered row with values for the affected entities

    updateNewRowStr = QString("UPDATE %1 SET previous = %2, next = %3 WHERE ordering_id = %4")
                          .arg(entityOrderingTableName, previousRowId == -1 ? "NULL" : QString::number(previousRowId),
                               nextRowId == -1 ? "NULL" : QString::number(nextRowId), QString::number(newOrderingId));

    if (!query.exec(updateNewRowStr))
    {
        return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updateNextRowStr));
    }
    qDebug() << "Executed SQL:" << updateNewRowStr;

    return Result<T>(insertedEntity);
}

template <class T>
Result<QList<QUuid>> OrderedDatabaseTable<T>::getAllOrderedUuid(const QHash<QString, QVariant> &filter)
{
    const QString &entityTableName = this->tableName();
    const QString &entityOrderingTableName = this->orderingTableName();
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    QSqlQuery query(database);
    QStringList filterKeys = filter.keys();

    const QStringList &columns = this->propertyColumns();

    QString fields;
    for (const QString &column : columns)
    {
        fields += column + ",";
    }
    fields.chop(1);

    QString ePointFields;
    for (const QString &column : columns)
    {
        ePointFields += "e." + column + ",";
    }
    ePointFields.chop(1);

    // Generate the filter query
    QString filterQuery;
    for (const QString &filterKey : filterKeys)
    {
        QString column = Tools<T>::fromPascalToSnakeCase(filterKey);
        filterQuery += column + " = :" + column + " AND ";
    }
    filterQuery.chop(5); // Remove the trailing " AND "

    // Generate the filter query with e.
    QString ePointFilterQuery;
    for (const QString &filterKey : filterKeys)
    {
        QString column = Tools<T>::fromPascalToSnakeCase(filterKey);
        ePointFilterQuery += " AND e." + column + " = :" + column;
    }

    QString queryStr = "WITH RECURSIVE ordered_entities(" + fields +
                       ", next, row_number) AS ("
                       "  SELECT " +
                       ePointFields +
                       ", deo.next, 1"
                       "  FROM " +
                       entityTableName +
                       " e"
                       "  JOIN " +
                       entityOrderingTableName +
                       " deo ON e.ordering_id = deo.ordering_id"
                       "  WHERE deo.previous IS NULL"
                       "  UNION ALL"
                       "  SELECT " +
                       ePointFields +
                       ", deo.next, row_number + 1"
                       "  FROM " +
                       entityTableName +
                       " e"
                       "  JOIN ordered_entities oe ON e.ordering_id = oe.next"
                       "  JOIN " +
                       entityOrderingTableName +
                       " deo ON e.ordering_id = deo.ordering_id"
                       ")"
                       " SELECT uuid FROM ordered_entities WHERE " +
                       filterQuery + " ORDER BY row_number ASC";

    if (!query.prepare(queryStr))
    {
        return Result<QList<QUuid>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    // Bind the filter values
    for (const QString &column : filterKeys)
    {
        query.bindValue(":" + Tools<T>::fromPascalToSnakeCase(column), filter.value(column));
    }
    query.bindValue(":null", QUuid());

    if (!query.exec())
    {
        return Result<QList<QUuid>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    QList<QUuid> orderedFilteredUuids;
    while (query.next())
    {
        orderedFilteredUuids.append(query.value(0).toUuid());
    }
    return Result<QList<QUuid>>(orderedFilteredUuids);
}
template <class T> Result<QList<int>> OrderedDatabaseTable<T>::getAllOrderedId(const QHash<QString, QVariant> &filter)
{
    const QString &entityTableName = this->tableName();
    const QString &entityOrderingTableName = this->orderingTableName();
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    QSqlQuery query(database);
    QStringList filterKeys = filter.keys();

    const QStringList &columns = this->propertyColumns();

    QString fields;
    for (const QString &column : columns)
    {
        fields += column + ",";
    }
    fields.chop(1);

    QString ePointFields;
    for (const QString &column : columns)
    {
        ePointFields += "e." + column + ",";
    }
    ePointFields.chop(1);

    // Generate the filter query
    QString filterQuery;
    for (const QString &filterKey : filterKeys)
    {
        QString column = Tools<T>::fromPascalToSnakeCase(filterKey);
        filterQuery += column + " = :" + column + " AND ";
    }
    filterQuery.chop(5); // Remove the trailing " AND "

    // Generate the filter query with e.
    QString ePointFilterQuery;
    for (const QString &filterKey : filterKeys)
    {
        QString column = Tools<T>::fromPascalToSnakeCase(filterKey);
        ePointFilterQuery += " AND e." + column + " = :" + column;
    }

    QString queryStr = "WITH RECURSIVE ordered_entities(" + fields +
                       ", next, row_number) AS ("
                       "  SELECT " +
                       ePointFields +
                       ", deo.next, 1"
                       "  FROM " +
                       entityTableName +
                       " e"
                       "  JOIN " +
                       entityOrderingTableName +
                       " deo ON e.ordering_id = deo.ordering_id"
                       "  WHERE deo.previous IS NULL"
                       "  UNION ALL"
                       "  SELECT " +
                       ePointFields +
                       ", deo.next, row_number + 1"
                       "  FROM " +
                       entityTableName +
                       " e"
                       "  JOIN ordered_entities oe ON e.ordering_id = oe.next"
                       "  JOIN " +
                       entityOrderingTableName +
                       " deo ON e.ordering_id = deo.ordering_id"
                       ")"
                       " SELECT id FROM ordered_entities WHERE " +
                       filterQuery + " ORDER BY row_number ASC";

    if (!query.prepare(queryStr))
    {
        return Result<QList<int>>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    // Bind the filter values
    for (const QString &column : filterKeys)
    {
        query.bindValue(":" + Tools<T>::fromPascalToSnakeCase(column), filter.value(column));
    }
    query.bindValue(":null", QUuid());

    if (!query.exec())
    {
        return Result<QList<int>>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    QList<int> orderedFilteredIds;
    while (query.next())
    {
        orderedFilteredIds.append(query.value(0).toInt());
    }
    return Result<QList<int>>(orderedFilteredIds);
}

template <class T> Result<void> OrderedDatabaseTable<T>::clear()
{
    // Clear DatabaseTable
    Result<void> clearResult = DatabaseTable<T>::clear();
    if (!clearResult)
    {
        return clearResult;
    }

    // Clear ordering table :

    const QString &orderingTableName = this->orderingTableName();
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    QSqlQuery query(database);
    QString queryStrMain = "DELETE FROM " + orderingTableName;

    if (!query.prepare(queryStrMain))
    {
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStrMain));
    }
    if (!query.exec())
    {
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_clear_failed", "Failed to clear the main table"));
    }

    return Result<void>();
}

template <class T> Result<QList<QUuid>> OrderedDatabaseTable<T>::getAllOrderedUuid()
{
    const QString &entityTableName = this->tableName();
    const QString &entityOrderingTableName = this->orderingTableName();
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();

    QSqlQuery query(database);

    QString queryStr = "WITH RECURSIVE ordered_entities(uuid, next, row_number) AS ("
                       "  SELECT e.uuid, deo.next, 1"
                       "  FROM " +
                       entityTableName +
                       " e"
                       "  JOIN " +
                       entityOrderingTableName +
                       " deo ON e.ordering_id = deo.ordering_id"
                       "  WHERE deo.previous IS NULL"
                       "  UNION ALL"
                       "  SELECT e.uuid, deo.next, row_number + 1"
                       "  FROM " +
                       entityTableName +
                       " e"
                       "  JOIN ordered_entities oe ON e.ordering_id = oe.next"
                       "  JOIN " +
                       entityOrderingTableName +
                       " deo ON e.ordering_id = deo.ordering_id"
                       ")"
                       " SELECT uuid"
                       " FROM ordered_entities"
                       " ORDER BY row_number ASC";

    if (!query.prepare(queryStr))
    {
        return Result<QList<QUuid>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    if (!query.exec())
    {
        return Result<QList<QUuid>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    QList<QUuid> orderedUuids;
    while (query.next())
    {
        orderedUuids.append(query.value(0).toUuid());
    }

    return Result<QList<QUuid>>(orderedUuids);
}

template <class T> Result<QList<int>> OrderedDatabaseTable<T>::getAllOrderedId()
{
    const QString &entityTableName = this->tableName();
    const QString &entityOrderingTableName = this->orderingTableName();
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();

    QSqlQuery query(database);

    QString queryStr = "WITH RECURSIVE ordered_entities(id, next, row_number) AS ("
                       "  SELECT e.id, deo.next, 1"
                       "  FROM " +
                       entityTableName +
                       " e"
                       "  JOIN " +
                       entityOrderingTableName +
                       " deo ON e.ordering_id = deo.ordering_id"
                       "  WHERE deo.previous IS NULL"
                       "  UNION ALL"
                       "  SELECT e.id, deo.next, row_number + 1"
                       "  FROM " +
                       entityTableName +
                       " e"
                       "  JOIN ordered_entities oe ON e.ordering_id = oe.next"
                       "  JOIN " +
                       entityOrderingTableName +
                       " deo ON e.ordering_id = deo.ordering_id"
                       ")"
                       " SELECT id"
                       " FROM ordered_entities"
                       " ORDER BY row_number ASC";

    if (!query.prepare(queryStr))
    {
        return Result<QList<int>>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    if (!query.exec())
    {
        return Result<QList<int>>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    QList<int> orderedIds;
    while (query.next())
    {
        orderedIds.append(query.value(0).toInt());
    }

    return Result<QList<int>>(orderedIds);
}
template <class T>
Result<T> OrderedDatabaseTable<T>::insert(T &&entity, int position, const QHash<QString, QVariant> &filter)
{
    const QString &entityTableName = this->tableName();
    const QString &entityOrderingTableName = this->orderingTableName();
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    QSqlQuery query(database);

    //------------------------
    // get general ordered Id
    //------------------------

    QList<int> orderedIds = getAllOrderedId().value();

    //------------------------
    // get filtered ordered Id
    //------------------------

    QList<int> orderedFilteredIds = getAllOrderedId(filter).value();

    //------------------------
    // get positions and Ids
    //------------------------

    int previousRowId = -1;
    int nextRowId = -1;

    if (position == -1 || position > orderedFilteredIds.count())
    {
        position = orderedFilteredIds.count();
    }

    if (orderedFilteredIds.count() == 0 && orderedIds.count() > 0)
    {
        previousRowId = orderedIds.last();
        // nextUuid stays invalid
    }
    // if appending to last of filtered list:
    else if (position == orderedFilteredIds.count())
    {
        // all is empty
        if (orderedFilteredIds.count() == 0 && orderedIds.count() == 0)
        {
            // previousUuid stays invalid
            // nextUuid stays invalid
        }
        // if it is really for the last of general list
        else if (orderedFilteredIds.last() == orderedIds.last())
        {
            previousRowId = orderedFilteredIds.last();
            // nextUuid stays invalid
        }
        // normal case : general uuid after the last of the filtered list
        else
        {
            previousRowId = orderedFilteredIds.last();
            nextRowId = orderedIds.at(orderedIds.indexOf(orderedFilteredIds.last()) + 1);
        }
    }

    // if appending to first of filtered list:
    else if (position == 0)
    {
        // all is empty
        if (orderedFilteredIds.count() == 0 && orderedIds.count() == 0)
        {
            // previousUuid stays invalid
            // nextUuid stays invalid
        }
        // if it is really for the first of general list
        else if (orderedFilteredIds.first() == orderedIds.first())
        {
            // previousUuid stays invalid
            nextRowId = orderedFilteredIds.first();
        }
        // normal case : general uuid after the last of the filtered list
        else
        {

            previousRowId = orderedIds.at(orderedIds.indexOf(orderedFilteredIds.first()) - 1);
            nextRowId = orderedFilteredIds.first();
        }
    }
    // normal operation
    else
    {
        previousRowId = orderedFilteredIds.at(position - 1);
        nextRowId = orderedFilteredIds.at(position);
    }
    // translate to ordering ids:

    int previousOrderingId = -1;
    if (previousRowId > 0)

    {
        auto result = getOrderingId(previousRowId);
        if (result.hasError())
        {
            return Result<T>(result.error());
        }
        previousOrderingId = result.value();
    }

    int nextOrderingId = -1;
    if (nextRowId > 0)
    {
        auto result = getOrderingId(nextRowId);
        if (result.hasError())
        {
            return Result<T>(result.error());
        }

        nextOrderingId = result.value();
    }
    //------------------------
    // Insert the new entity using the add method
    //------------------------

    Result<T> addResult = DatabaseTable<T>::add(std::move(entity));
    if (!addResult)
    {
        return addResult;
    }
    T insertedEntity = addResult.value();

    // Create a new row in the ordering table
    QString insertOrderingStr =
        QString("INSERT INTO %1 (previous, next) VALUES (NULL, NULL)").arg(entityOrderingTableName);
    if (!query.exec(insertOrderingStr))
    {
        return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), insertOrderingStr));
    }
    qDebug() << "Executed SQL:" << insertOrderingStr;

    // Get the new entity's ordering_id
    int newOrderingId = query.lastInsertId().toInt();

    // update entity table ordering_id
    {
        QString updateEntityStr = "UPDATE " + entityTableName + " SET ordering_id = :ordering_id WHERE id = :id";
        query.prepare(updateEntityStr);
        query.bindValue(":ordering_id", newOrderingId);
        query.bindValue(":id", insertedEntity.id());
        if (!query.exec())
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updateEntityStr));
        }
        qDebug() << "Executed SQL:" << updateEntityStr;
    }

    // Update the previous and next values for the affected entities
    QString updateNewRowStr, updateNextRowStr, updatePrevRowStr;

    if (previousOrderingId >= 0 || (previousOrderingId == -1 && numberOfRows() > 0))
    {

        updatePrevRowStr = QString("UPDATE %1 SET next = %2 WHERE ordering_id = %3")
                               .arg(entityOrderingTableName)
                               .arg(newOrderingId)
                               .arg(previousOrderingId);

        if (!query.exec(updatePrevRowStr))
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updatePrevRowStr));
        }
        qDebug() << "Executed SQL:" << updatePrevRowStr;
    }
    if (nextOrderingId >= 0)
    {

        updateNextRowStr = QString("UPDATE %1 SET previous = %2 WHERE ordering_id = %3")
                               .arg(entityOrderingTableName)
                               .arg(newOrderingId)
                               .arg(nextOrderingId);

        if (!query.exec(updateNextRowStr))
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updateNextRowStr));
        }
        qDebug() << "Executed SQL:" << updateNextRowStr;
    }

    // Update the new ordered row with values for the affected entities

    updateNewRowStr =
        QString("UPDATE %1 SET previous = %2, next = %3 WHERE ordering_id = %4")
            .arg(entityOrderingTableName, previousOrderingId == -1 ? "NULL" : QString::number(previousOrderingId),
                 nextOrderingId == -1 ? "NULL" : QString::number(nextOrderingId), QString::number(newOrderingId));

    if (!query.exec(updateNewRowStr))
    {
        return Result<T>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updateNextRowStr));
    }
    qDebug() << "Executed SQL:" << updateNewRowStr;

    return Result<T>(insertedEntity);
}

//----------------------------------

template <class T> Result<QList<T>> OrderedDatabaseTable<T>::getAll(const QHash<QString, QVariant> &filter)
{
    const QString &entityTableName = this->tableName();
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    const QString &entityOrderingTableName = this->orderingTableName();
    QSqlQuery query(database);
    QStringList filterKeys = filter.keys();

    const QStringList &columns = this->propertyColumns();

    QString fields;
    for (const QString &column : columns)
    {
        fields += column + ",";
    }
    fields.chop(1);

    QString ePointFields;
    for (const QString &column : columns)
    {
        ePointFields += "e." + column + ",";
    }
    ePointFields.chop(1);

    // Generate the filter query
    QString filterQuery;
    for (const QString &filterKey : filterKeys)
    {
        QString column = Tools<T>::fromPascalToSnakeCase(filterKey);
        filterQuery += column + " = :" + column + " AND ";
    }
    filterQuery.chop(5); // Remove the trailing " AND "

    // Generate the filter query with e.
    QString ePointFilterQuery;
    for (const QString &filterKey : filterKeys)
    {
        QString column = Tools<T>::fromPascalToSnakeCase(filterKey);
        ePointFilterQuery += " AND e." + column + " = :" + column;
    }

    QString queryStr = "WITH RECURSIVE ordered_entities(" + fields +
                       ", next, row_number) AS ("
                       "  SELECT " +
                       ePointFields +
                       ", deo.next, 1"
                       "  FROM " +
                       entityTableName +
                       " e"
                       "  JOIN " +
                       entityOrderingTableName +
                       " deo ON e.ordering_id = deo.ordering_id"
                       "  WHERE deo.previous IS NULL"
                       "  UNION ALL"
                       "  SELECT " +
                       ePointFields +
                       ", deo.next, row_number + 1"
                       "  FROM " +
                       entityTableName +
                       " e"
                       "  JOIN ordered_entities oe ON e.ordering_id = oe.next"
                       "  JOIN " +
                       entityOrderingTableName +
                       " deo ON e.ordering_id = deo.ordering_id"
                       ")"
                       " SELECT " +
                       fields + " FROM ordered_entities WHERE " + filterQuery + " ORDER BY row_number ASC";

    // qDebug() << queryStr;
    if (!query.prepare(queryStr))
    {
        return Result<QList<T>>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    // Bind the filter values
    for (const QString &column : filterKeys)
    {
        query.bindValue(":" + Tools<T>::fromPascalToSnakeCase(column), filter.value(column));
    }
    query.bindValue(":null", QUuid());

    if (!query.exec())
    {
        return Result<QList<T>>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    QList<T> entities;
    while (query.next())
    {
        T entity;
        Tools<T>::readEntityFromQuery(entity, query);
        entities.append(entity);
    }
    return Result<QList<T>>(entities);
}

//----------------------------------

template <class T> Result<QList<T>> OrderedDatabaseTable<T>::getAll()
{
    const QString &entityTableName = Tools<T>::getEntityTableName();
    const QString &orderingTableName = this->orderingTableName();
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    QSqlQuery query(database);

    const QStringList &columns = this->propertyColumns();

    QString ePointFields;
    for (const QString &column : columns)
    {
        ePointFields += "e." + column + ",";
    }
    ePointFields.chop(1);

    QString fields;
    for (const QString &column : columns)
    {
        fields += column + ",";
    }
    fields.chop(1);
    QString queryStr = "WITH RECURSIVE ordered_entities(" + fields +
                       ", next, row_number) AS ("
                       "  SELECT " +
                       ePointFields +
                       ", deo.next, 1"
                       "  FROM " +
                       entityTableName +
                       " e"
                       "  JOIN " +
                       orderingTableName +
                       " deo ON e.ordering_id = deo.ordering_id"
                       "  WHERE deo.previous IS NULL"
                       "  UNION ALL"
                       "  SELECT " +
                       ePointFields +
                       ", deo.next, row_number + 1"
                       "  FROM " +
                       entityTableName +
                       " e"
                       "  JOIN ordered_entities oe ON e.ordering_id = oe.next"
                       "  JOIN " +
                       orderingTableName +
                       " deo ON e.ordering_id = deo.ordering_id"
                       ")"
                       " SELECT " +
                       fields +
                       " FROM ordered_entities"
                       " ORDER BY row_number ASC";

    qDebug() << "queryStr" << queryStr;
    if (!query.prepare(queryStr))
    {
        return Result<QList<T>>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    if (!query.exec())
    {
        return Result<QList<T>>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    QList<T> entities;
    while (query.next())
    {
        T entity;
        Tools<T>::readEntityFromQuery(entity, query);
        entities.append(entity);
    }
    return Result<QList<T>>(entities);
}

//----------------------------------

template <class T> Result<T> OrderedDatabaseTable<T>::remove(T &&entity)
{
    // Get the previous and next ordered ids for the given entity
    Result<QPair<int, int>> previousAndNextOrderedIdsResult = getPreviousAndNextOrderedIds(entity.id());
    if (previousAndNextOrderedIdsResult.isError())
    {
        return Result<T>(previousAndNextOrderedIdsResult.error());
    }

    QPair<int, int> previousAndNextOrderedIds = previousAndNextOrderedIdsResult.value();
    int previousOrderedId = previousAndNextOrderedIds.first;
    int nextOrderedId = previousAndNextOrderedIds.second;

    // Update the previous and next rows in the ordering table
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    const QString &entityOrderingTableName = this->orderingTableName();

    QSqlQuery query(database);

    // Update the previous row in the ordering table
    if (previousOrderedId != -1)
    {
        QString updatePreviousRowQueryStr = QString("UPDATE %1 SET next = %2 WHERE ordering_id = %3")
                                                .arg(entityOrderingTableName)
                                                .arg(nextOrderedId == -1 ? "NULL" : QString::number(nextOrderedId))
                                                .arg(previousOrderedId);

        if (!query.exec(updatePreviousRowQueryStr))
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updatePreviousRowQueryStr));
        }
    }

    // Update the next row in the ordering table
    if (nextOrderedId != -1)
    {
        QString updateNextRowQueryStr = QString("UPDATE %1 SET previous = %2 WHERE ordering_id = %3")
                                            .arg(entityOrderingTableName)
                                            .arg(previousOrderedId == -1 ? "NULL" : QString::number(previousOrderedId))
                                            .arg(nextOrderedId);

        if (!query.exec(updateNextRowQueryStr))
        {
            return Result<T>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updateNextRowQueryStr));
        }
    }

    // Remove the entity
    return DatabaseTable<T>::remove(std::forward<T>(entity));
}

template <class T> Result<SaveData> OrderedDatabaseTable<T>::save(const QList<int> &idList)
{
    QMap<QString, QList<QVariantHash>> resultMap;

    // Save the main table rows
    resultMap = DatabaseTable<T>::save(idList).value();

    QSqlDatabase database = this->databaseContext()->getConnection();
    const QString &entityName = this->tableName();

    QList<QVariantHash> orderingTableRows;

    QString orderingTableName = this->orderingTableName();
    QString queryStr;

    if (idList.isEmpty())
    {
        queryStr = "SELECT ordering_id, previous, next FROM " + orderingTableName;
    }
    else
    {
        queryStr = "SELECT ordering_id, previous, next FROM " + orderingTableName + " WHERE ordering_id IN (%1)";

        QString idPlaceholders;
        for (int i = 0; i < idList.count(); ++i)
        {
            idPlaceholders += ":id" + QString::number(i) + ",";
        }
        idPlaceholders.chop(1);
        queryStr = queryStr.arg(idPlaceholders);
    }

    QSqlQuery query(database);
    if (!query.prepare(queryStr))
    {
        return Result<QMap<QString, QList<QVariantHash>>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }
    if (!idList.isEmpty())
    {
        for (int i = 0; i < idList.count(); ++i)
        {
            query.bindValue(":id" + QString::number(i), idList[i]);
        }
    }

    QSet<int> processedIds;

    if (query.exec())
    {
        while (query.next())
        {
            QVariantHash row;
            row["ordering_id"] = query.value("ordering_id");
            row["previous"] = query.value("previous");
            row["next"] = query.value("next");

            orderingTableRows.append(row);

            processedIds.insert(row["ordering_id"].toInt());
            processedIds.insert(row["previous"].toInt());
            processedIds.insert(row["next"].toInt());
        }
    }
    else
    {
        // Handle query error
        return Result<QMap<QString, QList<QVariantHash>>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    // Add the ordering table rows to the result map
    resultMap.insert(orderingTableName, orderingTableRows);

    // Save the rows for next and previous IDs not in the idList
    if (!idList.isEmpty())
    {
        QList<int> nextPrevIds;
        for (const int &processedId : processedIds)
        {
            if (!idList.contains(processedId))
            {
                nextPrevIds.append(processedId);
            }
        }

        // save the nextPrevUuids

        queryStr = "SELECT ordering_id, previous, next FROM " + orderingTableName + " WHERE ordering_id IN (%1)";
        QString idPlaceholders;
        for (int i = 0; i < nextPrevIds.count(); ++i)
        {
            idPlaceholders += ":id" + QString::number(i) + ",";
        }
        idPlaceholders.chop(1);
        queryStr = queryStr.arg(idPlaceholders);

        QSqlQuery query(database);
        if (!query.prepare(queryStr))
        {
            return Result<QMap<QString, QList<QVariantHash>>>(
                Error("OrderedDatabaseTable", Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        if (!nextPrevIds.isEmpty())
        {
            for (int i = 0; i < nextPrevIds.count(); ++i)
            {
                query.bindValue(":id" + QString::number(i), nextPrevIds[i]);
            }
        }

        if (query.exec())
        {
            while (query.next())
            {
                QVariantHash row;
                row["ordering_id"] = query.value("ordering_id");
                row["previous"] = query.value("previous");
                row["next"] = query.value("next");

                orderingTableRows.append(row);
            }
        }
        else
        {
            // Handle query error
            return Result<QMap<QString, QList<QVariantHash>>>(
                Error("OrderedDatabaseTable", Error::Critical, "sql_error", query.lastError().text(), queryStr));
        }
        resultMap.insert(orderingTableName, orderingTableRows);

        // save the corresponding rows in the other tables:
        QMap<QString, QList<QVariantHash>> nextPrevResult = DatabaseTable<T>::save(nextPrevIds).value();
        resultMap[entityName].append(nextPrevResult[entityName]);
    }

    return Result<QMap<QString, QList<QVariantHash>>>(resultMap);
}

template <class T> Result<void> OrderedDatabaseTable<T>::restore(const SaveData &tablesData)
{
    // Restore main tables
    Result<void> mainRestoreResult = DatabaseTable<T>::restore(tablesData);
    if (!mainRestoreResult)
    {
        return mainRestoreResult;
    }

    QSqlDatabase database = this->databaseContext()->getConnection();
    QString orderingTableName = this->orderingTableName();

    // Restore the ordering table
    if (tablesData.contains(orderingTableName))
    {
        QList<QVariantHash> orderingTableRows = tablesData.value(orderingTableName);

        for (const QVariantHash &row : orderingTableRows)
        {
            // Check if the row exists in the table
            QSqlQuery checkQuery(database);
            QString checkQueryStr = "SELECT COUNT(*) FROM " + orderingTableName + " WHERE ordering_id = :ordering_id";

            if (!checkQuery.prepare(checkQueryStr))
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", checkQuery.lastError().text(), checkQueryStr));
            }
            checkQuery.bindValue(":ordering_id", row.value("ordering_id"));
            if (!checkQuery.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", checkQuery.lastError().text(), checkQueryStr));
            }
            if (!checkQuery.next())
            {
                return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", checkQuery.lastError().text()));
            }

            int rowCount = checkQuery.value(0).toInt();

            if (rowCount == 1)
            {
                // Update the existing row
                QString updateStr = "UPDATE " + orderingTableName +
                                    " SET previous = :previous, next = :next WHERE ordering_id = :ordering_id";
                QSqlQuery updateQuery(database);
                updateQuery.prepare(updateStr);
                updateQuery.bindValue(":ordering_id", row.value("ordering_id"));
                updateQuery.bindValue(":previous", row.value("previous"));
                updateQuery.bindValue(":next", row.value("next"));

                if (!updateQuery.exec())
                {
                    return Result<void>(
                        Error(Q_FUNC_INFO, Error::Critical, "sql_error", updateQuery.lastError().text()));
                }
            }
            else
            {
                // Insert the missing row
                QString insertStr = "INSERT INTO " + orderingTableName +
                                    " (ordering_id, previous, next) VALUES (:ordering_id, :previous, :next)";
                QSqlQuery insertQuery(database);
                if (!insertQuery.prepare(insertStr))
                {
                    return Result<void>(
                        Error(Q_FUNC_INFO, Error::Critical, "sql_error", insertQuery.lastError().text(), insertStr));
                }
                insertQuery.bindValue(":ordering_id", row.value("ordering_id"));
                insertQuery.bindValue(":previous", row.value("previous"));
                insertQuery.bindValue(":next", row.value("next"));

                if (!insertQuery.exec())
                {
                    return Result<void>(
                        Error(Q_FUNC_INFO, Error::Critical, "sql_error", insertQuery.lastError().text()));
                }
            }
        }
    }

    return Result<void>();
}

//--------------------------------------------------------------

template <class T> Result<QPair<int, int>> OrderedDatabaseTable<T>::getFuturePreviousAndNextOrderedIds(int position)
{
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    const QString &entityTableName = Tools<T>::getEntityTableName();
    const QString &orderingTableName = this->orderingTableName();

    QSqlQuery query(database);

    // dertermine previous row id and next row id:

    int previousRowId = -1;
    int nextRowId = -1;

    // empty ?
    QString checkQueryStr = "SELECT COUNT(*) FROM " + orderingTableName;

    if (!query.prepare(checkQueryStr))
    {
        return Result<QPair<int, int>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), checkQueryStr));
    }

    if (!query.exec())
    {
        return Result<QPair<int, int>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), checkQueryStr));
    }
    if (!query.next())
    {
        return Result<QPair<int, int>>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text()));
    }

    int rowCount = query.value(0).toInt();

    if (position == -1 || position > rowCount)
    {
        position = rowCount;
    }

    if (rowCount == 0)
    {
        previousRowId = -1;
        nextRowId = -1;
    }
    else if (rowCount > 0 && rowCount <= position)
    {

        QString selectNextStr = QString("SELECT ordering_id FROM %1 WHERE next IS NULL").arg(orderingTableName);

        if (!query.exec(selectNextStr))
        {
            return Result<QPair<int, int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), selectNextStr));
        }

        if (query.next())
        {
            previousRowId = query.value(0).toInt();
        }
        nextRowId = -1;
    }
    else if (position == 0)
    {
        previousRowId = -1;

        QString selectPreviousStr = QString("SELECT ordering_id FROM %1 WHERE previous IS NULL").arg(orderingTableName);

        if (!query.exec(selectPreviousStr))
        {
            return Result<QPair<int, int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), selectPreviousStr));
        }

        if (query.next())
        {
            nextRowId = query.value(0).toInt();
        }
    }
    else
    {
        QString selectPreviousStr = QString("WITH RECURSIVE ordered_entities(ordering_id, next, row_number) AS ("
                                            "  SELECT ordering_id, next, 1"
                                            "  FROM %1"
                                            "  WHERE previous IS NULL"
                                            "  UNION ALL"
                                            "  SELECT deo.ordering_id, deo.next, oe.row_number + 1"
                                            "  FROM %1 deo"
                                            "  JOIN ordered_entities oe ON deo.previous = oe.ordering_id"
                                            ")"
                                            "SELECT ordering_id FROM ordered_entities WHERE row_number = %2")
                                        .arg(orderingTableName)
                                        .arg(position);
        if (!query.exec(selectPreviousStr))
        {
            return Result<QPair<int, int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), selectPreviousStr));
        }
        if (query.next())
        {
            previousRowId = query.value(0).toInt();
        }

        QString selectNextStr = QString("WITH RECURSIVE ordered_entities(ordering_id, next, row_number) AS ("
                                        "  SELECT ordering_id, next, 1"
                                        "  FROM %1"
                                        "  WHERE previous IS NULL"
                                        "  UNION ALL"
                                        "  SELECT deo.ordering_id, deo.next, oe.row_number + 1"
                                        "  FROM %1 deo"
                                        "  JOIN ordered_entities oe ON deo.previous = oe.ordering_id"
                                        ")"
                                        "SELECT ordering_id FROM ordered_entities WHERE row_number = %2")
                                    .arg(orderingTableName)
                                    .arg(position + 1);
        if (!query.exec(selectNextStr))
        {
            return Result<QPair<int, int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), selectNextStr));
        }
        if (query.next())
        {
            nextRowId = query.value(0).toInt();
        }
    }

    return Result<QPair<int, int>>(QPair<int, int>(previousRowId, nextRowId));
}

template <class T> Result<QPair<int, int>> OrderedDatabaseTable<T>::getPreviousAndNextOrderedIds(int entityId)
{
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    const QString &entityTableName = Tools<T>::getEntityTableName();
    const QString &orderingTableName = this->orderingTableName();

    QSqlQuery query(database);

    // Determine previous and next row ids
    int previousRowId = -1;
    int nextRowId = -1;

    // Get the ordering_id for the given entityId
    QString getEntityOrderingIdStr =
        QString("SELECT ordering_id FROM %1 WHERE id = %2").arg(entityTableName).arg(entityId);
    if (!query.exec(getEntityOrderingIdStr))
    {
        return Result<QPair<int, int>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), getEntityOrderingIdStr));
    }

    int entityOrderingId = -1;
    if (query.next())
    {
        entityOrderingId = query.value(0).toInt();
    }
    else
    {
        return Result<QPair<int, int>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", "Entity not found for given entityId."));
    }

    // Get previous and next ordering ids using the entity's ordering_id
    QString selectPreviousAndNextStr =
        QString("SELECT previous, next FROM %1 WHERE ordering_id = %2").arg(orderingTableName).arg(entityOrderingId);
    if (!query.exec(selectPreviousAndNextStr))
    {
        return Result<QPair<int, int>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), selectPreviousAndNextStr));
    }

    if (query.next())
    {
        QVariant previousValue = query.value(0);
        QVariant nextValue = query.value(1);

        previousRowId = previousValue.isNull() ? -1 : previousValue.toInt();
        nextRowId = nextValue.isNull() ? -1 : nextValue.toInt();
    }
    else
    {
        return Result<QPair<int, int>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", "Ordering not found for given entityId."));
    }

    return Result<QPair<int, int>>(QPair<int, int>(previousRowId, nextRowId));
}

template <class T> QString OrderedDatabaseTable<T>::orderingTableName() const
{
    return m_orderingTableName;
}

template <class T> Result<QList<QVariantMap>> OrderedDatabaseTable<T>::getOrderingTableData() const
{
    const QString &entityOrderingTableName = this->orderingTableName();
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    QSqlQuery query(database);

    QString queryStr = "WITH RECURSIVE ordered_entities AS ("
                       "  SELECT eo.*, 1 AS row_number"
                       "  FROM " +
                       entityOrderingTableName +
                       " eo"
                       "  WHERE eo.previous IS NULL"
                       "  UNION ALL"
                       "  SELECT eo.*, oe.row_number + 1 AS row_number"
                       "  FROM " +
                       entityOrderingTableName +
                       " eo"
                       "  JOIN ordered_entities oe ON eo.previous = oe.ordering_id"
                       ")"
                       " SELECT *"
                       " FROM ordered_entities"
                       " ORDER BY row_number ASC";

    if (!query.prepare(queryStr))
    {
        return Result<QList<QVariantMap>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    if (!query.exec())
    {
        return Result<QList<QVariantMap>>(
            Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
    }

    QList<QVariantMap> orderedTableData;
    QSqlRecord record;
    while (query.next())
    {
        record = query.record();
        QVariantMap rowData;
        for (int i = 0; i < record.count(); i++)
        {
            rowData.insert(record.fieldName(i), query.value(i));
        }
        orderedTableData.append(rowData);
    }
    return Result<QList<QVariantMap>>(orderedTableData);
}

template <class T> int OrderedDatabaseTable<T>::numberOfRows()
{
    const QString &entityOrderingTableName = this->orderingTableName();
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    QSqlQuery query(database);

    QString queryString = QString("SELECT COUNT(*) FROM %1").arg(entityOrderingTableName);
    query.exec(queryString);
    query.next();
    return query.value(0).toInt();
}

template <class T> Result<int> OrderedDatabaseTable<T>::getOrderingId(int entityId)
{
    QSqlDatabase database = DatabaseTable<T>::databaseContext()->getConnection();
    QSqlQuery query(database);

    QString queryString = QString("SELECT ordering_id FROM %1 WHERE id=:id").arg(this->tableName());
    if (!query.prepare(queryString))
    {
        return Result<int>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryString));
    }
    query.bindValue(":id", entityId);
    query.exec();

    int orderingId = -1;
    if (query.next())
    {
        orderingId = query.value(0).toInt();
    }
    return Result<int>(orderingId);
}

} // namespace Database

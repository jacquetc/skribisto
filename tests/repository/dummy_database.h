#pragma once

#include "author.h"
#include "database/interface_database_table.h"

using namespace Contracts::Database;

class DummyDatabase : public Contracts::Database::InterfaceDatabaseTable<Domain::Author>
{

  public:
    DummyDatabase()
    {
    }

    void fillGet(const Domain::Author &entity);
    void fillGetAll(const QList<Domain::Author> &list);
    void fillGetAllFiltered(const QList<Domain::Author> &list);
    void fillRemove(int id);
    void fillAdd(const Domain::Author &entity);
    void fillUpdate(const Domain::Author &entity);
    void fillExists(bool value);
    void fillSave(QList<QVariantHash> value);

    // InterfaceDatabaseTable interface

  public:
    Result<Domain::Author> get(const QUuid &uuid) override;
    Result<Domain::Author> get(const int &id) override;

    Result<QList<Domain::Author>> getAll() override;
    Result<QList<Domain::Author>> getAll(const QHash<QString, QVariant> &filters) override;
    Result<int> remove(int id) override;
    Result<Domain::Author> add(Domain::Author &&entity) override;
    Result<Domain::Author> update(Domain::Author &&entity) override;
    Result<bool> exists(const QUuid &uuid) override;
    Result<bool> exists(int id) override;
    Result<void> clear() override;
    Result<QMap<QString, QList<QVariantHash>>> save(const QList<int> &idList) override;
    Result<void> restore(const QMap<QString, QList<QVariantHash>> &rows) override;
    Result<void> beginTransaction() override;
    Result<void> commit() override;
    Result<void> rollback() override;

  private:
    Domain::Author m_getEntity;
    QList<Domain::Author> m_getAllList;
    QList<Domain::Author> m_getAllFilteredList;
    int m_removeEntityId;
    Domain::Author m_addEntity;
    Domain::Author m_updateEntity;
    bool m_exists;
    QMap<QString, QList<QVariantHash>> m_save;

    // InterfaceForeignEntity interface
  public:
    Result<QList<int>> getRelatedForeignIds(const Domain::Author &entity, const QString &propertyName) override;
    Result<QList<int>> getRelatedForeignIds(int entityId, const QString &propertyName) override;
};

inline Result<QList<int>> DummyDatabase::getRelatedForeignIds(const Domain::Author &entity, const QString &propertyName)
{
}

inline Result<QList<int>> DummyDatabase::getRelatedForeignIds(int entityId, const QString &propertyName)
{
}

inline void DummyDatabase::fillGet(const Domain::Author &entity)
{
    m_getEntity = entity;
}

inline void DummyDatabase::fillGetAll(const QList<Domain::Author> &list)
{
    m_getAllList = list;
}

inline void DummyDatabase::fillGetAllFiltered(const QList<Domain::Author> &list)
{
    m_getAllFilteredList = list;
}

inline void DummyDatabase::fillRemove(int entityId)
{
    m_removeEntityId = entityId;
}

inline void DummyDatabase::fillAdd(const Domain::Author &entity)
{
    m_addEntity = entity;
}

inline void DummyDatabase::fillUpdate(const Domain::Author &entity)
{
    m_updateEntity = entity;
}

inline void DummyDatabase::fillExists(bool value)
{
    m_exists = value;
}

inline Result<Domain::Author> DummyDatabase::get(const QUuid &uuid)
{
    return Result<Domain::Author>(m_getEntity);
}

inline Result<Domain::Author> DummyDatabase::get(const int &id)
{
    return Result<Domain::Author>(m_getEntity);
}

inline Result<QList<Domain::Author>> DummyDatabase::getAll()
{
    return Result<QList<Domain::Author>>(m_getAllList);
}

Result<QList<Domain::Author>> DummyDatabase::getAll(const QHash<QString, QVariant> &filters)
{
    return Result<QList<Domain::Author>>(m_getAllFilteredList);
}

inline Result<int> DummyDatabase::remove(int id)
{
    return Result<int>(m_removeEntityId);
}

inline Result<Domain::Author> DummyDatabase::add(Domain::Author &&entity)
{
    return Result<Domain::Author>(m_addEntity);
}

inline Result<Domain::Author> DummyDatabase::update(Domain::Author &&entity)
{
    return Result<Domain::Author>(m_updateEntity);
}

inline Result<bool> DummyDatabase::exists(const QUuid &uuid)
{
    return Result<bool>(m_exists);
}

inline Result<bool> DummyDatabase::exists(int id)
{
    return Result<bool>(m_exists);
}

inline Result<void> DummyDatabase::clear()
{
    return Result<void>();
}

inline Result<QMap<QString, QList<QVariantHash>>> DummyDatabase::save(const QList<int> &idList)
{
    return Result<QMap<QString, QList<QVariantHash>>>(m_save);
}

inline Result<void> DummyDatabase::restore(const QMap<QString, QList<QVariantHash>> &rows)
{
    return Result<void>();
}

inline Result<void> DummyDatabase::beginTransaction()
{
    return Result<void>();
}

inline Result<void> DummyDatabase::commit()
{
    return Result<void>();
}

inline Result<void> DummyDatabase::rollback()
{
    return Result<void>();
}

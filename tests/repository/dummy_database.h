#pragma once

#include "author.h"
#include "database/interface_database.h"

using namespace Contracts::Database;

class DummyDatabase : public Contracts::Database::InterfaceDatabase<Domain::Author>
{

  public:
    DummyDatabase()
    {
    }

    void fillGet(const Domain::Author &entity);
    void fillGetAll(const QList<Domain::Author> &list);
    void fillRemove(const Domain::Author &entity);
    void fillAdd(const Domain::Author &entity);
    void fillUpdate(const Domain::Author &entity);
    void fillExists(bool value);

    // InterfaceGenericRepository interface

  public:
    Result<Domain::Author> get(const QUuid &uuid) override;
    Result<QList<Domain::Author>> getAll() override;
    Result<Domain::Author> remove(Domain::Author &&entity) override;
    Result<Domain::Author> add(Domain::Author &&entity) override;
    Result<Domain::Author> update(Domain::Author &&entity) override;
    Result<bool> exists(const QUuid &uuid) override;

  private:
    Domain::Author m_getEntity;
    QList<Domain::Author> m_getAllList;
    Domain::Author m_removeEntity;
    Domain::Author m_addEntity;
    Domain::Author m_updateEntity;
    bool m_exists;
};

inline void DummyDatabase::fillGet(const Domain::Author &entity)
{
    m_getEntity = entity;
}

inline void DummyDatabase::fillGetAll(const QList<Domain::Author> &list)
{
    m_getAllList = list;
}

inline void DummyDatabase::fillRemove(const Domain::Author &entity)
{
    m_removeEntity = entity;
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

inline Result<QList<Domain::Author>> DummyDatabase::getAll()
{
    return Result<QList<Domain::Author>>(m_getAllList);
}

inline Result<Domain::Author> DummyDatabase::remove(Domain::Author &&entity)
{
    return Result<Domain::Author>(m_removeEntity);
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

#pragma once

#include "author.h"
#include "persistence/interface_author_repository.h"

#include <QObject>

class DummyAuthorRepository : public QObject, public Contracts::Persistence::InterfaceAuthorRepository
{
    Q_OBJECT

  public:
    DummyAuthorRepository(QObject *parent) : QObject{parent}, m_exists(true)
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

inline void DummyAuthorRepository::fillGet(const Domain::Author &entity)
{
    m_getEntity = entity;
}

inline void DummyAuthorRepository::fillGetAll(const QList<Domain::Author> &list)
{
    m_getAllList = list;
}

inline void DummyAuthorRepository::fillRemove(const Domain::Author &entity)
{
    m_removeEntity = entity;
}

inline void DummyAuthorRepository::fillAdd(const Domain::Author &entity)
{
    m_addEntity = entity;
}

inline void DummyAuthorRepository::fillUpdate(const Domain::Author &entity)
{
    m_updateEntity = entity;
}

inline void DummyAuthorRepository::fillExists(bool value)
{
    m_exists = value;
}

inline Result<Domain::Author> DummyAuthorRepository::get(const QUuid &uuid)
{

    int waitTime = 0;

    while (waitTime < 500000)
    {
        waitTime++;
    }
    return Result<Domain::Author>(m_getEntity);
}

inline Result<QList<Domain::Author>> DummyAuthorRepository::getAll()
{
    return Result<QList<Domain::Author>>(m_getAllList);
}

inline Result<Domain::Author> DummyAuthorRepository::remove(Domain::Author &&entity)
{
    return Result<Domain::Author>(m_removeEntity);
}

inline Result<Domain::Author> DummyAuthorRepository::add(Domain::Author &&entity)
{

    int waitTime = 0;

    while (waitTime < 500000000)
    {
        waitTime++;
    }

    return Result<Domain::Author>(m_addEntity);
}

inline Result<Domain::Author> DummyAuthorRepository::update(Domain::Author &&entity)
{
    return Result<Domain::Author>(m_updateEntity);
}

inline Result<bool> DummyAuthorRepository::exists(const QUuid &uuid)
{
    return Result<bool>(m_exists);
}

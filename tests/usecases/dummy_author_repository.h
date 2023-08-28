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
    void fillRemove(int id);
    void fillRemove(QList<int> ids);
    void fillAdd(const Domain::Author &entity);
    void fillUpdate(const Domain::Author &entity);
    void fillExists(bool value);
    void fillSave(const SaveData &save);

    // InterfaceGenericRepository interface

  public:
    Result<Domain::Author> get(const QUuid &uuid) override;
    Result<Domain::Author> get(const int &id) override;
    Result<QList<Domain::Author>> getAll() override;
    Result<QList<Domain::Author>> getAll(const QHash<QString, QVariant> &filters) override;
    Result<int> remove(int id) override;
    Result<QList<int>> remove(QList<int> ids) override;
    Result<Domain::Author> add(Domain::Author &&entity) override;
    Result<Domain::Author> update(Domain::Author &&entity) override;
    Result<bool> exists(const QUuid &uuid) override;
    Result<bool> exists(int id) override;
    Result<void> clear() override;
    Result<SaveData> save(const QList<int> &idList) override;
    Result<void> restore(const SaveData &saveData) override;
    Result<void> beginChanges() override;
    Result<void> saveChanges() override;
    Result<void> cancelChanges() override;

  private:
    Domain::Author m_getEntity;
    QList<Domain::Author> m_getAllList;
    int m_removeEntityId;
    QList<int> m_removeEntityIds;
    Domain::Author m_addEntity;
    Domain::Author m_updateEntity;
    bool m_exists;
    SaveData m_save;

    // InterfaceAuthorRepository interface
  public:
    QHash<int, QList<int>> removeInCascade(QList<int> ids) override;
};

inline QHash<int, QList<int>> DummyAuthorRepository::removeInCascade(QList<int> ids)
{
    return QHash<int, QList<int>>();
}

inline void DummyAuthorRepository::fillGet(const Domain::Author &entity)
{
    m_getEntity = entity;
}

inline void DummyAuthorRepository::fillGetAll(const QList<Domain::Author> &list)
{
    m_getAllList = list;
}

inline void DummyAuthorRepository::fillRemove(int id)
{
    m_removeEntityId = id;
}

inline void DummyAuthorRepository::fillRemove(QList<int> entityIds)
{
    m_removeEntityIds = entityIds;
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

inline void DummyAuthorRepository::fillSave(const SaveData &save)
{
    m_save = save;
}

inline Result<Domain::Author> DummyAuthorRepository::get(const QUuid &uuid)
{
    return Result<Domain::Author>(m_getEntity);
}

inline Result<Domain::Author> DummyAuthorRepository::get(const int &id)
{
    return Result<Domain::Author>(m_getEntity);
}

inline Result<QList<Domain::Author>> DummyAuthorRepository::getAll()
{
    return Result<QList<Domain::Author>>(m_getAllList);
}

inline Result<QList<Domain::Author>> DummyAuthorRepository::getAll(const QHash<QString, QVariant> &filters)
{
    return Result<QList<Domain::Author>>(m_getAllList);
}

inline Result<int> DummyAuthorRepository::remove(int id)
{
    return Result<int>(m_removeEntityId);
}

inline Result<QList<int>> DummyAuthorRepository::remove(QList<int> ids)
{

    return Result<QList<int>>(m_removeEntityIds);
}

inline Result<Domain::Author> DummyAuthorRepository::add(Domain::Author &&entity)
{
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

inline Result<bool> DummyAuthorRepository::exists(int id)
{
    return Result<bool>(m_exists);
}

inline Result<void> DummyAuthorRepository::clear()
{
    return Result<void>();
}

inline Result<SaveData> DummyAuthorRepository::save(const QList<int> &idList)
{
    return Result<SaveData>(m_save);
}

inline Result<void> DummyAuthorRepository::restore(const SaveData &saveData)
{
    return Result<void>();
}

inline Result<void> DummyAuthorRepository::beginChanges()
{
    return Result<void>();
}

inline Result<void> DummyAuthorRepository::saveChanges()
{
    return Result<void>();
}

inline Result<void> DummyAuthorRepository::cancelChanges()
{
    return Result<void>();
}

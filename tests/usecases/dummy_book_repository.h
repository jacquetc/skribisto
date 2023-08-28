#pragma once

#include "book.h"
#include "persistence/interface_book_repository.h"

#include <QObject>

class DummyBookRepository : public QObject, public Contracts::Persistence::InterfaceBookRepository
{
    Q_OBJECT

  public:
    DummyBookRepository(QObject *parent) : QObject{parent}, m_exists(true)
    {
    }

    void fillGet(const Domain::Book &entity);
    void fillGetAll(const QList<Domain::Book> &list);
    void fillRemove(int id);
    void fillRemove(QList<int> ids);
    void fillAdd(const Domain::Book &entity);
    void fillUpdate(const Domain::Book &entity);
    void fillExists(bool value);
    void fillSave(const SaveData &save);
    void fillAuthor(const Domain::Author &author);

    // InterfaceGenericRepository interface

  public:
    Result<Domain::Book> get(const QUuid &uuid) override;
    Result<Domain::Book> get(const int &id) override;
    Result<QList<Domain::Book>> getAll() override;
    Result<QList<Domain::Book>> getAll(const QHash<QString, QVariant> &filters) override;
    Result<int> remove(int id) override;
    Result<QList<int>> remove(QList<int> ids) override;
    Result<Domain::Book> add(Domain::Book &&entity) override;
    Result<Domain::Book> update(Domain::Book &&entity) override;
    Result<bool> exists(const QUuid &uuid) override;
    Result<bool> exists(int id) override;
    Result<void> clear() override;
    Result<SaveData> save(const QList<int> &idList) override;
    Result<void> restore(const SaveData &saveData) override;
    Result<void> beginChanges() override;
    Result<void> saveChanges() override;
    Result<void> cancelChanges() override;

  private:
    Domain::Book m_getEntity;
    QList<Domain::Book> m_getAllList;
    int m_removeEntityId;
    QList<int> m_removeEntityIds;
    QList<Domain::Chapter> m_chaptersList;
    Domain::Author m_author;
    Domain::Book m_addEntity;
    Domain::Book m_updateEntity;
    bool m_exists;
    SaveData m_save;

    // InterfaceBookRepository interface
  public:
    QHash<int, QList<int>> removeInCascade(QList<int> ids) override;

    // InterfaceBookRepository interface
  public:
    Domain::Book::ChaptersLoader fetchChaptersLoader() override;
    Domain::Book::AuthorLoader fetchAuthorLoader() override;
};

inline Domain::Book::ChaptersLoader DummyBookRepository::fetchChaptersLoader()
{
    return [=](int entityId) { return QList<Domain::Chapter>(m_chaptersList); };
}

inline Domain::Book::AuthorLoader DummyBookRepository::fetchAuthorLoader()
{
    return [=](int entityId) { return m_author; };
}

inline QHash<int, QList<int>> DummyBookRepository::removeInCascade(QList<int> ids)
{
    return QHash<int, QList<int>>();
}

inline void DummyBookRepository::fillGet(const Domain::Book &entity)
{
    m_getEntity = entity;
}

inline void DummyBookRepository::fillGetAll(const QList<Domain::Book> &list)
{
    m_getAllList = list;
}

inline void DummyBookRepository::fillRemove(int id)
{
    m_removeEntityId = id;
}

inline void DummyBookRepository::fillRemove(QList<int> entityIds)
{
    m_removeEntityIds = entityIds;
}

inline void DummyBookRepository::fillAdd(const Domain::Book &entity)
{
    m_addEntity = entity;
}

inline void DummyBookRepository::fillUpdate(const Domain::Book &entity)
{
    m_updateEntity = entity;
}

inline void DummyBookRepository::fillExists(bool value)
{
    m_exists = value;
}

inline void DummyBookRepository::fillSave(const SaveData &save)
{
    m_save = save;
}

inline void DummyBookRepository::fillAuthor(const Domain::Author &author)
{
    m_author = author;
}

inline Result<Domain::Book> DummyBookRepository::get(const QUuid &uuid)
{
    return Result<Domain::Book>(m_getEntity);
}

inline Result<Domain::Book> DummyBookRepository::get(const int &id)
{
    return Result<Domain::Book>(m_getEntity);
}

inline Result<QList<Domain::Book>> DummyBookRepository::getAll()
{
    return Result<QList<Domain::Book>>(m_getAllList);
}

inline Result<QList<Domain::Book>> DummyBookRepository::getAll(const QHash<QString, QVariant> &filters)
{
    return Result<QList<Domain::Book>>(m_getAllList);
}

inline Result<int> DummyBookRepository::remove(int id)
{
    return Result<int>(m_removeEntityId);
}

inline Result<QList<int>> DummyBookRepository::remove(QList<int> ids)
{

    return Result<QList<int>>(m_removeEntityIds);
}

inline Result<Domain::Book> DummyBookRepository::add(Domain::Book &&entity)
{
    return Result<Domain::Book>(m_addEntity);
}

inline Result<Domain::Book> DummyBookRepository::update(Domain::Book &&entity)
{
    return Result<Domain::Book>(m_updateEntity);
}

inline Result<bool> DummyBookRepository::exists(const QUuid &uuid)
{
    return Result<bool>(m_exists);
}

inline Result<bool> DummyBookRepository::exists(int id)
{
    return Result<bool>(m_exists);
}

inline Result<void> DummyBookRepository::clear()
{
    return Result<void>();
}

inline Result<SaveData> DummyBookRepository::save(const QList<int> &idList)
{
    return Result<SaveData>(m_save);
}

inline Result<void> DummyBookRepository::restore(const SaveData &saveData)
{
    return Result<void>();
}

inline Result<void> DummyBookRepository::beginChanges()
{
    return Result<void>();
}

inline Result<void> DummyBookRepository::saveChanges()
{
    return Result<void>();
}

inline Result<void> DummyBookRepository::cancelChanges()
{
    return Result<void>();
}

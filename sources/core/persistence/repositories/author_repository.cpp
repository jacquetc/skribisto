#include "author_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Repository;
using namespace Contracts::Persistence;

AuthorRepository::AuthorRepository(Domain::EntitySchema *entitySchema,
                                   InterfaceDatabaseTable<Domain::Author> *authorDatabase)
    : m_entitySchema(entitySchema), Repository::GenericRepository<Domain::Author>(authorDatabase)
{
}

QHash<int, QList<int>> AuthorRepository::removeInCascade(QList<int> ids)
{
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // finally remove the entites of this repository

    Result<QList<int>> removedIds = this->databaseTable()->remove(ids);
    returnedHashOfEntityWithRemovedIds.insert(Domain::Entities::Author, removedIds.value());

    return returnedHashOfEntityWithRemovedIds;
}
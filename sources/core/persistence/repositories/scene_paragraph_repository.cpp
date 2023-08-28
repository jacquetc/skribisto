#include "scene_paragraph_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Repository;
using namespace Contracts::Persistence;

SceneParagraphRepository::SceneParagraphRepository(
    Domain::EntitySchema *entitySchema, InterfaceDatabaseTable<Domain::SceneParagraph> *sceneParagraphDatabase)
    : m_entitySchema(entitySchema), Repository::GenericRepository<Domain::SceneParagraph>(sceneParagraphDatabase)
{
}

QHash<int, QList<int>> SceneParagraphRepository::removeInCascade(QList<int> ids)
{
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // finally remove the entites of this repository

    Result<QList<int>> removedIds = this->databaseTable()->remove(ids);
    returnedHashOfEntityWithRemovedIds.insert(Domain::Entities::SceneParagraph, removedIds.value());

    return returnedHashOfEntityWithRemovedIds;
}
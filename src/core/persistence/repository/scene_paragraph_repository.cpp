// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "scene_paragraph_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Persistence::Repository;
using namespace Skribisto::Contracts::Repository;

SceneParagraphRepository::SceneParagraphRepository(
    InterfaceDatabaseTableGroup<Skribisto::Domain::SceneParagraph> *sceneParagraphDatabase)
    : Qleany::Repository::GenericRepository<Skribisto::Domain::SceneParagraph>(sceneParagraphDatabase)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *SceneParagraphRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<QHash<int, QList<int>>> SceneParagraphRepository::removeInCascade(QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // finally remove the entites of this repository

    Result<void> associationRemovalResult = this->databaseTable()->removeAssociationsWith(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, associationRemovalResult)
    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removedIdsResult)

    returnedHashOfEntityWithRemovedIds.insert(Skribisto::Domain::Entities::SceneParagraph, removedIdsResult.value());

    emit m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<int, QList<int>>> SceneParagraphRepository::changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(Skribisto::Domain::Entities::SceneParagraph,
                                                    changedIdsResult.value());
    emit m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "workspace_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Persistence::Repository;
using namespace Skribisto::Contracts::Repository;

WorkspaceRepository::WorkspaceRepository(InterfaceDatabaseTableGroup<Skribisto::Domain::Workspace> *workspaceDatabase,
                                         InterfaceFileRepository *fileRepository)
    : Qleany::Repository::GenericRepository<Skribisto::Domain::Workspace>(workspaceDatabase),
      m_fileRepository(fileRepository)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *WorkspaceRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<Skribisto::Domain::Workspace> WorkspaceRepository::update(Domain::Workspace &&entity)
{
    QWriteLocker locker(&m_lock);

    if (entity.metaData().filesSet)
    {

        Result<QList<Domain::File>> filesResult = m_fileRepository->updateEntitiesInRelationOf(
            Domain::Workspace::schema, entity.id(), "files", entity.files());

#ifdef QT_DEBUG
        if (filesResult.isError())
        {
            qCritical() << filesResult.error().code() << filesResult.error().message() << filesResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Domain::Workspace, filesResult)
    }

    return Qleany::Repository::GenericRepository<Domain::Workspace>::update(std::move(entity));
}

Result<Skribisto::Domain::Workspace> WorkspaceRepository::getWithDetails(int entityId)
{
    QWriteLocker locker(&m_lock);
    auto getResult = Qleany::Repository::GenericRepository<Domain::Workspace>::get(entityId);

    if (getResult.isError())
    {
        return getResult;
    }

    Domain::Workspace entity = getResult.value();

    Result<QList<Domain::File>> filesResult =
        m_fileRepository->getEntitiesInRelationOf(Domain::Workspace::schema, entity.id(), "files");

#ifdef QT_DEBUG
    if (filesResult.isError())
    {
        qCritical() << filesResult.error().code() << filesResult.error().message() << filesResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Domain::Workspace, filesResult)

    entity.setFiles(filesResult.value());

    return Result<Domain::Workspace>(entity);
}

Skribisto::Domain::Workspace::FilesLoader WorkspaceRepository::fetchFilesLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "files" property in the entity Workspace using staticMetaObject
    int propertyIndex = Skribisto::Domain::Workspace::staticMetaObject.indexOfProperty("files");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Workspace doesn't have a property named files";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntitiesResult =
            m_fileRepository->getEntitiesInRelationOf(Skribisto::Domain::Workspace::schema, entityId, "files");

        if (foreignEntitiesResult.isError())
        {
            qCritical() << foreignEntitiesResult.error().code() << foreignEntitiesResult.error().message()
                        << foreignEntitiesResult.error().data();
            return QList<Skribisto::Domain::File>();
        }

        return foreignEntitiesResult.value();
    };
}

Result<QHash<int, QList<int>>> WorkspaceRepository::removeInCascade(QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the files in cascade

    Qleany::Domain::RelationshipInfo fileFilesRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Skribisto::Domain::Workspace::schema.relationships)
    {
        if (relationship.rightEntityId == Skribisto::Domain::Entities::File && relationship.fieldName == "files")
        {
            fileFilesRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (fileFilesRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<Skribisto::Domain::File> foreignFiles = this->fetchFilesLoader().operator()(entityId);

            if (foreignFiles.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &file : foreignFiles)
            {
                foreignIds.append(file.id());
            }

            auto removalResult = m_fileRepository->removeInCascade(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // finally remove the entites of this repository

    Result<void> associationRemovalResult = this->databaseTable()->removeAssociationsWith(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, associationRemovalResult)
    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removedIdsResult)

    returnedHashOfEntityWithRemovedIds.insert(Skribisto::Domain::Entities::Workspace, removedIdsResult.value());

    emit m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<int, QList<int>>> WorkspaceRepository::changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // cahnge active status of the files in cascade

    Qleany::Domain::RelationshipInfo fileFilesRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Skribisto::Domain::Workspace::schema.relationships)
    {
        if (relationship.rightEntityId == Skribisto::Domain::Entities::File && relationship.fieldName == "files")
        {
            fileFilesRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (fileFilesRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<Skribisto::Domain::File> foreignFiles = this->fetchFilesLoader().operator()(entityId);

            if (foreignFiles.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &file : foreignFiles)
            {
                foreignIds.append(file.id());
            }

            auto changeResult = m_fileRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(Skribisto::Domain::Entities::Workspace, changedIdsResult.value());
    emit m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}
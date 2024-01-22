// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "atelier_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Persistence::Repository;
using namespace Skribisto::Contracts::Repository;

AtelierRepository::AtelierRepository(InterfaceDatabaseTableGroup<Skribisto::Domain::Atelier> *atelierDatabase,
                                     InterfaceGitRepository *gitRepository,
                                     InterfaceWorkspaceRepository *workspaceRepository)
    : Qleany::Repository::GenericRepository<Skribisto::Domain::Atelier>(atelierDatabase),
      m_gitRepository(gitRepository), m_workspaceRepository(workspaceRepository)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *AtelierRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<Skribisto::Domain::Atelier> AtelierRepository::update(Domain::Atelier &&entity)
{
    QWriteLocker locker(&m_lock);

    if (entity.metaData().gitSet)
    {

        Result<Domain::Git> gitResult =
            m_gitRepository->updateEntityInRelationOf(Domain::Atelier::schema, entity.id(), "git", entity.git());

#ifdef QT_DEBUG
        if (gitResult.isError())
        {
            qCritical() << gitResult.error().code() << gitResult.error().message() << gitResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Domain::Atelier, gitResult)
    }

    if (entity.metaData().workspacesSet)
    {

        Result<QList<Domain::Workspace>> workspacesResult = m_workspaceRepository->updateEntitiesInRelationOf(
            Domain::Atelier::schema, entity.id(), "workspaces", entity.workspaces());

#ifdef QT_DEBUG
        if (workspacesResult.isError())
        {
            qCritical() << workspacesResult.error().code() << workspacesResult.error().message()
                        << workspacesResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Domain::Atelier, workspacesResult)
    }

    return Qleany::Repository::GenericRepository<Domain::Atelier>::update(std::move(entity));
}

Result<Skribisto::Domain::Atelier> AtelierRepository::getWithDetails(int entityId)
{
    QWriteLocker locker(&m_lock);
    auto getResult = Qleany::Repository::GenericRepository<Domain::Atelier>::get(entityId);

    if (getResult.isError())
    {
        return getResult;
    }

    Domain::Atelier entity = getResult.value();

    Result<Domain::Git> gitResult = m_gitRepository->getEntityInRelationOf(Domain::Atelier::schema, entity.id(), "git");

#ifdef QT_DEBUG
    if (gitResult.isError())
    {
        qCritical() << gitResult.error().code() << gitResult.error().message() << gitResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Domain::Atelier, gitResult)

    entity.setGit(gitResult.value());

    Result<QList<Domain::Workspace>> workspacesResult =
        m_workspaceRepository->getEntitiesInRelationOf(Domain::Atelier::schema, entity.id(), "workspaces");

#ifdef QT_DEBUG
    if (workspacesResult.isError())
    {
        qCritical() << workspacesResult.error().code() << workspacesResult.error().message()
                    << workspacesResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Domain::Atelier, workspacesResult)

    entity.setWorkspaces(workspacesResult.value());

    return Result<Domain::Atelier>(entity);
}

Skribisto::Domain::Atelier::GitLoader AtelierRepository::fetchGitLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "git" property in the entity Atelier using staticMetaObject
    int propertyIndex = Skribisto::Domain::Atelier::staticMetaObject.indexOfProperty("git");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Atelier doesn't have a property named git";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntityResult =
            m_gitRepository->getEntityInRelationOf(Skribisto::Domain::Atelier::schema, entityId, "git");

        if (foreignEntityResult.isError())
        {
            qCritical() << foreignEntityResult.error().code() << foreignEntityResult.error().message()
                        << foreignEntityResult.error().data();
            return Skribisto::Domain::Git();
        }

        return foreignEntityResult.value();
    };
}

Skribisto::Domain::Atelier::WorkspacesLoader AtelierRepository::fetchWorkspacesLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "workspaces" property in the entity Atelier using staticMetaObject
    int propertyIndex = Skribisto::Domain::Atelier::staticMetaObject.indexOfProperty("workspaces");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Atelier doesn't have a property named workspaces";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntitiesResult =
            m_workspaceRepository->getEntitiesInRelationOf(Skribisto::Domain::Atelier::schema, entityId, "workspaces");

        if (foreignEntitiesResult.isError())
        {
            qCritical() << foreignEntitiesResult.error().code() << foreignEntitiesResult.error().message()
                        << foreignEntitiesResult.error().data();
            return QList<Skribisto::Domain::Workspace>();
        }

        return foreignEntitiesResult.value();
    };
}

Result<QHash<int, QList<int>>> AtelierRepository::removeInCascade(QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the git in cascade

    Qleany::Domain::RelationshipInfo gitGitRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Skribisto::Domain::Atelier::schema.relationships)
    {
        if (relationship.rightEntityId == Skribisto::Domain::Entities::Git && relationship.fieldName == "git")
        {
            gitGitRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (gitGitRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            Skribisto::Domain::Git foreignGit = this->fetchGitLoader().operator()(entityId);

            if (!foreignGit.isValid())
            {
                continue;
            }

            QList<int> foreignIds;

            foreignIds.append(foreignGit.id());

            auto removalResult = m_gitRepository->removeInCascade(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // remove the workspaces in cascade

    Qleany::Domain::RelationshipInfo workspaceWorkspacesRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Skribisto::Domain::Atelier::schema.relationships)
    {
        if (relationship.rightEntityId == Skribisto::Domain::Entities::Workspace &&
            relationship.fieldName == "workspaces")
        {
            workspaceWorkspacesRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (workspaceWorkspacesRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<Skribisto::Domain::Workspace> foreignWorkspaces = this->fetchWorkspacesLoader().operator()(entityId);

            if (foreignWorkspaces.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &workspace : foreignWorkspaces)
            {
                foreignIds.append(workspace.id());
            }

            auto removalResult = m_workspaceRepository->removeInCascade(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // finally remove the entites of this repository

    Result<void> associationRemovalResult = this->databaseTable()->removeAssociationsWith(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, associationRemovalResult)
    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removedIdsResult)

    returnedHashOfEntityWithRemovedIds.insert(Skribisto::Domain::Entities::Atelier, removedIdsResult.value());

    emit m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<int, QList<int>>> AtelierRepository::changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // cahnge active status of the git in cascade

    Qleany::Domain::RelationshipInfo gitGitRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Skribisto::Domain::Atelier::schema.relationships)
    {
        if (relationship.rightEntityId == Skribisto::Domain::Entities::Git && relationship.fieldName == "git")
        {
            gitGitRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (gitGitRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            Skribisto::Domain::Git foreignGit = this->fetchGitLoader().operator()(entityId);

            if (!foreignGit.isValid())
            {
                continue;
            }

            QList<int> foreignIds;

            foreignIds.append(foreignGit.id());

            auto changeResult = m_gitRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // cahnge active status of the workspaces in cascade

    Qleany::Domain::RelationshipInfo workspaceWorkspacesRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Skribisto::Domain::Atelier::schema.relationships)
    {
        if (relationship.rightEntityId == Skribisto::Domain::Entities::Workspace &&
            relationship.fieldName == "workspaces")
        {
            workspaceWorkspacesRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (workspaceWorkspacesRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<Skribisto::Domain::Workspace> foreignWorkspaces = this->fetchWorkspacesLoader().operator()(entityId);

            if (foreignWorkspaces.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &workspace : foreignWorkspaces)
            {
                foreignIds.append(workspace.id());
            }

            auto changeResult = m_workspaceRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(Skribisto::Domain::Entities::Atelier, changedIdsResult.value());
    emit m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}
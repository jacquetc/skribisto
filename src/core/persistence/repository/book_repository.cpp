// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "book_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Persistence::Repository;
using namespace Skribisto::Contracts::Repository;

BookRepository::BookRepository(InterfaceDatabaseTableGroup<Skribisto::Domain::Book> *bookDatabase,
                               InterfaceChapterRepository *chapterRepository)
    : Qleany::Repository::GenericRepository<Skribisto::Domain::Book>(bookDatabase),
      m_chapterRepository(chapterRepository)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *BookRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<Skribisto::Domain::Book> BookRepository::update(Domain::Book &&entity)
{
    QWriteLocker locker(&m_lock);

    if (entity.metaData().chaptersSet)
    {

        Result<QList<Domain::Chapter>> chaptersResult = m_chapterRepository->updateEntitiesInRelationOf(
            Domain::Book::schema, entity.id(), "chapters", entity.chapters());

#ifdef QT_DEBUG
        if (chaptersResult.isError())
        {
            qCritical() << chaptersResult.error().code() << chaptersResult.error().message()
                        << chaptersResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Domain::Book, chaptersResult)
    }

    return Qleany::Repository::GenericRepository<Domain::Book>::update(std::move(entity));
}

Result<Skribisto::Domain::Book> BookRepository::getWithDetails(int entityId)
{
    QWriteLocker locker(&m_lock);
    auto getResult = Qleany::Repository::GenericRepository<Domain::Book>::get(entityId);

    if (getResult.isError())
    {
        return getResult;
    }

    Domain::Book entity = getResult.value();

    Result<QList<Domain::Chapter>> chaptersResult =
        m_chapterRepository->getEntitiesInRelationOf(Domain::Book::schema, entity.id(), "chapters");

#ifdef QT_DEBUG
    if (chaptersResult.isError())
    {
        qCritical() << chaptersResult.error().code() << chaptersResult.error().message()
                    << chaptersResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Domain::Book, chaptersResult)

    entity.setChapters(chaptersResult.value());

    return Result<Domain::Book>(entity);
}

Skribisto::Domain::Book::ChaptersLoader BookRepository::fetchChaptersLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "chapters" property in the entity Book using staticMetaObject
    int propertyIndex = Skribisto::Domain::Book::staticMetaObject.indexOfProperty("chapters");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Book doesn't have a property named chapters";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntitiesResult =
            m_chapterRepository->getEntitiesInRelationOf(Skribisto::Domain::Book::schema, entityId, "chapters");

        if (foreignEntitiesResult.isError())
        {
            qCritical() << foreignEntitiesResult.error().code() << foreignEntitiesResult.error().message()
                        << foreignEntitiesResult.error().data();
            return QList<Skribisto::Domain::Chapter>();
        }

        return foreignEntitiesResult.value();
    };
}

Result<QHash<int, QList<int>>> BookRepository::removeInCascade(QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the chapters in cascade

    Qleany::Domain::RelationshipInfo chapterChaptersRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Skribisto::Domain::Book::schema.relationships)
    {
        if (relationship.rightEntityId == Skribisto::Domain::Entities::Chapter && relationship.fieldName == "chapters")
        {
            chapterChaptersRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (chapterChaptersRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<Skribisto::Domain::Chapter> foreignChapters = this->fetchChaptersLoader().operator()(entityId);

            if (foreignChapters.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &chapter : foreignChapters)
            {
                foreignIds.append(chapter.id());
            }

            auto removalResult = m_chapterRepository->removeInCascade(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // finally remove the entites of this repository

    Result<void> associationRemovalResult = this->databaseTable()->removeAssociationsWith(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, associationRemovalResult)
    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removedIdsResult)

    returnedHashOfEntityWithRemovedIds.insert(Skribisto::Domain::Entities::Book, removedIdsResult.value());

    emit m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<int, QList<int>>> BookRepository::changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // cahnge active status of the chapters in cascade

    Qleany::Domain::RelationshipInfo chapterChaptersRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Skribisto::Domain::Book::schema.relationships)
    {
        if (relationship.rightEntityId == Skribisto::Domain::Entities::Chapter && relationship.fieldName == "chapters")
        {
            chapterChaptersRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (chapterChaptersRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<Skribisto::Domain::Chapter> foreignChapters = this->fetchChaptersLoader().operator()(entityId);

            if (foreignChapters.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &chapter : foreignChapters)
            {
                foreignIds.append(chapter.id());
            }

            auto changeResult = m_chapterRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(Skribisto::Domain::Entities::Book, changedIdsResult.value());
    emit m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}
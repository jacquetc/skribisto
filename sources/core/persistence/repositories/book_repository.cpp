#include "book_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Repository;
using namespace Contracts::Persistence;

BookRepository::BookRepository(Domain::EntitySchema *entitySchema, InterfaceDatabaseTable<Domain::Book> *bookDatabase,
                               InterfaceChapterRepository *chapterRepository,
                               InterfaceAuthorRepository *authorRepository)
    : m_entitySchema(entitySchema), Repository::GenericRepository<Domain::Book>(bookDatabase),
      m_chapterRepository(chapterRepository), m_authorRepository(authorRepository)
{
}

Domain::Book::ChaptersLoader BookRepository::fetchChaptersLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "chapters" property in the entity Book using staticMetaObject
    int propertyIndex = Domain::Book::staticMetaObject.indexOfProperty("chapters");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Book doesn't have a property named chapters";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto result = this->databaseTable()->getRelatedForeignIds(entityId, "chapters");
        if (result.isError())
        {
            qCritical() << result.error().code() << result.error().message() << result.error().data();

            return QList<Domain::Chapter>();
        }

        QList<int> foreignIds = result.value();

        QList<Domain::Chapter> foreignEntities;
        for (int foreignId : foreignIds)
        {
            foreignEntities.append(m_chapterRepository->get(foreignId).value());
        }
        return foreignEntities;
    };
}

Domain::Book::AuthorLoader BookRepository::fetchAuthorLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "author" property in the entity Book using staticMetaObject
    int propertyIndex = Domain::Book::staticMetaObject.indexOfProperty("author");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Book doesn't have a property named author";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto result = this->databaseTable()->getRelatedForeignIds(entityId, "author");
        if (result.isError())
        {
            qCritical() << result.error().code() << result.error().message() << result.error().data();

            return Domain::Author();
        }

        QList<int> foreignIds = result.value();

        Domain::Author foreignEntity;
        if (foreignIds.size() > 0)
        {
            foreignEntity = m_authorRepository->get(foreignIds[0]).value();
        }
        return foreignEntity;
    };
}

QHash<int, QList<int>> BookRepository::removeInCascade(QList<int> ids)
{
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the chapters in cascade

    Domain::EntitySchema::EntitySchemaRelationship chapterRelationship =
        m_entitySchema->relationship(Domain::Book::enumValue(), "chapters");

    for (int entityId : ids)
    {
        if (chapterRelationship.dependency == Domain::EntitySchema::Strong)
        {

            QList<Domain::Chapter> foreignChapters = this->fetchChaptersLoader().operator()(entityId);

            QList<int> foreignIds;

            for (const auto &chapter : foreignChapters)
            {
                foreignIds.append(chapter.id());
            }

            returnedHashOfEntityWithRemovedIds.insert(m_chapterRepository->removeInCascade(foreignIds));
        }
    }

    // remove the author in cascade

    Domain::EntitySchema::EntitySchemaRelationship authorRelationship =
        m_entitySchema->relationship(Domain::Book::enumValue(), "author");

    for (int entityId : ids)
    {
        if (authorRelationship.dependency == Domain::EntitySchema::Strong)
        {

            Domain::Author foreignAuthor = this->fetchAuthorLoader().operator()(entityId);

            QList<int> foreignIds;

            foreignIds.append(foreignAuthor.id());

            returnedHashOfEntityWithRemovedIds.insert(m_authorRepository->removeInCascade(foreignIds));
        }
    }

    // finally remove the entites of this repository

    Result<QList<int>> removedIds = this->databaseTable()->remove(ids);
    returnedHashOfEntityWithRemovedIds.insert(Domain::Entities::Book, removedIds.value());

    return returnedHashOfEntityWithRemovedIds;
}
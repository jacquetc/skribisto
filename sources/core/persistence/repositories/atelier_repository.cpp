#include "atelier_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Repository;
using namespace Contracts::Persistence;

AtelierRepository::AtelierRepository(Domain::EntitySchema *entitySchema,
                                     InterfaceDatabaseTable<Domain::Atelier> *atelierDatabase,
                                     InterfaceBookRepository *bookRepository)
    : m_entitySchema(entitySchema), Repository::GenericRepository<Domain::Atelier>(atelierDatabase),
      m_bookRepository(bookRepository)
{
}

Domain::Atelier::BooksLoader AtelierRepository::fetchBooksLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "books" property in the entity Atelier using staticMetaObject
    int propertyIndex = Domain::Atelier::staticMetaObject.indexOfProperty("books");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Atelier doesn't have a property named books";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto result = this->databaseTable()->getRelatedForeignIds(entityId, "books");
        if (result.isError())
        {
            qCritical() << result.error().code() << result.error().message() << result.error().data();

            return QList<Domain::Book>();
        }

        QList<int> foreignIds = result.value();

        QList<Domain::Book> foreignEntities;
        for (int foreignId : foreignIds)
        {
            foreignEntities.append(m_bookRepository->get(foreignId).value());
        }
        return foreignEntities;
    };
}

QHash<int, QList<int>> AtelierRepository::removeInCascade(QList<int> ids)
{
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the books in cascade

    Domain::EntitySchema::EntitySchemaRelationship bookRelationship =
        m_entitySchema->relationship(Domain::Atelier::enumValue(), "books");

    for (int entityId : ids)
    {
        if (bookRelationship.dependency == Domain::EntitySchema::Strong)
        {

            QList<Domain::Book> foreignBooks = this->fetchBooksLoader().operator()(entityId);

            QList<int> foreignIds;

            for (const auto &book : foreignBooks)
            {
                foreignIds.append(book.id());
            }

            returnedHashOfEntityWithRemovedIds.insert(m_bookRepository->removeInCascade(foreignIds));
        }
    }

    // finally remove the entites of this repository

    Result<QList<int>> removedIds = this->databaseTable()->remove(ids);
    returnedHashOfEntityWithRemovedIds.insert(Domain::Entities::Atelier, removedIds.value());

    return returnedHashOfEntityWithRemovedIds;
}
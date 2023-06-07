#include "book_repository.h"

using namespace Repository;

BookRepository::BookRepository(InterfaceDatabaseTable<Domain::Book> *bookDatabase, InterfaceDatabaseTable<Domain::Chapter> *chapterDatabase)
    : QObject(nullptr), Repository::GenericRepository<Domain::Book>(bookDatabase), m_chapterDatabase(chapterDatabase)
{
}


Domain::Book::ChaptersLoader BookRepository::fetchChaptersLoader()
{
    return [this](int entityId) {
        auto result = m_bookDatabase->getRelatedForeignIds(entityId, "chapters");
        if (result.isError())
        {
            qCritical() << result.error().code() << result.error().message() << result.error().data();
            
            return QList<Domain::Chapter>();
            
        }

        QList<int> foreignIds = result.value();
           
        QList<Domain::Chapter> foreignEntities;
        for (int foreignId : foreignIds)
        {
            foreignEntities.append(m_chapterDatabase->get(foreignId).value());
        }
        return foreignEntities;
        

    };
}




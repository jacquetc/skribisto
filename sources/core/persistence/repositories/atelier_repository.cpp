#include "atelier_repository.h"

using namespace Repository;

AtelierRepository::AtelierRepository(InterfaceDatabaseTable<Domain::Atelier> *atelierDatabase, InterfaceDatabaseTable<Domain::Book> *bookDatabase)
    : QObject(nullptr), Repository::GenericRepository<Domain::Atelier>(atelierDatabase), m_bookDatabase(bookDatabase)
{
}


Domain::Atelier::BooksLoader AtelierRepository::fetchBooksLoader()
{
    return [this](int entityId) {
        auto result = m_atelierDatabase->getRelatedForeignIds(entityId, "books");
        if (result.isError())
        {
            qCritical() << result.error().code() << result.error().message() << result.error().data();
            
            return QList<Domain::Book>();
            
        }

        QList<int> foreignIds = result.value();
           
        QList<Domain::Book> foreignEntities;
        for (int foreignId : foreignIds)
        {
            foreignEntities.append(m_bookDatabase->get(foreignId).value());
        }
        return foreignEntities;
        

    };
}




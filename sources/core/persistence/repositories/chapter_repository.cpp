#include "chapter_repository.h"

using namespace Repository;

ChapterRepository::ChapterRepository(InterfaceDatabaseTable<Domain::Chapter> *database)
    : QObject(nullptr), Repository::GenericRepository<Domain::Chapter>(database)
{
}

#include "chapter_repository.h"

using namespace Repository;

ChapterRepository::ChapterRepository(InterfaceDatabaseTable<Domain::Chapter> *chapterDatabase)
    : QObject(nullptr), Repository::GenericRepository<Domain::Chapter>(chapterDatabase)
{
}





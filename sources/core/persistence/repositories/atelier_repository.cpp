#include "atelier_repository.h"

using namespace Repository;

AtelierRepository::AtelierRepository(InterfaceDatabaseTable<Domain::Atelier> *database)
    : QObject(nullptr), Repository::GenericRepository<Domain::Atelier>(database)
{
}

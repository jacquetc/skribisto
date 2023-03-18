#include "atelier_repository.h"

using namespace Repository;

AtelierRepository::AtelierRepository(InterfaceDatabase<Domain::Atelier> *database)
    : QObject(nullptr), Repository::GenericRepository<Domain::Atelier>(database)
{
}

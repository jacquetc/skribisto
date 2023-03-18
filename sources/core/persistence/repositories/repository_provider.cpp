#include "repository_provider.h"

using namespace Repository;

QScopedPointer<RepositoryProvider> RepositoryProvider::s_instance = QScopedPointer<RepositoryProvider>(nullptr);

RepositoryProvider *RepositoryProvider::instance()
{
    if (s_instance.isNull())
    {
        s_instance.reset(new RepositoryProvider());
    }

    return s_instance.data();
}

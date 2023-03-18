#pragma once

#include "infrastructure/skrib/interface_skrib_loader.h"
#include "persistence/interface_repository_provider.h"
#include "infrastructure_global.h"

using namespace Contracts::DTO::System;
using namespace Contracts::Infrastructure::Skrib;
using namespace Contracts::Persistence;

namespace Infrastructure::Skrib
{

class SKR_INFRASTRUCTURE_EXPORT SkribLoader : public InterfaceSkribLoader
{

  public:
    explicit SkribLoader(InterfaceRepositoryProvider *repositoryProvider);

    Result<void> load(const LoadSystemDTO &dto);

  private:
    static Result<void> loadDatabase(const QUrl &fileName, QString &databaseName);
    static Result<void> updateDatabase(QString &databaseName);
    static Result<void> fillRepositories(InterfaceRepositoryProvider *repositoryProvider, QString &databaseName);
    static Result<void> closeDatabase(QString &databaseName);

    InterfaceRepositoryProvider *m_repositoryProvider;
};
} // namespace Infrastructure::Skrib

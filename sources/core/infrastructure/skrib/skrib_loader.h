#pragma once

#include "infrastructure/skrib/interface_skrib_loader.h"
#include "infrastructure_global.h"
#include "persistence/interface_repository_provider.h"

using namespace Contracts::DTO::System;
using namespace Contracts::Infrastructure::Skrib;
using namespace Contracts::Persistence;

namespace Infrastructure::Skrib
{

class SKR_INFRASTRUCTURE_EXPORT SkribLoader : public InterfaceSkribLoader
{

  public:
    explicit SkribLoader(InterfaceRepositoryProvider *repositoryProvider);

    Result<void> load(QPromise<Result<void>> &progressPromise, const LoadSystemDTO &dto);

  private:
    static Result<void> loadDatabase(QPromise<Result<void>> &progressPromise, const QUrl &fileName,
                                     QString &databaseName);
    static Result<void> updateDatabase(QPromise<Result<void>> &progressPromise, QString &databaseName);
    static Result<void> fillRepositories(QPromise<Result<void>> &progressPromise,
                                         InterfaceRepositoryProvider *repositoryProvider, QString &databaseName);
    static Result<void> closeDatabase(QPromise<Result<void>> &progressPromise, QString &databaseName);

    InterfaceRepositoryProvider *m_repositoryProvider;
};
} // namespace Infrastructure::Skrib

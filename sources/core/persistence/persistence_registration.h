#pragma once

#include "entity_schema.h"
#include "persistence_global.h"
#include "repositories/repository_provider.h"
#include <QObject>

namespace Persistence
{
class SKR_PERSISTENCE_EXPORT PersistenceRegistration : public QObject
{
    Q_OBJECT

  public:
    explicit PersistenceRegistration(QObject *parent, Domain::EntitySchema *entitySchema);

    Repository::RepositoryProvider *repositoryProvider();

  signals:

  private:
};
} // namespace Persistence

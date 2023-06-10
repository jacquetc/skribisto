#pragma once

#include "persistence_global.h"
#include <QObject>
#include "repositories/repository_provider.h"

namespace Persistence {
class SKR_PERSISTENCE_EXPORT PersistenceRegistration : public QObject {
    Q_OBJECT

public:

    explicit PersistenceRegistration(QObject *parent);

  Repository::RepositoryProvider* repositoryProvider();

signals:

private:
};
} // namespace Persistence

#pragma once

#include "persistence/interface_repository.h"
#include "persistence/interface_repository_provider.h"
#include "persistence_global.h"
#include <QDebug>
#include <QHash>
#include <QMutex>
#include <QObject>
#include <QSharedPointer>
#include <type_traits>

using namespace Contracts::Persistence;

namespace Repository
{
class SKR_PERSISTENCE_EXPORT RepositoryProvider : public QObject, public Contracts::Persistence::InterfaceRepositoryProvider
{
    Q_OBJECT
  public:
    static RepositoryProvider *instance();

    // InterfaceRepositoryProvider interface
  public:
    void registerRepository(Entities entity, QSharedPointer<InterfaceRepository> repository) override
    {
        QMutexLocker locker(&m_mutex);
        if (m_repositories.contains(entity))
        {
            qWarning() << "Repositories: m_repositories contains already this InterfaceRepository";
            return;
        }
        m_repositories.insert(entity, repository);
    }
    QSharedPointer<InterfaceRepository> repository(Entities entity) override
    {
        QMutexLocker locker(&m_mutex);
        auto repository = m_repositories.value(entity, nullptr);
        if (!repository)
        {
            qWarning() << "No repository registered for type" << entity;
        }
        return repository;
    }

  private:
    RepositoryProvider() = default;
    RepositoryProvider(const RepositoryProvider &) = delete;
    RepositoryProvider &operator=(const RepositoryProvider &) = delete;

    QHash<RepositoryProvider::Entities, QSharedPointer<InterfaceRepository>> m_repositories;
    QMutex m_mutex;
    static QScopedPointer<RepositoryProvider> s_instance;
};

} // namespace Repository

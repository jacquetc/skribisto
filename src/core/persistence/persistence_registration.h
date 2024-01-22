// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "persistence_export.h"
#include <QObject>
#include <qleany/repository/repository_provider.h>

namespace Skribisto::Persistence
{
class SKRIBISTO_PERSISTENCE_EXPORT PersistenceRegistration : public QObject
{
    Q_OBJECT

  public:
    explicit PersistenceRegistration(QObject *parent);

    Qleany::Repository::RepositoryProvider *repositoryProvider();

  signals:

  private:
    QScopedPointer<Qleany::Repository::RepositoryProvider> m_repositoryProvider;
};
} // namespace Skribisto::Persistence
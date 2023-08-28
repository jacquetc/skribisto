#pragma once
#include "persistence/interface_repository_provider.h"
#include "presenter_global.h"

#include <QObject>

namespace Presenter
{
class SKR_PRESENTER_EXPORT PresenterRegistration : public QObject
{
    Q_OBJECT

  public:
    explicit PresenterRegistration(QObject *parent,
                                   Contracts::Persistence::InterfaceRepositoryProvider *repositoryProvider);

  signals:

  private:
};
} // namespace Presenter

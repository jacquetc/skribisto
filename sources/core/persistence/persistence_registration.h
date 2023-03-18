#pragma once

#include "persistence_global.h"
#include <QObject>

namespace Persistence
{
class SKR_PERSISTENCE_EXPORT PersistenceRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit PersistenceRegistration(QObject *parent);

  signals:
  private:
};
} // namespace Persistence

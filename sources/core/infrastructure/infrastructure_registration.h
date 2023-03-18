#pragma once

#include "infrastructure_global.h"
#include <QObject>

class SKR_INFRASTRUCTURE_EXPORT InfrastructureRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit InfrastructureRegistration(QObject *parent);

  signals:
  private:
};

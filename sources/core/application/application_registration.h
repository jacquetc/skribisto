#pragma once

#include "application_global.h"
#include <QObject>

namespace Application
{
class SKR_APPLICATION_EXPORT ApplicationRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit ApplicationRegistration(QObject *parent = nullptr);

  signals:
  private:
};
}



#pragma once

#include "entity_schema.h"
#include <QObject>

namespace Domain
{

class DomainRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit DomainRegistration(QObject *parent = nullptr);

    EntitySchema *entitySchema() const;

  signals:

  private:
    EntitySchema *m_entitySchema;
};
} // namespace Domain

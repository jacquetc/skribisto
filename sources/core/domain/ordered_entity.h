#pragma once
#include "domain_global.h"
#include "entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT OrderedEntity : public Entity
{
    Q_OBJECT

  public:
    OrderedEntity() : Entity(){};

    OrderedEntity(int id, const QUuid &uuid) : Entity(id, uuid)
    {
    }

    OrderedEntity(int id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate)
        : Entity(id, uuid, creationDate, updateDate)
    {
    }

    OrderedEntity(const OrderedEntity &other) : Entity(other)
    {
    }

    OrderedEntity &operator=(const OrderedEntity &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
        }
        return *this;
    }

  public:
};

} // namespace Domain

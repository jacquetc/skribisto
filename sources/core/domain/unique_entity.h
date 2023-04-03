#pragma once
#include "domain_global.h"
#include "entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT UniqueEntity : public Entity
{
    Q_OBJECT

  public:
    UniqueEntity() : Entity(){};

    UniqueEntity(const QDateTime &creationDate, const QDateTime &updateDate)
        : Entity(-1, QUuid(), creationDate, updateDate)
    {
    }

    UniqueEntity(const UniqueEntity &other) : Entity(other)
    {
    }

    UniqueEntity &operator=(const UniqueEntity &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
        }
        return *this;
    }
};
} // namespace Domain

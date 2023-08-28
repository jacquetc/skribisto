#pragma once

#include "domain_global.h"
#include <QDateTime>
#include <QUuid>

#include "entities.h"
#include "entity_base.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Entity : public EntityBase
{
    Q_GADGET

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)

    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)

    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)

  public:
    Entity() : EntityBase(), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime())
    {
    }

    ~Entity()
    {
    }

    Entity(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate)
        : EntityBase(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate)
    {
    }

    Entity(const Entity &other)
        : EntityBase(other), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate)
    {
    }

    static Domain::Entities::EntityEnum enumValue()
    {
        return Domain::Entities::EntityEnum::Entity;
    }

    Entity &operator=(const Entity &other)
    {
        if (this != &other)
        {
            EntityBase::operator=(other);
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
        }
        return *this;
    }

    friend bool operator==(const Entity &lhs, const Entity &rhs);

    friend uint qHash(const Entity &entity, uint seed) noexcept;

    // ------ uuid : -----

    QUuid uuid() const
    {

        return m_uuid;
    }

    void setUuid(const QUuid &uuid)
    {
        m_uuid = uuid;
    }

    // ------ creationDate : -----

    QDateTime creationDate() const
    {

        return m_creationDate;
    }

    void setCreationDate(const QDateTime &creationDate)
    {
        m_creationDate = creationDate;
    }

    // ------ updateDate : -----

    QDateTime updateDate() const
    {

        return m_updateDate;
    }

    void setUpdateDate(const QDateTime &updateDate)
    {
        m_updateDate = updateDate;
    }

  private:
    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
};

inline bool operator==(const Entity &lhs, const Entity &rhs)
{

    return static_cast<const EntityBase &>(lhs) == static_cast<const EntityBase &>(rhs) &&

           lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate && lhs.m_updateDate == rhs.m_updateDate;
}

inline uint qHash(const Entity &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const EntityBase &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_uuid, seed);
    hash ^= ::qHash(entity.m_creationDate, seed);
    hash ^= ::qHash(entity.m_updateDate, seed);

    return hash;
}

} // namespace Domain
Q_DECLARE_METATYPE(Domain::Entity)
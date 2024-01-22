// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QDateTime>
#include <QUuid>

#include "entities.h"
#include "qleany/domain/entity_base.h"
#include <qleany/domain/entity_schema.h>

using namespace Qleany::Domain;

namespace Skribisto::Domain
{

class Entity : public EntityBase
{
    Q_GADGET

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)

    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)

    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)

  public:
    struct MetaData
    {
        MetaData(Entity *entity) : m_entity(entity)
        {
        }
        MetaData(Entity *entity, const MetaData &other) : m_entity(entity)
        {
        }

        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "uuid")
            {
                return true;
            }
            if (fieldName == "creationDate")
            {
                return true;
            }
            if (fieldName == "updateDate")
            {
                return true;
            }
            return m_entity->EntityBase::metaData().getSet(fieldName);
        }

        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "uuid")
            {
                return true;
            }
            if (fieldName == "creationDate")
            {
                return true;
            }
            if (fieldName == "updateDate")
            {
                return true;
            }
            return m_entity->EntityBase::metaData().getLoaded(fieldName);
        }

      private:
        Entity *m_entity = nullptr;
    };

    Entity() : EntityBase(), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_metaData(this)
    {
    }

    ~Entity()
    {
    }

    Entity(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate)
        : EntityBase(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_metaData(this)
    {
    }

    Entity(const Entity &other)
        : EntityBase(other), m_metaData(other.m_metaData), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Skribisto::Domain::Entities::EntityEnum enumValue()
    {
        return Skribisto::Domain::Entities::EntityEnum::Entity;
    }

    Entity &operator=(const Entity &other)
    {
        if (this != &other)
        {
            EntityBase::operator=(other);
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;

            m_metaData = MetaData(this, other.metaData());
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

    static Qleany::Domain::EntitySchema schema;

    MetaData metaData() const
    {
        return m_metaData;
    }

  protected:
    MetaData m_metaData;

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

/// Schema for Entity entity
inline Qleany::Domain::EntitySchema Entity::schema = {Skribisto::Domain::Entities::EntityEnum::Entity,
                                                      "Entity",

                                                      // relationships:
                                                      {

                                                      },

                                                      // fields:
                                                      {{"id", FieldType::Integer, true, false},
                                                       {"uuid", FieldType::Uuid, false, false},
                                                       {"creationDate", FieldType::DateTime, false, false},
                                                       {"updateDate", FieldType::DateTime, false, false}}};

} // namespace Skribisto::Domain
Q_DECLARE_METATYPE(Skribisto::Domain::Entity)
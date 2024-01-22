// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QString>

#include "entities.h"
#include "entity.h"
#include <qleany/domain/entity_schema.h>

using namespace Qleany::Domain;

namespace Skribisto::Domain
{

class RecentAtelier : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

    Q_PROPERTY(QString path READ path WRITE setPath)

    Q_PROPERTY(bool isReachable READ isReachable WRITE setIsReachable)

  public:
    struct MetaData
    {
        MetaData(RecentAtelier *entity) : m_entity(entity)
        {
        }
        MetaData(RecentAtelier *entity, const MetaData &other) : m_entity(entity)
        {
        }

        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "name")
            {
                return true;
            }
            if (fieldName == "path")
            {
                return true;
            }
            if (fieldName == "isReachable")
            {
                return true;
            }
            return m_entity->Entity::metaData().getSet(fieldName);
        }

        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "name")
            {
                return true;
            }
            if (fieldName == "path")
            {
                return true;
            }
            if (fieldName == "isReachable")
            {
                return true;
            }
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        RecentAtelier *m_entity = nullptr;
    };

    RecentAtelier() : Entity(), m_name(QString()), m_path(QString()), m_isReachable(false), m_metaData(this)
    {
    }

    ~RecentAtelier()
    {
    }

    RecentAtelier(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                  const QString &name, const QString &path, bool isReachable)
        : Entity(id, uuid, creationDate, updateDate), m_name(name), m_path(path), m_isReachable(isReachable),
          m_metaData(this)
    {
    }

    RecentAtelier(const RecentAtelier &other)
        : Entity(other), m_metaData(other.m_metaData), m_name(other.m_name), m_path(other.m_path),
          m_isReachable(other.m_isReachable)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Skribisto::Domain::Entities::EntityEnum enumValue()
    {
        return Skribisto::Domain::Entities::EntityEnum::RecentAtelier;
    }

    RecentAtelier &operator=(const RecentAtelier &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_name = other.m_name;
            m_path = other.m_path;
            m_isReachable = other.m_isReachable;

            m_metaData = MetaData(this, other.metaData());
        }
        return *this;
    }

    friend bool operator==(const RecentAtelier &lhs, const RecentAtelier &rhs);

    friend uint qHash(const RecentAtelier &entity, uint seed) noexcept;

    // ------ name : -----

    QString name() const
    {

        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
    }

    // ------ path : -----

    QString path() const
    {

        return m_path;
    }

    void setPath(const QString &path)
    {
        m_path = path;
    }

    // ------ isReachable : -----

    bool isReachable() const
    {

        return m_isReachable;
    }

    void setIsReachable(bool isReachable)
    {
        m_isReachable = isReachable;
    }

    static Qleany::Domain::EntitySchema schema;

    MetaData metaData() const
    {
        return m_metaData;
    }

  protected:
    MetaData m_metaData;

  private:
    QString m_name;
    QString m_path;
    bool m_isReachable;
};

inline bool operator==(const RecentAtelier &lhs, const RecentAtelier &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_name == rhs.m_name && lhs.m_path == rhs.m_path && lhs.m_isReachable == rhs.m_isReachable;
}

inline uint qHash(const RecentAtelier &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);
    hash ^= ::qHash(entity.m_path, seed);
    hash ^= ::qHash(entity.m_isReachable, seed);

    return hash;
}

/// Schema for RecentAtelier entity
inline Qleany::Domain::EntitySchema RecentAtelier::schema = {Skribisto::Domain::Entities::EntityEnum::RecentAtelier,
                                                             "RecentAtelier",

                                                             // relationships:
                                                             {

                                                             },

                                                             // fields:
                                                             {{"id", FieldType::Integer, true, false},
                                                              {"uuid", FieldType::Uuid, false, false},
                                                              {"creationDate", FieldType::DateTime, false, false},
                                                              {"updateDate", FieldType::DateTime, false, false},
                                                              {"name", FieldType::String, false, false},
                                                              {"path", FieldType::String, false, false},
                                                              {"isReachable", FieldType::Bool, false, false}}};

} // namespace Skribisto::Domain
Q_DECLARE_METATYPE(Skribisto::Domain::RecentAtelier)
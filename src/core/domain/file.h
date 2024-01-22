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

class File : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString path READ path WRITE setPath)

    Q_PROPERTY(QString hash READ hash WRITE setHash)

  public:
    struct MetaData
    {
        MetaData(File *entity) : m_entity(entity)
        {
        }
        MetaData(File *entity, const MetaData &other) : m_entity(entity)
        {
        }

        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "path")
            {
                return true;
            }
            if (fieldName == "hash")
            {
                return true;
            }
            return m_entity->Entity::metaData().getSet(fieldName);
        }

        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "path")
            {
                return true;
            }
            if (fieldName == "hash")
            {
                return true;
            }
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        File *m_entity = nullptr;
    };

    File() : Entity(), m_path(QString()), m_hash(QString()), m_metaData(this)
    {
    }

    ~File()
    {
    }

    File(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
         const QString &path, const QString &hash)
        : Entity(id, uuid, creationDate, updateDate), m_path(path), m_hash(hash), m_metaData(this)
    {
    }

    File(const File &other) : Entity(other), m_metaData(other.m_metaData), m_path(other.m_path), m_hash(other.m_hash)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Skribisto::Domain::Entities::EntityEnum enumValue()
    {
        return Skribisto::Domain::Entities::EntityEnum::File;
    }

    File &operator=(const File &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_path = other.m_path;
            m_hash = other.m_hash;

            m_metaData = MetaData(this, other.metaData());
        }
        return *this;
    }

    friend bool operator==(const File &lhs, const File &rhs);

    friend uint qHash(const File &entity, uint seed) noexcept;

    // ------ path : -----

    QString path() const
    {

        return m_path;
    }

    void setPath(const QString &path)
    {
        m_path = path;
    }

    // ------ hash : -----

    QString hash() const
    {

        return m_hash;
    }

    void setHash(const QString &hash)
    {
        m_hash = hash;
    }

    static Qleany::Domain::EntitySchema schema;

    MetaData metaData() const
    {
        return m_metaData;
    }

  protected:
    MetaData m_metaData;

  private:
    QString m_path;
    QString m_hash;
};

inline bool operator==(const File &lhs, const File &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_path == rhs.m_path && lhs.m_hash == rhs.m_hash;
}

inline uint qHash(const File &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_path, seed);
    hash ^= ::qHash(entity.m_hash, seed);

    return hash;
}

/// Schema for File entity
inline Qleany::Domain::EntitySchema File::schema = {
    Skribisto::Domain::Entities::EntityEnum::File,
    "File",

    // relationships:
    {{Skribisto::Domain::Entities::EntityEnum::Workspace, "Workspace", Skribisto::Domain::Entities::EntityEnum::File,
      "File", "files", RelationshipType::OneToMany, RelationshipStrength::Strong,
      RelationshipCardinality::ManyUnordered, RelationshipDirection::Backward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"path", FieldType::String, false, false},
     {"hash", FieldType::String, false, false}}};

} // namespace Skribisto::Domain
Q_DECLARE_METATYPE(Skribisto::Domain::File)
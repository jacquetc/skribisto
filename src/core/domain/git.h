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

class Git : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString remoteGitRepositoryUrl READ remoteGitRepositoryUrl WRITE setRemoteGitRepositoryUrl)

    Q_PROPERTY(bool isGitHub READ isGitHub WRITE setIsGitHub)

  public:
    struct MetaData
    {
        MetaData(Git *entity) : m_entity(entity)
        {
        }
        MetaData(Git *entity, const MetaData &other) : m_entity(entity)
        {
        }

        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "remoteGitRepositoryUrl")
            {
                return true;
            }
            if (fieldName == "isGitHub")
            {
                return true;
            }
            return m_entity->Entity::metaData().getSet(fieldName);
        }

        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "remoteGitRepositoryUrl")
            {
                return true;
            }
            if (fieldName == "isGitHub")
            {
                return true;
            }
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        Git *m_entity = nullptr;
    };

    Git() : Entity(), m_remoteGitRepositoryUrl(QString()), m_isGitHub(false), m_metaData(this)
    {
    }

    ~Git()
    {
    }

    Git(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
        const QString &remoteGitRepositoryUrl, bool isGitHub)
        : Entity(id, uuid, creationDate, updateDate), m_remoteGitRepositoryUrl(remoteGitRepositoryUrl),
          m_isGitHub(isGitHub), m_metaData(this)
    {
    }

    Git(const Git &other)
        : Entity(other), m_metaData(other.m_metaData), m_remoteGitRepositoryUrl(other.m_remoteGitRepositoryUrl),
          m_isGitHub(other.m_isGitHub)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Skribisto::Domain::Entities::EntityEnum enumValue()
    {
        return Skribisto::Domain::Entities::EntityEnum::Git;
    }

    Git &operator=(const Git &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_remoteGitRepositoryUrl = other.m_remoteGitRepositoryUrl;
            m_isGitHub = other.m_isGitHub;

            m_metaData = MetaData(this, other.metaData());
        }
        return *this;
    }

    friend bool operator==(const Git &lhs, const Git &rhs);

    friend uint qHash(const Git &entity, uint seed) noexcept;

    // ------ remoteGitRepositoryUrl : -----

    QString remoteGitRepositoryUrl() const
    {

        return m_remoteGitRepositoryUrl;
    }

    void setRemoteGitRepositoryUrl(const QString &remoteGitRepositoryUrl)
    {
        m_remoteGitRepositoryUrl = remoteGitRepositoryUrl;
    }

    // ------ isGitHub : -----

    bool isGitHub() const
    {

        return m_isGitHub;
    }

    void setIsGitHub(bool isGitHub)
    {
        m_isGitHub = isGitHub;
    }

    static Qleany::Domain::EntitySchema schema;

    MetaData metaData() const
    {
        return m_metaData;
    }

  protected:
    MetaData m_metaData;

  private:
    QString m_remoteGitRepositoryUrl;
    bool m_isGitHub;
};

inline bool operator==(const Git &lhs, const Git &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_remoteGitRepositoryUrl == rhs.m_remoteGitRepositoryUrl && lhs.m_isGitHub == rhs.m_isGitHub;
}

inline uint qHash(const Git &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_remoteGitRepositoryUrl, seed);
    hash ^= ::qHash(entity.m_isGitHub, seed);

    return hash;
}

/// Schema for Git entity
inline Qleany::Domain::EntitySchema Git::schema = {
    Skribisto::Domain::Entities::EntityEnum::Git,
    "Git",

    // relationships:
    {{Skribisto::Domain::Entities::EntityEnum::Atelier, "Atelier", Skribisto::Domain::Entities::EntityEnum::Git, "Git",
      "git", RelationshipType::OneToOne, RelationshipStrength::Strong, RelationshipCardinality::One,
      RelationshipDirection::Backward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"remoteGitRepositoryUrl", FieldType::String, false, false},
     {"isGitHub", FieldType::Bool, false, false}}};

} // namespace Skribisto::Domain
Q_DECLARE_METATYPE(Skribisto::Domain::Git)
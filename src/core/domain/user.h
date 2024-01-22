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

class User : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

    Q_PROPERTY(QString email READ email WRITE setEmail)

  public:
    struct MetaData
    {
        MetaData(User *entity) : m_entity(entity)
        {
        }
        MetaData(User *entity, const MetaData &other) : m_entity(entity)
        {
        }

        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "name")
            {
                return true;
            }
            if (fieldName == "email")
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
            if (fieldName == "email")
            {
                return true;
            }
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        User *m_entity = nullptr;
    };

    User() : Entity(), m_name(QString()), m_email(QString()), m_metaData(this)
    {
    }

    ~User()
    {
    }

    User(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
         const QString &name, const QString &email)
        : Entity(id, uuid, creationDate, updateDate), m_name(name), m_email(email), m_metaData(this)
    {
    }

    User(const User &other) : Entity(other), m_metaData(other.m_metaData), m_name(other.m_name), m_email(other.m_email)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Skribisto::Domain::Entities::EntityEnum enumValue()
    {
        return Skribisto::Domain::Entities::EntityEnum::User;
    }

    User &operator=(const User &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_name = other.m_name;
            m_email = other.m_email;

            m_metaData = MetaData(this, other.metaData());
        }
        return *this;
    }

    friend bool operator==(const User &lhs, const User &rhs);

    friend uint qHash(const User &entity, uint seed) noexcept;

    // ------ name : -----

    QString name() const
    {

        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
    }

    // ------ email : -----

    QString email() const
    {

        return m_email;
    }

    void setEmail(const QString &email)
    {
        m_email = email;
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
    QString m_email;
};

inline bool operator==(const User &lhs, const User &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_name == rhs.m_name && lhs.m_email == rhs.m_email;
}

inline uint qHash(const User &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);
    hash ^= ::qHash(entity.m_email, seed);

    return hash;
}

/// Schema for User entity
inline Qleany::Domain::EntitySchema User::schema = {Skribisto::Domain::Entities::EntityEnum::User,
                                                    "User",

                                                    // relationships:
                                                    {

                                                    },

                                                    // fields:
                                                    {{"id", FieldType::Integer, true, false},
                                                     {"uuid", FieldType::Uuid, false, false},
                                                     {"creationDate", FieldType::DateTime, false, false},
                                                     {"updateDate", FieldType::DateTime, false, false},
                                                     {"name", FieldType::String, false, false},
                                                     {"email", FieldType::String, false, false}}};

} // namespace Skribisto::Domain
Q_DECLARE_METATYPE(Skribisto::Domain::User)
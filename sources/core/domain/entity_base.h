#pragma once

#include "domain_global.h"
#include <QObject>

namespace Domain
{

class SKR_DOMAIN_EXPORT EntityBase
{
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)

  public:
    EntityBase() : m_id(0)
    {
    }

    ~EntityBase()
    {
    }
    EntityBase(int id) : m_id(id)
    {
    }

    EntityBase(const EntityBase &other) : m_id(other.m_id)
    {
    }

    EntityBase &operator=(const EntityBase &other)
    {
        if (this != &other)
        {

            m_id = other.m_id;
        }
        return *this;
    }

    bool isValid()
    {
        return m_id > 0;
    }

    friend bool operator==(const EntityBase &lhs, const EntityBase &rhs);

    friend uint qHash(const EntityBase &entity, uint seed) noexcept;

    // ------ id : -----

    int id() const
    {

        return m_id;
    }

    void setId(int id)
    {
        m_id = id;
    }

  private:
    int m_id;
};

inline bool operator==(const EntityBase &lhs, const EntityBase &rhs)
{

    return

        lhs.m_id == rhs.m_id;
}

inline uint qHash(const EntityBase &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_id, seed);

    return hash;
}

} // namespace Domain
Q_DECLARE_METATYPE(Domain::EntityBase)
Q_DECLARE_METATYPE(QSet<Domain::EntityBase>);

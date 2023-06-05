#pragma once

#include "domain_global.h"
#include <QString>

#include "dummy_entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT DummyOtherEntity : public DummyEntity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

  public:
    DummyOtherEntity() : DummyEntity(), m_name(QString())
    {
    }

    ~DummyOtherEntity()
    {
    }

    DummyOtherEntity(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QString &name)
        : DummyEntity(id, uuid, creationDate), m_name(name)
    {
    }

    DummyOtherEntity(const DummyOtherEntity &other) : DummyEntity(other), m_name(other.m_name)
    {
    }

    DummyOtherEntity &operator=(const DummyOtherEntity &other)
    {
        if (this != &other)
        {
            DummyEntity::operator=(other);
            m_name = other.m_name;
        }
        return *this;
    }

    friend bool operator==(const DummyOtherEntity &lhs, const DummyOtherEntity &rhs);

    friend uint qHash(const DummyOtherEntity &entity, uint seed) noexcept;

    // ------ name : -----

    QString name() const
    {

        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
    }

  private:
    QString m_name;
};

inline bool operator==(const DummyOtherEntity &lhs, const DummyOtherEntity &rhs)
{

    return static_cast<const DummyEntity &>(lhs) == static_cast<const DummyEntity &>(rhs) &&

           lhs.m_name == rhs.m_name;
}

inline uint qHash(const DummyOtherEntity &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const DummyEntity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);

    return hash;
}

} // namespace Domain
Q_DECLARE_METATYPE(Domain::DummyOtherEntity)

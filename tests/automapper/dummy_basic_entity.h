#pragma once

#include <QString>

#include "dummy_entity.h"

namespace Domain
{

class DummyBasicEntity : public DummyEntity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

  public:
    DummyBasicEntity() : DummyEntity(), m_name(QString())
    {
    }

    ~DummyBasicEntity()
    {
    }

    DummyBasicEntity(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QString &name)
        : DummyEntity(id, uuid, creationDate), m_name(name)
    {
    }

    DummyBasicEntity(const DummyBasicEntity &other) : DummyEntity(other), m_name(other.m_name)
    {
    }

    DummyBasicEntity &operator=(const DummyBasicEntity &other)
    {
        if (this != &other)
        {
            DummyEntity::operator=(other);
            m_name = other.m_name;
        }
        return *this;
    }

    friend bool operator==(const DummyBasicEntity &lhs, const DummyBasicEntity &rhs);

    friend uint qHash(const DummyBasicEntity &entity, uint seed) noexcept;

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

inline bool operator==(const DummyBasicEntity &lhs, const DummyBasicEntity &rhs)
{

    return static_cast<const DummyEntity &>(lhs) == static_cast<const DummyEntity &>(rhs) &&

           lhs.m_name == rhs.m_name;
}

inline uint qHash(const DummyBasicEntity &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const DummyEntity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);

    return hash;
}

} // namespace Domain
Q_DECLARE_METATYPE(Domain::DummyBasicEntity)

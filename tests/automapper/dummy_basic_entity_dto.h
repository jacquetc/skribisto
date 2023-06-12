#pragma once

#include <QDateTime>
#include <QObject>
#include <QString>
#include <QUuid>

namespace Contracts::DTO::DummyBasicEntity
{

class DummyBasicEntityDTO
{
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QString name READ name WRITE setName)

  public:
    DummyBasicEntityDTO() : m_id(0), m_uuid(QUuid()), m_creationDate(QDateTime()), m_name(QString())
    {
    }

    ~DummyBasicEntityDTO()
    {
    }

    DummyBasicEntityDTO(int id, const QUuid &uuid, const QDateTime &creationDate, const QString &name)
        : m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_name(name)
    {
    }

    DummyBasicEntityDTO(const DummyBasicEntityDTO &other)
        : m_id(other.m_id), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate), m_name(other.m_name)
    {
    }

    DummyBasicEntityDTO &operator=(const DummyBasicEntityDTO &other)
    {
        if (this != &other)
        {
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_name = other.m_name;
        }
        return *this;
    }

    friend bool operator==(const DummyBasicEntityDTO &lhs, const DummyBasicEntityDTO &rhs);

    friend uint qHash(const DummyBasicEntityDTO &dto, uint seed) noexcept;

    // ------ id : -----

    int id() const
    {
        return m_id;
    }

    void setId(int id)
    {
        m_id = id;
    }

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
    int m_id;
    QUuid m_uuid;
    QDateTime m_creationDate;
    QString m_name;
};

inline bool operator==(const DummyBasicEntityDTO &lhs, const DummyBasicEntityDTO &rhs)
{

    return lhs.m_id == rhs.m_id && lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_name == rhs.m_name;
}

inline uint qHash(const DummyBasicEntityDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_id, seed);
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_name, seed);

    return hash;
}

} // namespace Contracts::DTO::DummyBasicEntity
Q_DECLARE_METATYPE(Contracts::DTO::DummyBasicEntity::DummyBasicEntityDTO)
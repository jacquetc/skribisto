#pragma once

#include "dummy_basic_entity_dto.h"
#include <QDateTime>
#include <QObject>
#include <QString>
#include <QUuid>

using namespace Contracts::DTO::DummyBasicEntity;

namespace Contracts::DTO::DummyEntityWithDetails {
class DummyEntityWithDetailsDTO {
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QString name READ name WRITE setName)
    Q_PROPERTY(QList<DummyBasicEntityDTO>details READ details WRITE setDetails)
    Q_PROPERTY(DummyBasicEntityDTO uniqueDetail READ uniqueDetail WRITE setUniqueDetail)

public:

    DummyEntityWithDetailsDTO() : m_id(0), m_uuid(QUuid()), m_creationDate(QDateTime()), m_name(QString())
    {}

    ~DummyEntityWithDetailsDTO()
    {}

    DummyEntityWithDetailsDTO(int id, const QUuid& uuid, const QDateTime& creationDate, const QString& name,
                              const QList<DummyBasicEntityDTO>& details, const DummyBasicEntityDTO& uniqueDetail)
        : m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_name(name), m_details(details), m_uniqueDetail(
            uniqueDetail)
    {}

    DummyEntityWithDetailsDTO(const DummyEntityWithDetailsDTO& other)
        : m_id(other.m_id), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate), m_name(other.m_name),
        m_details(other.m_details), m_uniqueDetail(other.m_uniqueDetail)

    {}

    DummyEntityWithDetailsDTO& operator=(const DummyEntityWithDetailsDTO& other)
    {
        if (this != &other)
        {
            m_id           = other.m_id;
            m_uuid         = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_name         = other.m_name;
            m_details      = other.m_details;
            m_uniqueDetail = other.m_uniqueDetail;
        }
        return *this;
    }

    friend bool operator==(const DummyEntityWithDetailsDTO& lhs,
                           const DummyEntityWithDetailsDTO& rhs);

    friend uint qHash(const DummyEntityWithDetailsDTO& dto,
                      uint                             seed) noexcept;

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

    void setUuid(const QUuid& uuid)
    {
        m_uuid = uuid;
    }

    // ------ creationDate : -----

    QDateTime creationDate() const
    {
        return m_creationDate;
    }

    void setCreationDate(const QDateTime& creationDate)
    {
        m_creationDate = creationDate;
    }

    // ------ name : -----

    QString name() const
    {
        return m_name;
    }

    void setName(const QString& name)
    {
        m_name = name;
    }

    // ------ details : -----

    QList<DummyBasicEntityDTO>details() const
    {
        return m_details;
    }

    void setDetails(const QList<DummyBasicEntityDTO>& details)
    {
        m_details = details;
    }

    // ------ unique detail : -----

    DummyBasicEntityDTO uniqueDetail() const
    {
        return m_uniqueDetail;
    }

    void setUniqueDetail(const DummyBasicEntityDTO& uniqueDetail)
    {
        m_uniqueDetail = uniqueDetail;
    }

private:

    int m_id;
    QUuid m_uuid;
    QDateTime m_creationDate;
    QString m_name;
    QList<DummyBasicEntityDTO>m_details;
    DummyBasicEntityDTO m_uniqueDetail;
};

inline bool operator==(const DummyEntityWithDetailsDTO& lhs, const DummyEntityWithDetailsDTO& rhs)
{
    return lhs.m_id == rhs.m_id && lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_name == rhs.m_name && lhs.m_details == rhs.m_details && lhs.m_uniqueDetail == rhs.m_uniqueDetail;
}

inline uint qHash(const DummyEntityWithDetailsDTO& dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_id, seed);
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_name, seed);
    hash ^= ::qHash(dto.m_details, seed);
    hash ^= ::qHash(dto.m_uniqueDetail, seed);


    return hash;
}
} // namespace Contracts::DTO::DummyEntityWithDetails
Q_DECLARE_METATYPE(Contracts::DTO::DummyEntityWithDetails::DummyEntityWithDetailsDTO)

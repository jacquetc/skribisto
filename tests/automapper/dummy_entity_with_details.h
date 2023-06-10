#pragma once

#include "domain_global.h"
#include <QString>

#include "dummy_basic_entity.h"
#include "dummy_entity.h"

namespace Domain {
class SKR_DOMAIN_EXPORT DummyEntityWithDetails : public DummyEntity {
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)
    Q_PROPERTY(QList<DummyBasicEntity>details READ details WRITE setDetails)
    Q_PROPERTY(DummyBasicEntity uniqueDetail READ uniqueDetail WRITE setUniqueDetail)

public:

    DummyEntityWithDetails() : DummyEntity(), m_name(QString())
    {}

    ~DummyEntityWithDetails()
    {}

    DummyEntityWithDetails(const int& id, const QUuid& uuid, const QDateTime& creationDate, const QString& name,
                           const QList<DummyBasicEntity>& details, const DummyBasicEntity& uniqueDetail)
        : DummyEntity(id, uuid, creationDate), m_name(name), m_details(details), m_uniqueDetail(uniqueDetail)
    {}

    DummyEntityWithDetails(const DummyEntityWithDetails& other)
        : DummyEntity(other), m_name(other.m_name), m_details(other.m_details), m_uniqueDetail(other.m_uniqueDetail)

    {}

    DummyEntityWithDetails& operator=(const DummyEntityWithDetails& other)
    {
        if (this != &other)
        {
            DummyEntity::operator=(other);
            m_name         = other.m_name;
            m_details      = other.m_details;
            m_uniqueDetail = other.m_uniqueDetail;
        }
        return *this;
    }

    friend bool operator==(const DummyEntityWithDetails& lhs,
                           const DummyEntityWithDetails& rhs);

    friend uint qHash(const DummyEntityWithDetails& entity,
                      uint                          seed) noexcept;

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

    QList<DummyBasicEntity>details() const
    {
        return m_details;
    }

    void setDetails(const QList<DummyBasicEntity>& details)
    {
        m_details = details;
    }

    // ------ uniqueDetail : -----

    DummyBasicEntity uniqueDetail() const
    {
        return m_uniqueDetail;
    }

    void setUniqueDetail(const DummyBasicEntity& uniqueDetail)
    {
        m_uniqueDetail = uniqueDetail;
    }

private:

    QString m_name;
    QList<DummyBasicEntity>m_details;
    DummyBasicEntity m_uniqueDetail;
};

inline bool operator==(const DummyEntityWithDetails& lhs, const DummyEntityWithDetails& rhs)
{
    return static_cast<const DummyEntity&>(lhs) == static_cast<const DummyEntity&>(rhs) &&

           lhs.m_name == rhs.m_name && lhs.m_details == rhs.m_details && lhs.m_uniqueDetail == rhs.m_uniqueDetail;
}

inline uint qHash(const DummyEntityWithDetails& entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    hash ^= qHash(static_cast<const DummyEntity&>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);
    hash ^= ::qHash(entity.m_details, seed);
    hash ^= ::qHash(entity.m_uniqueDetail, seed);


    return hash;
}
} // namespace Domain
Q_DECLARE_METATYPE(Domain::DummyEntityWithDetails)

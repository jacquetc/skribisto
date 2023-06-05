#pragma once

#include "domain_global.h"
#include <QDateTime>
#include <QUuid>

#include "entity_base.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT DummyEntity : public EntityBase
{
    Q_GADGET

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)

    
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)

    

  public:
    DummyEntity() : EntityBase() , m_uuid(QUuid()), m_creationDate(QDateTime())
    {
    }

    ~DummyEntity()
    {
    }

   DummyEntity(  const int &id,   const QUuid &uuid,   const QDateTime &creationDate ) 
        : EntityBase(id), m_uuid(uuid), m_creationDate(creationDate)
    {
    }

    DummyEntity(const DummyEntity &other) : EntityBase(other), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate)
    {
    }

    DummyEntity &operator=(const DummyEntity &other)
    {
        if (this != &other)
        {
            EntityBase::operator=(other);
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            
        }
        return *this;
    }

    friend bool operator==(const DummyEntity &lhs, const DummyEntity &rhs);


    friend uint qHash(const DummyEntity &entity, uint seed) noexcept;



    // ------ uuid : -----

    QUuid uuid() const
    {
        
        return m_uuid;
    }

    void setUuid( const QUuid &uuid)
    {
        m_uuid = uuid;
    }
    

    // ------ creationDate : -----

    QDateTime creationDate() const
    {
        
        return m_creationDate;
    }

    void setCreationDate( const QDateTime &creationDate)
    {
        m_creationDate = creationDate;
    }
    


  private:
QUuid m_uuid;
    QDateTime m_creationDate;
    
};

inline bool operator==(const DummyEntity &lhs, const DummyEntity &rhs)
{

    return 
            static_cast<const EntityBase&>(lhs) == static_cast<const EntityBase&>(rhs) &&
    
            lhs.m_uuid == rhs.m_uuid  && lhs.m_creationDate == rhs.m_creationDate 
    ;
}

inline uint qHash(const DummyEntity &entity, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;
        hash ^= qHash(static_cast<const EntityBase&>(entity), seed);

        // Combine with this class's properties
        hash ^= ::qHash(entity.m_uuid, seed);
        hash ^= ::qHash(entity.m_creationDate, seed);
        

        return hash;
}

} // namespace Domain
Q_DECLARE_METATYPE(Domain::DummyEntity)

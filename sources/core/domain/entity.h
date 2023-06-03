#pragma once

#include "domain_global.h"
#include <QUuid>
#include <QDateTime>



namespace Domain
{

class SKR_DOMAIN_EXPORT Entity : public QObject
{
    Q_OBJECT

    Q_PROPERTY(int id READ id WRITE setId)

    
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)

    
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)

    
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)

    

  public:
    Entity() : QObject(){};

   Entity(   int id,   const QUuid &uuid,   const QDateTime &creationDate,   const QDateTime &updateDate ) 
        : QObject(), m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate)
    {
    }

    Entity(const Entity &other) : QObject(), m_id(other.m_id), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate), m_updateDate(other.m_updateDate)
    {
    }

    Entity &operator=(const Entity &other)
    {
        if (this != &other)
        {
            
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            
        }
        return *this;
    }


    // ------ id : -----

    int id() const
    {
        
        return m_id;
    }

    void setId( int id)
    {
        m_id = id;
    }
    

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
    

    // ------ updateDate : -----

    QDateTime updateDate() const
    {
        
        return m_updateDate;
    }

    void setUpdateDate( const QDateTime &updateDate)
    {
        m_updateDate = updateDate;
    }
    


  private:
int m_id;
    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    
};

} // namespace Domain
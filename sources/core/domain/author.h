#pragma once

#include "domain_global.h"
#include <QString>

#include "entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Author : public Entity
{
    Q_OBJECT

    Q_PROPERTY(QString name READ name WRITE setName)

    

  public:
    Author() : Entity(){};

   Author(  const int &id,  const QUuid &uuid,  const QDateTime &creationDate,  const QDateTime &updateDate,   const QString &name ) 
        : Entity(id, uuid, creationDate, updateDate), m_name(name)
    {
    }

    Author(const Author &other) : Entity(other), m_name(other.m_name)
    {
    }

    Author &operator=(const Author &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_name = other.m_name;
            
        }
        return *this;
    }


    // ------ name : -----

    QString name() const
    {
        
        return m_name;
    }

    void setName( const QString &name)
    {
        m_name = name;
    }
    


  private:
QString m_name;
    
};

} // namespace Domain
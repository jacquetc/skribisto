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
    Author() : Entity() , m_name(QString()){}

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

    friend bool operator==(const Author &lhs, const Author &rhs);


    friend uint qHash(const Author &entity, uint seed) noexcept;



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

inline bool operator==(const Author &lhs, const Author &rhs)
{

    return 
            static_cast<const Entity&>(lhs) == static_cast<const Entity&>(rhs) &&
    
            lhs.m_name == rhs.m_name 
    ;
}

inline uint qHash(const Author &entity, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;
        hash ^= qHash(static_cast<const Entity&>(entity), seed);

        // Combine with this class's properties
        hash ^= ::qHash(entity.m_name, seed);
        

        return hash;
}

} // namespace Domain
Q_DECLARE_METATYPE(Domain::Author)
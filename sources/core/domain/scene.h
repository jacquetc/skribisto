#pragma once

#include "domain_global.h"
#include <QString>

#include "entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Scene : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString title READ title WRITE setTitle)

    

  public:
    Scene() : Entity() , m_title(QString())
    {
    }

    ~Scene()
    {
    }

   Scene(  const int &id,  const QUuid &uuid,  const QDateTime &creationDate,  const QDateTime &updateDate,   const QString &title ) 
        : Entity(id, uuid, creationDate, updateDate), m_title(title)
    {
    }

    Scene(const Scene &other) : Entity(other), m_title(other.m_title)
    {
    }

    Scene &operator=(const Scene &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_title = other.m_title;
            
        }
        return *this;
    }

    friend bool operator==(const Scene &lhs, const Scene &rhs);


    friend uint qHash(const Scene &entity, uint seed) noexcept;



    // ------ title : -----

    QString title() const
    {
        
        return m_title;
    }

    void setTitle( const QString &title)
    {
        m_title = title;
    }
    


  private:
QString m_title;
    
};

inline bool operator==(const Scene &lhs, const Scene &rhs)
{

    return 
            static_cast<const Entity&>(lhs) == static_cast<const Entity&>(rhs) &&
    
            lhs.m_title == rhs.m_title 
    ;
}

inline uint qHash(const Scene &entity, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;
        hash ^= qHash(static_cast<const Entity&>(entity), seed);

        // Combine with this class's properties
        hash ^= ::qHash(entity.m_title, seed);
        

        return hash;
}

} // namespace Domain
Q_DECLARE_METATYPE(Domain::Scene)
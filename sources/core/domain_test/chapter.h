#pragma once

#include "domain_global.h"
#include <QString>

#include "entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Chapter : public Entity
{
    Q_OBJECT

    Q_PROPERTY(QString title READ title WRITE setTitle)

    

  public:
    Chapter() : Entity(id(-1), uuid(QUuid::createUuid()), creationDate(QDateTime::currentDateTime()), updateDate(QDateTime::currentDateTime())){};

   Chapter(  const int &id,  const QUuid &uuid,  const QDateTime &creationDate,  const QDateTime &updateDate,   const QString &title ) 
        : Entity(id, uuid, creationDate, updateDate), m_title(title)
    {
    }

    Chapter(const Chapter &other) : Entity(other), m_title(other.m_title)
    {
    }

    Chapter &operator=(const Chapter &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_title = other.m_title;
        }
        return *this;
    }


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

} // namespace Domain
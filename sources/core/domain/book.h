#pragma once

#include "domain_global.h"
#include "entity.h"

#include "book.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Book : public Entity
{
    Q_OBJECT
    Q_PROPERTY(QString name READ getName WRITE setName)

  public:
    Book() : Entity(){};

    Book(const QUuid &uuid, const QString &name, const QUuid &relative) : Entity(uuid)
    {
        m_name = name;
    }

    Book(const QUuid &uuid, const QString &name, const QUuid &relative, const QDateTime &creationDate,
         const QDateTime &updateDate)
        : Entity(uuid, creationDate, updateDate), m_name(name)
    {
    }

    Book(const Book &other) : Entity(other), m_name(other.m_name)
    {
    }

    Book &operator=(const Book &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_name = other.m_name;
        }
        return *this;
    }

    QString getName() const
    {
        return m_name;
    }

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

} // namespace Domain

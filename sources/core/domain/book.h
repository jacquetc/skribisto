#pragma once

#include "book.h"
#include "domain_global.h"
#include "ordered_entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Book : public OrderedEntity
{
    Q_OBJECT
    Q_PROPERTY(QString name READ name WRITE setName)

  public:
    Book() : OrderedEntity(){};

    Book(int id, const QUuid &uuid, const QString &name) : OrderedEntity(id, uuid)
    {
        m_name = name;
    }

    Book(int id, const QUuid &uuid, const QString &name, const QDateTime &creationDate, const QDateTime &updateDate)
        : OrderedEntity(id, uuid, creationDate, updateDate), m_name(name)
    {
    }

    Book(const Book &other) : OrderedEntity(other), m_name(other.m_name)
    {
    }

    Book &operator=(const Book &other)
    {
        if (this != &other)
        {
            OrderedEntity::operator=(other);
            m_name = other.m_name;
        }
        return *this;
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

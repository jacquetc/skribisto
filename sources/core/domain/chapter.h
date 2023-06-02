#pragma once

#include "domain_global.h"
#include "ordered_entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Chapter : public OrderedEntity
{
    Q_OBJECT
    Q_PROPERTY(QString title READ title WRITE setTitle)

  public:
    Chapter() : OrderedEntity(){};

    Chapter(int id, const QUuid &uuid, const QString &title) : OrderedEntity(id, uuid)
    {
        m_title = title;
    }

    Chapter(int id, const QUuid &uuid, const QString &title, const QDateTime &creationDate, const QDateTime &updateDate)
        : OrderedEntity(id, uuid, creationDate, updateDate), m_title(title)
    {
    }

    Chapter(const Chapter &other) : OrderedEntity(other), m_title(other.m_title)
    {
    }

    Chapter &operator=(const Chapter &other)
    {
        if (this != &other)
        {
            OrderedEntity::operator=(other);
            m_title = other.m_title;
        }
        return *this;
    }

    QString title() const
    {
        return m_title;
    }

    void setTitle(const QString &title)
    {
        m_title = title;
    }

  private:
    QString m_title;
};

} // namespace Domain

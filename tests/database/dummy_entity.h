#pragma once

#include "ordered_entity.h"

namespace Domain
{

class DummyEntity : public OrderedEntity
{
    Q_OBJECT
    Q_PROPERTY(QString name READ name WRITE setName)
    Q_PROPERTY(QString author READ author WRITE setAuthor)

  public:
    DummyEntity() : OrderedEntity(){};

    DummyEntity(int id, const QUuid &uuid, const QString &name, const QString &author) : OrderedEntity(id, uuid)
    {
        m_name = name;
        m_author = author;
    }

    DummyEntity(int id, const QUuid &uuid, const QString &name, const QString &author, const QDateTime &creationDate,
                const QDateTime &updateDate)
        : OrderedEntity(id, uuid, creationDate, updateDate), m_name(name), m_author(author)
    {
    }

    DummyEntity(const DummyEntity &other) : OrderedEntity(other), m_name(other.m_name), m_author(other.m_author)
    {
    }

    DummyEntity &operator=(const DummyEntity &other)
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

    QString author() const
    {
        return m_author;
    }
    void setAuthor(const QString &newAuthor)
    {
        m_author = newAuthor;
    }

  private:
    QString m_name;
    QString m_author;
};

} // namespace Domain

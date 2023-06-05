#pragma once

#include "entity.h"

namespace Domain
{

class DummyEntity : public Entity
{
    Q_OBJECT
    Q_PROPERTY(QString name READ name WRITE setName)
    Q_PROPERTY(QString author READ author WRITE setAuthor)

  public:
    DummyEntity() : Entity(){};

    DummyEntity(int id, const QUuid &uuid, const QString &name, const QString &author)
        : Entity(id, uuid, QDateTime(), QDateTime())
    {
        m_name = name;
        m_author = author;
    }

    DummyEntity(int id, const QUuid &uuid, const QString &name, const QString &author, const QDateTime &creationDate,
                const QDateTime &updateDate)
        : Entity(id, uuid, creationDate, updateDate), m_name(name), m_author(author)
    {
    }

    DummyEntity(const DummyEntity &other) : Entity(other), m_name(other.m_name), m_author(other.m_author)
    {
    }

    DummyEntity &operator=(const DummyEntity &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_name = other.m_name;
            m_author = other.m_author;
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
Q_DECLARE_METATYPE(Domain::DummyEntity)

#pragma once

#include "entity.h"

namespace Domain
{

class DummyOtherEntity : public Entity
{
    Q_OBJECT
    Q_PROPERTY(QString name READ name WRITE setName)

  public:
    DummyOtherEntity() : Entity(){};

    DummyOtherEntity(int id, const QUuid &uuid, const QString &name, const QString &author)
        : Entity(id, uuid, QDateTime(), QDateTime())
    {
        m_name = name;
    }

    DummyOtherEntity(int id, const QUuid &uuid, const QString &name, const QString &author,
                     const QDateTime &creationDate, const QDateTime &updateDate)
        : Entity(id, uuid, creationDate, updateDate), m_name(name)
    {
    }

    DummyOtherEntity(const DummyOtherEntity &other) : Entity(other), m_name(other.m_name)
    {
    }

    DummyOtherEntity &operator=(const DummyOtherEntity &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
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

Q_DECLARE_METATYPE(Domain::DummyOtherEntity)

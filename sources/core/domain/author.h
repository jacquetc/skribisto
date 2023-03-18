#pragma once

#include "domain_global.h"
#include "entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Author : public Entity
{
    Q_OBJECT
    Q_PROPERTY(QString name READ getName WRITE setName)
    Q_PROPERTY(QUuid relative READ getRelative WRITE setRelative)

  public:
    Author() : Entity(){};

    Author(const QUuid &uuid, const QString &name, const QUuid &relative) : Entity(uuid)
    {
        m_name = name;
        m_relative = relative;
    }

    Author(const QUuid &uuid, const QString &name, const QUuid &relative, const QDateTime &creationDate,
           const QDateTime &updateDate)
        : Entity(uuid, creationDate, updateDate), m_name(name), m_relative(relative)
    {
    }

    Author(const Author &other) : Entity(other), m_name(other.m_name), m_relative(other.m_relative)
    {
    }

    Author &operator=(const Author &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_name = other.m_name;
            m_relative = other.m_relative;
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

    QUuid getRelative() const
    {
        return m_relative;
    }

    QUuid relative() const
    {
        return m_relative;
    }

    void setRelative(const QUuid &relative)
    {
        m_relative = relative;
    }

  private:
    QString m_name;
    QUuid m_relative;
};

} // namespace Domain

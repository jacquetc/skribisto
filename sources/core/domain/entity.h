#pragma once

#include "domain_global.h"
#include <QDateTime>
#include <QObject>
#include <QUuid>

namespace Domain
{

class SKR_DOMAIN_EXPORT Entity : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QUuid uuid READ getUuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ getCreationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ getUpdateDate WRITE setUpdateDate)
  public:
    Entity() : QObject()
    {
    }
    Entity(const QUuid &uuid) : QObject(), m_uuid(uuid)
    {
    }
    Entity(const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate)
        : QObject(), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate)
    {
    }

    Entity(const Entity &other)
        : m_uuid(other.m_uuid), m_creationDate(other.m_creationDate), m_updateDate(other.m_updateDate)
    {
    }

    Entity &operator=(const Entity &other)
    {
        if (this != &other)
        {
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
        }
        return *this;
    }

    QUuid getUuid() const
    {
        return m_uuid;
    }

    QUuid uuid() const
    {
        return m_uuid;
    }

    void setUuid(const QUuid &uuid)
    {
        m_uuid = uuid;
    }

    QDateTime getCreationDate() const
    {
        return m_creationDate;
    }

    QDateTime creationDate() const
    {
        return m_creationDate;
    }

    void setCreationDate(const QDateTime &creationDate)
    {
        m_creationDate = creationDate;
    }

    QDateTime getUpdateDate() const
    {
        return m_updateDate;
    }

    QDateTime updateDate() const
    {
        return m_updateDate;
    }

    void setUpdateDate(const QDateTime &updateDate)
    {
        m_updateDate = updateDate;
    }

  private:
    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
};
} // namespace Domain

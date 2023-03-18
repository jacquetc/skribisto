#pragma once

#include "contracts_global.h"
#include <QDateTime>
#include <QObject>
#include <QString>
#include <QUuid>

namespace Contracts::DTO::Author
{

class SKR_CONTRACTS_EXPORT AuthorDTOBase : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString name READ getName WRITE setName)
    Q_PROPERTY(QUuid relative READ getRelative WRITE setRelative)
    Q_PROPERTY(QDateTime creationDate READ getCreationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ getUpdateDate WRITE setUpdateDate)

  public:
    AuthorDTOBase(QObject *parent = nullptr) : QObject(parent)
    {
    }
    AuthorDTOBase(const QString &name, const QUuid &relative) : QObject(), m_name(name), m_relative(relative)
    {
    }
    AuthorDTOBase(const AuthorDTOBase &other)
        : QObject(), m_name(other.m_name), m_relative(other.m_relative), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate)
    {
    }

    AuthorDTOBase &operator=(const AuthorDTOBase &other)
    {
        m_name = other.m_name;
        m_relative = other.m_relative;
        m_creationDate = other.m_creationDate;
        m_updateDate = other.m_updateDate;
        return *this;
    }
    bool operator==(const AuthorDTOBase &other) const
    {
        return m_name == other.m_name && m_relative == other.m_relative && m_creationDate == other.m_creationDate &&
               m_updateDate == other.m_updateDate;
    }
    QDateTime getCreationDate() const;
    QDateTime creationDate() const;
    void setCreationDate(const QDateTime &newCreationDate);
    QDateTime getUpdateDate() const;
    QDateTime updateDate() const;
    void setUpdateDate(const QDateTime &newUpdateDate);
    QString getName() const;
    QString name() const;
    void setName(const QString &newName);
    QUuid getRelative() const;
    QUuid relative() const;
    void setRelative(const QUuid &newRelative);

  private:
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_name;
    QUuid m_relative;
};

inline QDateTime AuthorDTOBase::getCreationDate() const
{
    return m_creationDate;
}

inline QDateTime AuthorDTOBase::creationDate() const
{
    return m_creationDate;
}

inline void AuthorDTOBase::setCreationDate(const QDateTime &newCreationDate)
{
    m_creationDate = newCreationDate;
}

inline QDateTime AuthorDTOBase::getUpdateDate() const
{
    return m_updateDate;
}

inline QDateTime AuthorDTOBase::updateDate() const
{
    return m_updateDate;
}

inline void AuthorDTOBase::setUpdateDate(const QDateTime &newUpdateDate)
{
    m_updateDate = newUpdateDate;
}

inline QUuid AuthorDTOBase::getRelative() const
{
    return m_relative;
}

inline QUuid AuthorDTOBase::relative() const
{
    return m_relative;
}

inline void AuthorDTOBase::setRelative(const QUuid &newRelative)
{
    m_relative = newRelative;
}

inline QString AuthorDTOBase::getName() const
{
    return m_name;
}

inline QString AuthorDTOBase::name() const
{
    return m_name;
}

inline void AuthorDTOBase::setName(const QString &newName)
{
    m_name = newName;
}

} // namespace Contracts::DTO::Author

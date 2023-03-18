#pragma once

#include "author_dto_base.h"
#include "contracts_global.h"
#include <QUuid>

namespace Contracts::DTO::Author
{

//-------------------------------------------------

class SKR_CONTRACTS_EXPORT AuthorDTO : public AuthorDTOBase
{
    Q_OBJECT
    Q_PROPERTY(QUuid uuid READ getUuid WRITE setUuid)
  public:
    AuthorDTO(QObject *parent = nullptr) : AuthorDTOBase(parent)
    {
    }

    AuthorDTO(const QUuid &uuid, const QString &name, const QUuid &relative)
        : AuthorDTOBase(name, relative), m_uuid(uuid)
    {
    }

    AuthorDTO(const AuthorDTO &other) : AuthorDTOBase(other), m_uuid(other.m_uuid)
    {
    }
    AuthorDTO &operator=(const AuthorDTO &other)
    {
        if (this != &other)
        {
            AuthorDTOBase::operator=(other);
            m_uuid = other.m_uuid;
        }
        return *this;
    }

    bool operator==(const AuthorDTO &other) const
    {

        return AuthorDTOBase::operator==(other) && m_uuid == other.m_uuid;
    }
    QUuid getUuid() const;
    QUuid uuid() const;
    void setUuid(const QUuid &newUuid);

  private:
    QUuid m_uuid;
};

//-------------------------------------------------

inline QUuid AuthorDTO::getUuid() const
{
    return m_uuid;
}
inline QUuid AuthorDTO::uuid() const
{
    return m_uuid;
}

inline void AuthorDTO::setUuid(const QUuid &newUuid)
{
    m_uuid = newUuid;
}

} // namespace Contracts::DTO::Author

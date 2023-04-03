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
    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
  public:
    AuthorDTO(QObject *parent = nullptr) : AuthorDTOBase(parent)
    {
    }

    AuthorDTO(int id, const QUuid &uuid, const QString &name, const QUuid &relative)
        : AuthorDTOBase(name, relative), m_id(id), m_uuid(uuid)
    {
    }

    AuthorDTO(const AuthorDTO &other) : AuthorDTOBase(other), m_id(other.m_id), m_uuid(other.m_uuid)
    {
    }
    AuthorDTO &operator=(const AuthorDTO &other)
    {
        if (this != &other)
        {
            AuthorDTOBase::operator=(other);
            m_id = other.m_id;
            m_uuid = other.m_uuid;
        }
        return *this;
    }

    bool operator==(const AuthorDTO &other) const
    {

        return AuthorDTOBase::operator==(other) && m_id == other.m_id && m_uuid == other.m_uuid;
    }
    int id() const;
    void setId(int newId);
    QUuid uuid() const;
    void setUuid(const QUuid &newUuid);

  private:
    int m_id;
    QUuid m_uuid;
};

inline QUuid AuthorDTO::uuid() const
{
    return m_uuid;
}

inline void AuthorDTO::setUuid(const QUuid &newUuid)
{
    m_uuid = newUuid;
}
//-------------------------------------------------

inline int AuthorDTO::id() const
{
    return m_id;
}

inline void AuthorDTO::setId(int newId)
{
    m_id = newId;
}

} // namespace Contracts::DTO::Author

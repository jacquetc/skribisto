#pragma once

#include "author_dto_base.h"
#include "contracts_global.h"
#include <QUuid>

namespace Contracts::DTO::Author
{

class SKR_CONTRACTS_EXPORT UpdateAuthorDTO : public AuthorDTOBase
{
    Q_OBJECT
    Q_PROPERTY(QUuid uuid READ getUuid WRITE setUuid)
  public:
    UpdateAuthorDTO(QObject *parent = nullptr) : AuthorDTOBase(parent)
    {
    }

    UpdateAuthorDTO(const QUuid &uuid, const QString &name, const QUuid &relative)
        : AuthorDTOBase(name, relative), m_uuid(uuid)
    {
    }

    UpdateAuthorDTO(const UpdateAuthorDTO &other) : AuthorDTOBase(other), m_uuid(other.m_uuid)
    {
    }
    UpdateAuthorDTO &operator=(const UpdateAuthorDTO &other)
    {
        if (this != &other)
        {
            AuthorDTOBase::operator=(other);
            m_uuid = other.m_uuid;
        }
        return *this;
    }
    QUuid getUuid() const;
    QUuid uuid() const;
    void setUuid(const QUuid &newUuid);

  private:
    QUuid m_uuid;
};

//-------------------------------------------------

inline QUuid UpdateAuthorDTO::getUuid() const
{
    return m_uuid;
}
inline QUuid UpdateAuthorDTO::uuid() const
{
    return m_uuid;
}

inline void UpdateAuthorDTO::setUuid(const QUuid &newUuid)
{
    m_uuid = newUuid;
}

} // namespace Contracts::DTO::Author

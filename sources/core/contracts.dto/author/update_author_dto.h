#pragma once

#include "author_dto_base.h"
#include <QUuid>
#include <QQmlEngine>

namespace Contracts::DTO::Author
{

class  UpdateAuthorDTO : public AuthorDTOBase
{
    Q_OBJECT
    QML_ELEMENT
    Q_PROPERTY(int id READ id WRITE setId)
  public:
    UpdateAuthorDTO(QObject *parent = nullptr) : AuthorDTOBase(parent)
    {
    }

    UpdateAuthorDTO(int id, const QString &name) : AuthorDTOBase(name), m_id(id)
    {
    }

    UpdateAuthorDTO(const UpdateAuthorDTO &other) : AuthorDTOBase(other), m_id(other.m_id)
    {
    }
    UpdateAuthorDTO &operator=(const UpdateAuthorDTO &other)
    {
        if (this != &other)
        {
            AuthorDTOBase::operator=(other);
            m_id = other.m_id;
        }
        return *this;
    }
    int id() const;
    void setId(int newId);

  private:
    int m_id;
};

//-------------------------------------------------

inline int UpdateAuthorDTO::id() const
{
    return m_id;
}

inline void UpdateAuthorDTO::setId(int newId)
{
    m_id = newId;
}

} // namespace Contracts::DTO::Author

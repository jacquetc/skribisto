#pragma once

#include "QtCore/qobject.h"
#include "author_dto_base.h"
#include <QDateTime>
#include <QObject>

namespace Contracts::DTO::Author
{

//-------------------------------------------------

class CreateAuthorDTO : public AuthorDTOBase
{
    Q_OBJECT
  public:
    CreateAuthorDTO(QObject *parent = nullptr) : AuthorDTOBase(parent)
    {
    }
    CreateAuthorDTO(const QString &name) : AuthorDTOBase(name)
    {
    }

    CreateAuthorDTO(const CreateAuthorDTO &other) : AuthorDTOBase(other)
    {
    }
    CreateAuthorDTO &operator=(const CreateAuthorDTO &other)
    {
        if (this != &other)
        {
            AuthorDTOBase::operator=(other);
        }
        return *this;
    }

  private:
};

} // namespace Contracts::DTO::Author

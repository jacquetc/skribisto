#pragma once

#include "QtCore/qobject.h"
#include "author_dto_base.h"
#include "contracts_global.h"
#include <QDateTime>
#include <QObject>

namespace Contracts::DTO::Author
{

//-------------------------------------------------

class SKR_CONTRACTS_EXPORT CreateAuthorDTO : public AuthorDTOBase
{
    Q_OBJECT
  public:
    CreateAuthorDTO(QObject *parent = nullptr) : AuthorDTOBase(parent)
    {
    }
    CreateAuthorDTO(const QString &name, const QUuid &relative) : AuthorDTOBase(name, relative)
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

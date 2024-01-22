// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "book/update_book_dto.h"


#include "repository/interface_book_repository.h"

#include <qleany/common/result.h>

using namespace Qleany;

using namespace Skribisto::Contracts::Repository;

using namespace Skribisto::Contracts::DTO::Book;

namespace Skribisto::Contracts::CQRS::Book::Validators
{
class UpdateBookCommandValidator
{
  public:
    UpdateBookCommandValidator(InterfaceBookRepository *bookRepository)
        :  m_bookRepository(bookRepository)
    {
    }

    Result<void> validate(const UpdateBookDTO &dto) const

    {




        Result<bool> existsResult = m_bookRepository->exists(dto.id());

        if (!existsResult.value())
        {
            return Result<void>(QLN_ERROR_1(Q_FUNC_INFO, Error::Critical, "id_not_found"));
        }





        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceBookRepository *m_bookRepository;

};
} // namespace Skribisto::Contracts::CQRS::Book::Validators
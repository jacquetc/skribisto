// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "chapter/update_chapter_dto.h"


#include "repository/interface_chapter_repository.h"

#include <qleany/common/result.h>

using namespace Qleany;

using namespace Skribisto::Contracts::Repository;

using namespace Skribisto::Contracts::DTO::Chapter;

namespace Skribisto::Contracts::CQRS::Chapter::Validators
{
class UpdateChapterCommandValidator
{
  public:
    UpdateChapterCommandValidator(InterfaceChapterRepository *chapterRepository)
        :  m_chapterRepository(chapterRepository)
    {
    }

    Result<void> validate(const UpdateChapterDTO &dto) const

    {




        Result<bool> existsResult = m_chapterRepository->exists(dto.id());

        if (!existsResult.value())
        {
            return Result<void>(QLN_ERROR_1(Q_FUNC_INFO, Error::Critical, "id_not_found"));
        }





        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceChapterRepository *m_chapterRepository;

};
} // namespace Skribisto::Contracts::CQRS::Chapter::Validators
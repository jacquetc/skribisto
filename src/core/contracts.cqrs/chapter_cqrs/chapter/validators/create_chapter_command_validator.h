// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "chapter/create_chapter_dto.h"


#include "repository/interface_chapter_repository.h"

#include <qleany/common/result.h>

using namespace Qleany;

using namespace Skribisto::Contracts::Repository;

using namespace Skribisto::Contracts::DTO::Chapter;

namespace Skribisto::Contracts::CQRS::Chapter::Validators
{
class CreateChapterCommandValidator
{
  public:
    CreateChapterCommandValidator(InterfaceChapterRepository *chapterRepository)
        :  m_chapterRepository(chapterRepository)
    {
    }

    Result<void> validate(const CreateChapterDTO &dto) const

    {





        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceChapterRepository *m_chapterRepository;

};
} // namespace Skribisto::Contracts::CQRS::Chapter::Validators
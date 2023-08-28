#pragma once

#include "chapter/create_chapter_dto.h"

#include "persistence/interface_chapter_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

using namespace Contracts::DTO::Chapter;

namespace Contracts::CQRS::Chapter::Validators
{
class CreateChapterCommandValidator
{
  public:
    CreateChapterCommandValidator(InterfaceChapterRepository *chapterRepository)
        : m_chapterRepository(chapterRepository)
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
} // namespace Contracts::CQRS::Chapter::Validators
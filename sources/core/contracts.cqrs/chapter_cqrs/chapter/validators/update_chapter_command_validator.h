#pragma once

#include "chapter/update_chapter_dto.h"

#include "persistence/interface_chapter_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

using namespace Contracts::DTO::Chapter;

namespace Contracts::CQRS::Chapter::Validators
{
class UpdateChapterCommandValidator
{
  public:
    UpdateChapterCommandValidator(InterfaceChapterRepository *chapterRepository)
        : m_chapterRepository(chapterRepository)
    {
    }

    Result<void> validate(const UpdateChapterDTO &dto) const

    {

        Result<bool> existsResult = m_chapterRepository->exists(dto.id());

        if (!existsResult.value())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "id_already_exists"));
        }

        // Return that is Ok :
        return Result<void>();
    }

  private:
    InterfaceChapterRepository *m_chapterRepository;
};
} // namespace Contracts::CQRS::Chapter::Validators
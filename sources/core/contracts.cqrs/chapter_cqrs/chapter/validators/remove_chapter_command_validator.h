#pragma once

#include "persistence/interface_chapter_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

namespace Contracts::CQRS::Chapter::Validators
{
class RemoveChapterCommandValidator
{
  public:
    RemoveChapterCommandValidator(QSharedPointer<InterfaceChapterRepository> chapterRepository)
        : m_chapterRepository(chapterRepository)
    {
    }

    Result<void> validate(int id) const

    {

        Result<bool> existsResult = m_chapterRepository->exists(id);

        if (!existsResult.value())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "id_already_exists"));
        }

        // Return that is Ok :
        return Result<void>();
    }

  private:
    QSharedPointer<InterfaceChapterRepository> m_chapterRepository;
};
} // namespace Contracts::CQRS::Chapter::Validators
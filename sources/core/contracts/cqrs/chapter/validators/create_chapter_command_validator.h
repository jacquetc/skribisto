#pragma once

#include "contracts_global.h"
#include "create_chapter_dto.h"
#include "persistence/interface_chapter_repository.h"
#include "result.h"

using namespace Contracts::Persistence;
using namespace Contracts::DTO::Chapter;

namespace Contracts::CQRS::Chapter::Validators
{
class SKR_CONTRACTS_EXPORT CreateChapterCommandValidator
{
  public:
    CreateChapterCommandValidator(QSharedPointer<InterfaceChapterRepository> chapterRepository)
        : m_repository(chapterRepository)
    {
    }

    Result<void> validate(const CreateChapterDTO &dto) const
    {

        //        if (dto.relative().isNull())
        //        {
        //            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "relative_uuid_missing"));
        //        }

        // Return that is Ok :
        return Result<void>();
    }

  private:
    QSharedPointer<InterfaceChapterRepository> m_repository;
};
} // namespace Contracts::CQRS::Chapter::Validators

#pragma once

#include "contracts_global.h"
#include "persistence/interface_chapter_repository.h"
#include "result.h"
#include "update_chapter_dto.h"

using namespace Contracts::Persistence;
using namespace Contracts::DTO::Chapter;

namespace Contracts::CQRS::Chapter::Validators
{
class SKR_CONTRACTS_EXPORT UpdateChapterCommandValidator
{
  public:
    UpdateChapterCommandValidator(QSharedPointer<InterfaceChapterRepository> chapterRepository)
        : m_repository(chapterRepository)
    {
    }

    Result<void> validate(const UpdateChapterDTO &dto) const
    {

        Result<bool> exists = m_repository->exists(dto.id());
        if (!exists.value())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "id_already_exists"));
        }
        //        if (!dto.relative().isNull())
        //        {
        //            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "project_uuid_missing"));
        //        }

        // Return that is Ok :
        return Result<void>();
    }

  private:
    QSharedPointer<InterfaceChapterRepository> m_repository;
};
} // namespace Contracts::CQRS::Chapter::Validators

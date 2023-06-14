#pragma once

#include "chapter/move_chapter_dto.h"

#include "persistence/interface_book_repository.h"

#include "persistence/interface_chapter_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

using namespace Contracts::DTO::Chapter;

namespace Contracts::CQRS::Chapter::Validators
{
class MoveChapterCommandValidator
{
  public:
    MoveChapterCommandValidator(QSharedPointer<InterfaceBookRepository> bookRepository,
                                QSharedPointer<InterfaceChapterRepository> chapterRepository)
        : m_bookRepository(bookRepository), m_chapterRepository(chapterRepository)
    {
    }

    Result<void> validate(const MoveChapterDTO &dto) const

    {

        // Return that is Ok :
        return Result<void>();
    }

  private:
    QSharedPointer<InterfaceBookRepository> m_bookRepository;

    QSharedPointer<InterfaceChapterRepository> m_chapterRepository;
};
} // namespace Contracts::CQRS::Chapter::Validators
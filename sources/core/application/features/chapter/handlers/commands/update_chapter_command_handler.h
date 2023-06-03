#pragma once

#include "application_global.h"
#include "chapter_dto.h"
#include "cqrs/chapter/commands/update_chapter_command.h"
#include "handler.h"
#include "persistence/interface_chapter_repository.h"
#include "result.h"

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Commands;

namespace Application::Features::Chapter::Commands
{
class SKR_APPLICATION_EXPORT UpdateChapterCommandHandler : public Handler

{
    Q_OBJECT
  public:
    UpdateChapterCommandHandler(QSharedPointer<InterfaceChapterRepository> repository);
    Result<ChapterDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateChapterCommand &request);
    Result<ChapterDTO> restore();

  signals:
    void chapterUpdated(Contracts::DTO::Chapter::ChapterDTO chapterDto);

  private:
    QSharedPointer<InterfaceChapterRepository> m_repository;
    Result<ChapterDTO> handleImpl(const UpdateChapterCommand &request);
    Result<ChapterDTO> restoreImpl();
    Result<ChapterDTO> saveOldState();
    Result<ChapterDTO> m_oldState;
};
} // namespace Application::Features::Chapter::Commands

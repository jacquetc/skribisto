#pragma once

#include "application_global.h"
#include "chapter_dto.h"
#include "cqrs/chapter/commands/remove_chapter_command.h"
#include "handler.h"
#include "persistence/interface_chapter_repository.h"
#include "result.h"

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Commands;

namespace Application::Features::Chapter::Commands
{
class SKR_APPLICATION_EXPORT RemoveChapterCommandHandler : public Handler
{
    Q_OBJECT
  public:
    RemoveChapterCommandHandler(QSharedPointer<InterfaceChapterRepository> repository);
    Result<ChapterDTO> handle(QPromise<Result<void>> &progressPromise, const RemoveChapterCommand &request);
    Result<ChapterDTO> restore();

  signals:
    void chapterCreated(Contracts::DTO::Chapter::ChapterDTO result);
    void chapterRemoved(Contracts::DTO::Chapter::ChapterDTO result);

  private:
    QSharedPointer<InterfaceChapterRepository> m_repository;
    Result<ChapterDTO> handleImpl(const RemoveChapterCommand &request);
    Result<ChapterDTO> restoreImpl();
    Result<ChapterDTO> saveOldState();
    Result<ChapterDTO> m_oldState;
};
} // namespace Application::Features::Chapter::Commands

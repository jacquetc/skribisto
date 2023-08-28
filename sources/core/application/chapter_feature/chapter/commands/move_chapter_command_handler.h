#pragma once

#include "application_chapter_export.h"
#include "chapter/commands/move_chapter_command.h"
#include "chapter/move_chapter_reply_dto.h"

#include "persistence/interface_book_repository.h"
#include "persistence/interface_chapter_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Commands;

namespace Application::Features::Chapter::Commands
{
class SKRIBISTO_APPLICATION_CHAPTER_EXPORT MoveChapterCommandHandler : public QObject
{
    Q_OBJECT
  public:
    MoveChapterCommandHandler(InterfaceBookRepository *bookRepository, InterfaceChapterRepository *chapterRepository);

    Result<MoveChapterReplyDTO> handle(QPromise<Result<void>> &progressPromise, const MoveChapterCommand &request);

    Result<MoveChapterReplyDTO> restore();

  signals:
    void moveChapterChanged(Contracts::DTO::Chapter::MoveChapterReplyDTO moveChapterReplyDto);

  private:
    InterfaceBookRepository *m_bookRepository;
    InterfaceChapterRepository *m_chapterRepository;
    Result<MoveChapterReplyDTO> handleImpl(QPromise<Result<void>> &progressPromise, const MoveChapterCommand &request);

    Result<MoveChapterReplyDTO> restoreImpl();
    Result<MoveChapterReplyDTO> m_newState;

    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Chapter::Commands
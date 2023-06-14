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
    MoveChapterCommandHandler(QSharedPointer<InterfaceBookRepository> bookRepository,
                              QSharedPointer<InterfaceChapterRepository> chapterRepository);

    Result<MoveChapterReplyDTO> handle(QPromise<Result<void>> &progressPromise, const MoveChapterCommand &request);

  signals:
    void moveChapterChanged(Contracts::DTO::Chapter::MoveChapterReplyDTO moveChapterReplyDto);

  private:
    QSharedPointer<InterfaceBookRepository> m_bookRepository;
    QSharedPointer<InterfaceChapterRepository> m_chapterRepository;
    Result<MoveChapterReplyDTO> handleImpl(QPromise<Result<void>> &progressPromise, const MoveChapterCommand &request);

    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Chapter::Commands
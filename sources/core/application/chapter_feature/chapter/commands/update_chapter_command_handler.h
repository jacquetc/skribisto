#pragma once

#include "application_chapter_export.h"
#include "chapter/chapter_dto.h"
#include "chapter/commands/update_chapter_command.h"

#include "persistence/interface_chapter_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Commands;

namespace Application::Features::Chapter::Commands
{
class SKRIBISTO_APPLICATION_CHAPTER_EXPORT UpdateChapterCommandHandler : public QObject

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
    Result<ChapterDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdateChapterCommand &request);
    Result<ChapterDTO> restoreImpl();
    Result<ChapterDTO> saveOldState();
    Result<ChapterDTO> m_newState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Chapter::Commands
#pragma once

#include "application_chapter_export.h"
#include "chapter/chapter_dto.h"
#include "chapter/commands/create_chapter_command.h"
#include "persistence/interface_chapter_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Commands;

namespace Application::Features::Chapter::Commands
{
class SKRIBISTO_APPLICATION_CHAPTER_EXPORT CreateChapterCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateChapterCommandHandler(QSharedPointer<InterfaceChapterRepository> repository);

    Result<ChapterDTO> handle(QPromise<Result<void>> &progressPromise, const CreateChapterCommand &request);
    Result<ChapterDTO> restore();

  signals:
    void chapterCreated(Contracts::DTO::Chapter::ChapterDTO chapterDto);
    void chapterRemoved(int id);

  private:
    QSharedPointer<InterfaceChapterRepository> m_repository; // A pointer to the interface repositories object.
    Result<ChapterDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateChapterCommand &request);
    Result<ChapterDTO> restoreImpl();
    Result<ChapterDTO> m_newState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Chapter::Commands
#pragma once

#include "application_global.h"
#include "chapter_dto.h"
#include "cqrs/chapter/commands/create_chapter_command.h"
#include "handler.h"
#include "persistence/interface_chapter_repository.h"
#include "result.h"

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Commands;

namespace Application::Features::Chapter::Commands
{
class SKR_APPLICATION_EXPORT CreateChapterCommandHandler : public Handler
{
    Q_OBJECT
  public:
    CreateChapterCommandHandler(QSharedPointer<InterfaceChapterRepository> repository);

    Result<ChapterDTO> handle(QPromise<Result<void>> &progressPromise, const CreateChapterCommand &request);
    Result<ChapterDTO> restore();

  signals:
    void chapterCreated(Contracts::DTO::Chapter::ChapterDTO chapterDto);
    void chapterRemoved(Contracts::DTO::Chapter::ChapterDTO chapterDto);

  private:
    QSharedPointer<InterfaceChapterRepository> m_repository; // A pointer to the interface repositories object.
    Result<ChapterDTO> handleImpl(const CreateChapterCommand &request);
    Result<ChapterDTO> restoreImpl();
    Result<ChapterDTO> m_newState;
};

struct CallOnce
{
    CallOnce()
    {
        registerMappings();
    }
    void registerMappings();

};
// This line will ensure the function is called once at the start of the program.
static CallOnce callOnceInstance;


} // namespace Application::Features::Chapter::Commands
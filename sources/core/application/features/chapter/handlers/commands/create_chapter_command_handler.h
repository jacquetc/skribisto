/**
 * @file create_chapter_command_handler.h
 * @brief Contains the definition of the CreateChapterCommandHandler class.
 */

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
/**
 * @brief Handles the CreateChapterCommand and persists the new chapter to the data store.
 */
class SKR_APPLICATION_EXPORT CreateChapterCommandHandler : public Handler
{
    Q_OBJECT
  public:
    /**
     * @brief Constructs a new CreateChapterCommandHandler object.
     * @param repositories A pointer to the interface repositories object.
     */
    CreateChapterCommandHandler(QSharedPointer<InterfaceChapterRepository> repository);

    /**
     * @brief Handles a CreateChapterCommand object and returns the UUID of the newly created chapter.
     * @param request The CreateChapterCommand object to handle.
     * @return A Result object containing the UUID of the newly created chapter, or an error message if the operation
     * failed.
     */
    Result<ChapterDTO> handle(QPromise<Result<void>> &progressPromise, const CreateChapterCommand &request);
    Result<ChapterDTO> restore();

  signals:
    void chapterCreated(Contracts::DTO::Chapter::ChapterDTO chapterDto);
    void chapterRemoved(Contracts::DTO::Chapter::ChapterDTO chapterDto);

  private:
    QSharedPointer<InterfaceChapterRepository> m_repository; // A pointer to the interface repositories object.
    Result<ChapterDTO> handleImpl(const CreateChapterCommand &request);
    Result<ChapterDTO> restoreImpl();
    Result<ChapterDTO> m_oldState;
};
} // namespace Application::Features::Chapter::Commands

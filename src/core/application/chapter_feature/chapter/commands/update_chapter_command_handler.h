// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_chapter_export.h"
#include "chapter/chapter_dto.h"
#include "chapter/commands/update_chapter_command.h"

#include "repository/interface_chapter_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Chapter;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Chapter::Commands;

namespace Skribisto::Application::Features::Chapter::Commands
{
class SKRIBISTO_APPLICATION_CHAPTER_EXPORT UpdateChapterCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateChapterCommandHandler(InterfaceChapterRepository *repository);
    Result<ChapterDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateChapterCommand &request);
    Result<ChapterDTO> restore();

  signals:
    void chapterUpdated(Skribisto::Contracts::DTO::Chapter::ChapterDTO chapterDto);
    void chapterDetailsUpdated(int id);

  private:
    InterfaceChapterRepository *m_repository;
    Result<ChapterDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdateChapterCommand &request);
    Result<ChapterDTO> restoreImpl();
    Result<ChapterDTO> saveOldState();
    Result<ChapterDTO> m_undoState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::Chapter::Commands
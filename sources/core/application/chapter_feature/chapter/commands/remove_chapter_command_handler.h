#pragma once

#include "application_chapter_export.h"
#include "chapter/chapter_dto.h"
#include "chapter/commands/remove_chapter_command.h"

#include "persistence/interface_chapter_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Commands;

namespace Application::Features::Chapter::Commands
{
class SKRIBISTO_APPLICATION_CHAPTER_EXPORT RemoveChapterCommandHandler : public QObject
{
    Q_OBJECT
  public:
    RemoveChapterCommandHandler(QSharedPointer<InterfaceChapterRepository> repository);
    Result<int> handle(QPromise<Result<void>> &progressPromise, const RemoveChapterCommand &request);
    Result<int> restore();

  signals:
    void chapterCreated(Contracts::DTO::Chapter::ChapterDTO chapterDto);
    void chapterRemoved(int id);

  private:
    QSharedPointer<InterfaceChapterRepository> m_repository;
    Result<int> handleImpl(QPromise<Result<void>> &progressPromise, const RemoveChapterCommand &request);
    Result<int> restoreImpl();
    Domain::Chapter m_oldState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Chapter::Commands
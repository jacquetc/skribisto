#pragma once

#include "application_chapter_export.h"
#include "chapter/chapter_dto.h"
#include "chapter/queries/get_chapter_query.h"

#include "persistence/interface_chapter_repository.h"
#include <QPromise>

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Queries;

namespace Application::Features::Chapter::Queries
{
class SKRIBISTO_APPLICATION_CHAPTER_EXPORT GetChapterQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetChapterQueryHandler(InterfaceChapterRepository *repository);
    Result<ChapterDTO> handle(QPromise<Result<void>> &progressPromise, const GetChapterQuery &query);

  private:
    InterfaceChapterRepository *m_repository;
    Result<ChapterDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetChapterQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Chapter::Queries
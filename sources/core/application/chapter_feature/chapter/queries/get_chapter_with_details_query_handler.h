#pragma once

#include "application_chapter_export.h"
#include "chapter/chapter_with_details_dto.h"
#include "chapter/queries/get_chapter_query.h"

#include "persistence/interface_chapter_repository.h"
#include <QPromise>

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Queries;

namespace Application::Features::Chapter::Queries
{
class SKRIBISTO_APPLICATION_CHAPTER_EXPORT GetChapterWithDetailsQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetChapterWithDetailsQueryHandler(QSharedPointer<InterfaceChapterRepository> repository);
    Result<ChapterWithDetailsDTO> handle(QPromise<Result<void>> &progressPromise, const GetChapterQuery &query);

  private:
    QSharedPointer<InterfaceChapterRepository> m_repository;
    Result<ChapterWithDetailsDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetChapterQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Chapter::Queries
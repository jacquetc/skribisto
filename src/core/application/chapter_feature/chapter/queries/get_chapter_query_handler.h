// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_chapter_export.h"
#include "chapter/chapter_dto.h"
#include "chapter/queries/get_chapter_query.h"

#include "repository/interface_chapter_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Chapter;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Chapter::Queries;

namespace Skribisto::Application::Features::Chapter::Queries
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

} // namespace Skribisto::Application::Features::Chapter::Queries
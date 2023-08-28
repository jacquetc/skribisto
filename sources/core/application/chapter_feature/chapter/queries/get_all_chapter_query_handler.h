#pragma once

#include "application_chapter_export.h"
#include "chapter/chapter_dto.h"

#include "persistence/interface_chapter_repository.h"
#include <QPromise>

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;

namespace Application::Features::Chapter::Queries
{
class SKRIBISTO_APPLICATION_CHAPTER_EXPORT GetAllChapterQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllChapterQueryHandler(InterfaceChapterRepository *repository);
    Result<QList<ChapterDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    InterfaceChapterRepository *m_repository;
    Result<QList<ChapterDTO>> handleImpl(QPromise<Result<void>> &progressPromise);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Chapter::Queries
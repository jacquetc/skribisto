#pragma once

#include "application_global.h"
#include "chapter_dto.h"
#include "handler.h"
#include "persistence/interface_chapter_repository.h"

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;

namespace Application::Features::Chapter::Queries
{
class SKR_APPLICATION_EXPORT GetChapterListRequestHandler : public Handler
{
    Q_OBJECT
  public:
    GetChapterListRequestHandler(QSharedPointer<InterfaceChapterRepository> repository);
    Result<QList<ChapterDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    QSharedPointer<InterfaceChapterRepository> m_repository;
    Result<QList<ChapterDTO>> handleImpl();
};
} // namespace Application::Features::Chapter::Queries

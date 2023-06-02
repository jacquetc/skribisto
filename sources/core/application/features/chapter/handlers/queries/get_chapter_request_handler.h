#pragma once

#include "application_global.h"
#include "chapter_dto.h"
#include "cqrs/chapter/requests/get_chapter_request.h"
#include "handler.h"
#include "persistence/interface_chapter_repository.h"

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Requests;

namespace Application::Features::Chapter::Queries
{
class SKR_APPLICATION_EXPORT GetChapterRequestHandler : public Handler
{
    Q_OBJECT
  public:
    GetChapterRequestHandler(QSharedPointer<InterfaceChapterRepository> repository);
    Result<ChapterDTO> handle(QPromise<Result<void>> &progressPromise, const GetChapterRequest &request);

  private:
    QSharedPointer<InterfaceChapterRepository> m_repository;
    Result<ChapterDTO> handleImpl(const GetChapterRequest &request);
};
} // namespace Application::Features::Chapter::Queries

#pragma once

#include "application_writing_export.h"
#include "writing/get_scene_paragraph_reply_dto.h"
#include "writing/queries/get_scene_paragraph_query.h"

#include "persistence/interface_scene_paragraph_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Writing;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Writing::Queries;

namespace Application::Features::Writing::Queries
{
class SKRIBISTO_APPLICATION_WRITING_EXPORT GetSceneParagraphQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetSceneParagraphQueryHandler(QSharedPointer<InterfaceSceneParagraphRepository> sceneParagraphRepository);

    Result<GetSceneParagraphReplyDTO> handle(QPromise<Result<void>> &progressPromise,
                                             const GetSceneParagraphQuery &request);

  signals:
    void getSceneParagraphChanged(Contracts::DTO::Writing::GetSceneParagraphReplyDTO getSceneParagraphReplyDTO);

  private:
    QSharedPointer<InterfaceSceneParagraphRepository> m_sceneParagraphRepository;
    Result<GetSceneParagraphReplyDTO> handleImpl(QPromise<Result<void>> &progressPromise,
                                                 const GetSceneParagraphQuery &request);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Writing::Queries
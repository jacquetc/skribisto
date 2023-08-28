#pragma once

#include "application_writing_export.h"
#include "writing/get_scene_paragraph_id_reply_dto.h"
#include "writing/queries/get_scene_paragraph_id_query.h"

#include "persistence/interface_scene_paragraph_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Writing;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Writing::Queries;

namespace Application::Features::Writing::Queries
{
class SKRIBISTO_APPLICATION_WRITING_EXPORT GetSceneParagraphIdQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetSceneParagraphIdQueryHandler(InterfaceSceneParagraphRepository *sceneParagraphRepository);

    Result<GetSceneParagraphIdReplyDTO> handle(QPromise<Result<void>> &progressPromise,
                                               const GetSceneParagraphIdQuery &request);

  signals:
    void getSceneParagraphIdChanged(Contracts::DTO::Writing::GetSceneParagraphIdReplyDTO getSceneParagraphIdReplyDTO);

  private:
    QSharedPointer<InterfaceSceneParagraphRepository> m_sceneParagraphRepository;
    Result<GetSceneParagraphIdReplyDTO> handleImpl(QPromise<Result<void>> &progressPromise,
                                                   const GetSceneParagraphIdQuery &request);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Writing::Queries

#pragma once

#include "application_writing_export.h"
#include "writing/get_all_scene_paragraphs_reply_dto.h"
#include "writing/queries/get_all_scene_paragraphs_query.h"

#include "persistence/interface_scene_paragraph_repository.h"
#include "persistence/interface_scene_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Writing;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Writing::Queries;

namespace Application::Features::Writing::Queries
{
class SKRIBISTO_APPLICATION_WRITING_EXPORT GetAllSceneParagraphsQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllSceneParagraphsQueryHandler(InterfaceSceneRepository *sceneRepository,
                                      InterfaceSceneParagraphRepository *sceneParagraphRepository);

    Result<GetAllSceneParagraphsReplyDTO> handle(QPromise<Result<void>> &progressPromise,
                                                 const GetAllSceneParagraphsQuery &request);

  signals:
    void getAllSceneParagraphsChanged(
        Contracts::DTO::Writing::GetAllSceneParagraphsReplyDTO getAllSceneParagraphsReplyDTO);

  private:
    QSharedPointer<InterfaceSceneRepository> m_sceneRepository;
    QSharedPointer<InterfaceSceneParagraphRepository> m_sceneParagraphRepository;
    Result<GetAllSceneParagraphsReplyDTO> handleImpl(QPromise<Result<void>> &progressPromise,
                                                     const GetAllSceneParagraphsQuery &request);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Writing::Queries

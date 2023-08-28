#pragma once

#include "application_scene_export.h"
#include "scene/commands/move_scene_command.h"
#include "scene/move_scene_reply_dto.h"

#include "persistence/interface_chapter_repository.h"
#include "persistence/interface_scene_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Scene;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Scene::Commands;

namespace Application::Features::Scene::Commands
{
class SKRIBISTO_APPLICATION_SCENE_EXPORT MoveSceneCommandHandler : public QObject
{
    Q_OBJECT
  public:
    MoveSceneCommandHandler(InterfaceChapterRepository *chapterRepository, InterfaceSceneRepository *sceneRepository);

    Result<MoveSceneReplyDTO> handle(QPromise<Result<void>> &progressPromise, const MoveSceneCommand &request);

    Result<MoveSceneReplyDTO> restore();

  signals:
    void moveSceneChanged(Contracts::DTO::Scene::MoveSceneReplyDTO moveSceneReplyDto);

  private:
    InterfaceChapterRepository *m_chapterRepository;
    InterfaceSceneRepository *m_sceneRepository;
    Result<MoveSceneReplyDTO> handleImpl(QPromise<Result<void>> &progressPromise, const MoveSceneCommand &request);

    Result<MoveSceneReplyDTO> restoreImpl();
    Result<MoveSceneReplyDTO> m_newState;

    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Scene::Commands
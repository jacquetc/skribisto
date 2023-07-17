#pragma once

#include "application_writing_export.h"
#include "writing/commands/update_scene_paragraph_command.h"
#include "writing/scene_paragraph_changed_dto.h"

#include "persistence/interface_scene_paragraph_repository.h"
#include "persistence/interface_scene_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Writing;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Writing::Commands;

namespace Application::Features::Writing::Commands
{
class SKRIBISTO_APPLICATION_WRITING_EXPORT UpdateSceneParagraphCommandHandler : public QObject
{
    Q_OBJECT
  public:
    UpdateSceneParagraphCommandHandler(QSharedPointer<InterfaceSceneRepository> sceneRepository,
                                       QSharedPointer<InterfaceSceneParagraphRepository> sceneParagraphRepository);

    Result<SceneParagraphChangedDTO> handle(QPromise<Result<void>> &progressPromise,
                                            const UpdateSceneParagraphCommand &request);

    Result<SceneParagraphChangedDTO> restore();

  signals:
    void updateSceneParagraphChanged(Contracts::DTO::Writing::SceneParagraphChangedDTO sceneParagraphChangedDto);

  private:
    QSharedPointer<InterfaceSceneRepository> m_sceneRepository;
    QSharedPointer<InterfaceSceneParagraphRepository> m_sceneParagraphRepository;
    Result<SceneParagraphChangedDTO> handleImpl(QPromise<Result<void>> &progressPromise,
                                                const UpdateSceneParagraphCommand &request);

    Result<SceneParagraphChangedDTO> restoreImpl();
    Result<SceneParagraphChangedDTO> m_newState;

    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Writing::Commands
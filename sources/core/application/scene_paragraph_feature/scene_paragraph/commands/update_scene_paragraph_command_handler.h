#pragma once

#include "application_scene_paragraph_export.h"
#include "scene_paragraph/commands/update_scene_paragraph_command.h"
#include "scene_paragraph/scene_paragraph_dto.h"

#include "persistence/interface_scene_paragraph_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::SceneParagraph;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::SceneParagraph::Commands;

namespace Application::Features::SceneParagraph::Commands
{
class SKRIBISTO_APPLICATION_SCENE_PARAGRAPH_EXPORT UpdateSceneParagraphCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateSceneParagraphCommandHandler(QSharedPointer<InterfaceSceneParagraphRepository> repository);
    Result<SceneParagraphDTO> handle(QPromise<Result<void>> &progressPromise,
                                     const UpdateSceneParagraphCommand &request);
    Result<SceneParagraphDTO> restore();

  signals:
    void sceneParagraphUpdated(Contracts::DTO::SceneParagraph::SceneParagraphDTO sceneParagraphDto);

  private:
    QSharedPointer<InterfaceSceneParagraphRepository> m_repository;
    Result<SceneParagraphDTO> handleImpl(QPromise<Result<void>> &progressPromise,
                                         const UpdateSceneParagraphCommand &request);
    Result<SceneParagraphDTO> restoreImpl();
    Result<SceneParagraphDTO> saveOldState();
    Result<SceneParagraphDTO> m_newState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::SceneParagraph::Commands
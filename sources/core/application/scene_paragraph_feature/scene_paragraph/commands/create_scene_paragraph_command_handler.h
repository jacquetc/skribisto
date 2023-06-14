#pragma once

#include "application_scene_paragraph_export.h"
#include "persistence/interface_scene_paragraph_repository.h"
#include "result.h"
#include "scene_paragraph/commands/create_scene_paragraph_command.h"
#include "scene_paragraph/scene_paragraph_dto.h"
#include <QPromise>

using namespace Contracts::DTO::SceneParagraph;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::SceneParagraph::Commands;

namespace Application::Features::SceneParagraph::Commands
{
class SKRIBISTO_APPLICATION_SCENE_PARAGRAPH_EXPORT CreateSceneParagraphCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateSceneParagraphCommandHandler(QSharedPointer<InterfaceSceneParagraphRepository> repository);

    Result<SceneParagraphDTO> handle(QPromise<Result<void>> &progressPromise,
                                     const CreateSceneParagraphCommand &request);
    Result<SceneParagraphDTO> restore();

  signals:
    void sceneParagraphCreated(Contracts::DTO::SceneParagraph::SceneParagraphDTO sceneParagraphDto);
    void sceneParagraphRemoved(int id);

  private:
    QSharedPointer<InterfaceSceneParagraphRepository> m_repository; // A pointer to the interface repositories object.
    Result<SceneParagraphDTO> handleImpl(QPromise<Result<void>> &progressPromise,
                                         const CreateSceneParagraphCommand &request);
    Result<SceneParagraphDTO> restoreImpl();
    Result<SceneParagraphDTO> m_newState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::SceneParagraph::Commands
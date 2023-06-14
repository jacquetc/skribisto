#pragma once

#include "application_scene_paragraph_export.h"
#include "scene_paragraph/commands/remove_scene_paragraph_command.h"
#include "scene_paragraph/scene_paragraph_dto.h"

#include "persistence/interface_scene_paragraph_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::SceneParagraph;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::SceneParagraph::Commands;

namespace Application::Features::SceneParagraph::Commands
{
class SKRIBISTO_APPLICATION_SCENE_PARAGRAPH_EXPORT RemoveSceneParagraphCommandHandler : public QObject
{
    Q_OBJECT
  public:
    RemoveSceneParagraphCommandHandler(QSharedPointer<InterfaceSceneParagraphRepository> repository);
    Result<int> handle(QPromise<Result<void>> &progressPromise, const RemoveSceneParagraphCommand &request);
    Result<int> restore();

  signals:
    void sceneParagraphCreated(Contracts::DTO::SceneParagraph::SceneParagraphDTO sceneParagraphDto);
    void sceneParagraphRemoved(int id);

  private:
    QSharedPointer<InterfaceSceneParagraphRepository> m_repository;
    Result<int> handleImpl(QPromise<Result<void>> &progressPromise, const RemoveSceneParagraphCommand &request);
    Result<int> restoreImpl();
    Domain::SceneParagraph m_oldState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::SceneParagraph::Commands
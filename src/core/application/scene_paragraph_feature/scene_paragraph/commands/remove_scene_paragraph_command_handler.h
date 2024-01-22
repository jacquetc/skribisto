// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_scene_paragraph_export.h"
#include "scene_paragraph/commands/remove_scene_paragraph_command.h"
#include "scene_paragraph/scene_paragraph_dto.h"

#include "repository/interface_scene_paragraph_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::SceneParagraph;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::SceneParagraph::Commands;

namespace Skribisto::Application::Features::SceneParagraph::Commands
{
class SKRIBISTO_APPLICATION_SCENE_PARAGRAPH_EXPORT RemoveSceneParagraphCommandHandler : public QObject
{
    Q_OBJECT
  public:
    RemoveSceneParagraphCommandHandler(InterfaceSceneParagraphRepository *repository);
    Result<int> handle(QPromise<Result<void>> &progressPromise, const RemoveSceneParagraphCommand &request);
    Result<int> restore();

  signals:
    // repositories handle remove signals
    // void sceneParagraphRemoved(int id);

  private:
    InterfaceSceneParagraphRepository *m_repository;
    Result<int> handleImpl(QPromise<Result<void>> &progressPromise, const RemoveSceneParagraphCommand &request);
    Result<int> restoreImpl();
    Skribisto::Domain::SceneParagraph m_oldState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::SceneParagraph::Commands
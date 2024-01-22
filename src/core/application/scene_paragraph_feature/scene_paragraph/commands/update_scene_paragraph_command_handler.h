// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_scene_paragraph_export.h"
#include "scene_paragraph/commands/update_scene_paragraph_command.h"
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
class SKRIBISTO_APPLICATION_SCENE_PARAGRAPH_EXPORT UpdateSceneParagraphCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateSceneParagraphCommandHandler(InterfaceSceneParagraphRepository *repository);
    Result<SceneParagraphDTO> handle(QPromise<Result<void>> &progressPromise,
                                     const UpdateSceneParagraphCommand &request);
    Result<SceneParagraphDTO> restore();

  signals:
    void sceneParagraphUpdated(Skribisto::Contracts::DTO::SceneParagraph::SceneParagraphDTO sceneParagraphDto);
    void sceneParagraphDetailsUpdated(int id);

  private:
    InterfaceSceneParagraphRepository *m_repository;
    Result<SceneParagraphDTO> handleImpl(QPromise<Result<void>> &progressPromise,
                                         const UpdateSceneParagraphCommand &request);
    Result<SceneParagraphDTO> restoreImpl();
    Result<SceneParagraphDTO> saveOldState();
    Result<SceneParagraphDTO> m_undoState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::SceneParagraph::Commands
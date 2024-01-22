// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_scene_paragraph_export.h"
#include "scene_paragraph/queries/get_scene_paragraph_query.h"
#include "scene_paragraph/scene_paragraph_dto.h"

#include "repository/interface_scene_paragraph_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::SceneParagraph;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::SceneParagraph::Queries;

namespace Skribisto::Application::Features::SceneParagraph::Queries
{
class SKRIBISTO_APPLICATION_SCENE_PARAGRAPH_EXPORT GetSceneParagraphQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetSceneParagraphQueryHandler(InterfaceSceneParagraphRepository *repository);
    Result<SceneParagraphDTO> handle(QPromise<Result<void>> &progressPromise, const GetSceneParagraphQuery &query);

  private:
    InterfaceSceneParagraphRepository *m_repository;
    Result<SceneParagraphDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetSceneParagraphQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::SceneParagraph::Queries
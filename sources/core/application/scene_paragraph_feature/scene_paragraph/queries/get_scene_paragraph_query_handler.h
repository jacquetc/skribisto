#pragma once

#include "application_scene_paragraph_export.h"
#include "scene_paragraph/queries/get_scene_paragraph_query.h"
#include "scene_paragraph/scene_paragraph_dto.h"

#include "persistence/interface_scene_paragraph_repository.h"
#include <QPromise>

using namespace Contracts::DTO::SceneParagraph;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::SceneParagraph::Queries;

namespace Application::Features::SceneParagraph::Queries
{
class SKRIBISTO_APPLICATION_SCENE_PARAGRAPH_EXPORT GetSceneParagraphQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetSceneParagraphQueryHandler(QSharedPointer<InterfaceSceneParagraphRepository> repository);
    Result<SceneParagraphDTO> handle(QPromise<Result<void>> &progressPromise, const GetSceneParagraphQuery &query);

  private:
    QSharedPointer<InterfaceSceneParagraphRepository> m_repository;
    Result<SceneParagraphDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetSceneParagraphQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::SceneParagraph::Queries
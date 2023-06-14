#pragma once

#include "application_scene_paragraph_export.h"
#include "scene_paragraph/scene_paragraph_dto.h"

#include "persistence/interface_scene_paragraph_repository.h"
#include <QPromise>

using namespace Contracts::DTO::SceneParagraph;
using namespace Contracts::Persistence;

namespace Application::Features::SceneParagraph::Queries
{
class SKRIBISTO_APPLICATION_SCENE_PARAGRAPH_EXPORT GetAllSceneParagraphQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllSceneParagraphQueryHandler(QSharedPointer<InterfaceSceneParagraphRepository> repository);
    Result<QList<SceneParagraphDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    QSharedPointer<InterfaceSceneParagraphRepository> m_repository;
    Result<QList<SceneParagraphDTO>> handleImpl(QPromise<Result<void>> &progressPromise);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::SceneParagraph::Queries
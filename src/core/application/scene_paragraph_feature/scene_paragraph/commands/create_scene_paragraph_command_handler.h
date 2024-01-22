// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_scene_paragraph_export.h"
#include "repository/interface_scene_paragraph_repository.h"
#include "scene_paragraph/commands/create_scene_paragraph_command.h"
#include "scene_paragraph/scene_paragraph_dto.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Skribisto::Domain;
using namespace Skribisto::Contracts::DTO::SceneParagraph;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::SceneParagraph::Commands;

namespace Skribisto::Application::Features::SceneParagraph::Commands
{
class SKRIBISTO_APPLICATION_SCENE_PARAGRAPH_EXPORT CreateSceneParagraphCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateSceneParagraphCommandHandler(InterfaceSceneParagraphRepository *repository);

    Result<SceneParagraphDTO> handle(QPromise<Result<void>> &progressPromise,
                                     const CreateSceneParagraphCommand &request);
    Result<SceneParagraphDTO> restore();

  signals:
    void sceneParagraphCreated(Skribisto::Contracts::DTO::SceneParagraph::SceneParagraphDTO sceneParagraphDto);
    void sceneParagraphRemoved(int id);

    void relationWithOwnerInserted(int id, int ownerId, int position);
    void relationWithOwnerRemoved(int id, int ownerId);

  private:
    InterfaceSceneParagraphRepository *m_repository;
    Result<SceneParagraphDTO> handleImpl(QPromise<Result<void>> &progressPromise,
                                         const CreateSceneParagraphCommand &request);
    Result<SceneParagraphDTO> restoreImpl();
    Result<Skribisto::Domain::SceneParagraph> m_newEntity;

    int m_ownerId = -1;
    int m_position = -1;

    QList<Skribisto::Domain::SceneParagraph> m_oldOwnerParagraphs;
    QList<Skribisto::Domain::SceneParagraph> m_ownerParagraphsNewState;

    static bool s_mappingRegistered;
    void registerMappings();
    bool m_firstPass = true;
};

} // namespace Skribisto::Application::Features::SceneParagraph::Commands
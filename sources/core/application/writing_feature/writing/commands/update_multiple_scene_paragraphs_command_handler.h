#pragma once

#include "application_writing_export.h"
#include "writing/commands/update_multiple_scene_paragraphs_command.h"
#include "writing/multiple_scene_paragraphs_changed_dto.h"

#include "persistence/interface_scene_paragraph_repository.h"
#include "persistence/interface_scene_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Writing;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Writing::Commands;

namespace Application::Features::Writing::Commands
{
class SKRIBISTO_APPLICATION_WRITING_EXPORT UpdateMultipleSceneParagraphsCommandHandler : public QObject
{
    Q_OBJECT
  public:
    UpdateMultipleSceneParagraphsCommandHandler(InterfaceSceneRepository *sceneRepository,
                                                InterfaceSceneParagraphRepository *sceneParagraphRepository);

    Result<MultipleSceneParagraphsChangedDTO> handle(QPromise<Result<void>> &progressPromise,
                                                     const UpdateMultipleSceneParagraphsCommand &request);

    Result<MultipleSceneParagraphsChangedDTO> restore();

  signals:
    void updateMultipleSceneParagraphsChanged(
        Contracts::DTO::Writing::MultipleSceneParagraphsChangedDTO multipleSceneParagraphsChangedDto);

  private:
    QSharedPointer<InterfaceSceneRepository> m_sceneRepository;
    QSharedPointer<InterfaceSceneParagraphRepository> m_sceneParagraphRepository;
    Result<MultipleSceneParagraphsChangedDTO> handleImpl(QPromise<Result<void>> &progressPromise,
                                                         const UpdateMultipleSceneParagraphsCommand &request);

    Result<MultipleSceneParagraphsChangedDTO> restoreImpl();
    Result<MultipleSceneParagraphsChangedDTO> m_newState;

    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Writing::Commands

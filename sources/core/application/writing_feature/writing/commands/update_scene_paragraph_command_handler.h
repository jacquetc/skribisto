#pragma once

#include "application_writing_export.h"
#include "writing/commands/update_scene_paragraph_command.h"
#include "writing/scene_paragraph_changed_dto.h"

#include "persistence/interface_scene_paragraph_repository.h"
#include "persistence/interface_scene_repository.h"
#include "result.h"
#include <QPromise>
#include <QScopedPointer>

using namespace Contracts::DTO::Writing;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Writing::Commands;

namespace Application::Features::Writing::Commands
{
class SKRIBISTO_APPLICATION_WRITING_EXPORT UpdateSceneParagraphCommandHandler : public QObject
{
    Q_OBJECT
  public:
    UpdateSceneParagraphCommandHandler(QSharedPointer<InterfaceSceneRepository> sceneRepository,
                                       QSharedPointer<InterfaceSceneParagraphRepository> sceneParagraphRepository);

    Result<SceneParagraphChangedDTO> handle(QPromise<Result<void>> &progressPromise,
                                            const UpdateSceneParagraphCommand &request);

    Result<SceneParagraphChangedDTO> restore();

  signals:
    void sceneParagraphChanged(Contracts::DTO::Writing::SceneParagraphChangedDTO sceneParagraphChangedDto);

  private:
    QSharedPointer<InterfaceSceneRepository> m_sceneRepository;
    QSharedPointer<InterfaceSceneParagraphRepository> m_sceneParagraphRepository;
    Result<SceneParagraphChangedDTO> handleImpl(QPromise<Result<void>> &progressPromise,
                                                const UpdateSceneParagraphCommand &request);

    Result<SceneParagraphChangedDTO> restoreImpl();

    static bool s_mappingRegistered;
    void registerMappings();

    struct OriginalState
    {
        int paragraphId = 0;
        QUuid paragraphUuid;
        int sceneId;
        QUuid sceneUuid;
        int paragraphIndex;
        int oldCursorPosition;
        QString text;
        bool paragraphExists;
    };

    struct NewState
    {
        int paragraphId = 0;
        QUuid paragraphUuid;
        int sceneId;
        QUuid sceneUuid;
        int paragraphIndex;
        int newCursorPosition;
        QString text;
    };
    QScopedPointer<OriginalState> m_originalState;
    QScopedPointer<NewState> m_newState;
};

} // namespace Application::Features::Writing::Commands

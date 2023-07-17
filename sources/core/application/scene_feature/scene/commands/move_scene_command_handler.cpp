#include "move_scene_command_handler.h"
#include "automapper/automapper.h"
#include "scene/move_scene_dto.h"
#include "scene/validators/move_scene_command_validator.h"
#include <QDebug>

using namespace Contracts::DTO::Scene;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Scene::Validators;
using namespace Application::Features::Scene::Commands;

MoveSceneCommandHandler::MoveSceneCommandHandler(QSharedPointer<InterfaceChapterRepository> chapterRepository,
                                                 QSharedPointer<InterfaceSceneRepository> sceneRepository)
    : m_chapterRepository(chapterRepository), m_sceneRepository(sceneRepository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<MoveSceneReplyDTO> MoveSceneCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                          const MoveSceneCommand &request)
{
    Result<MoveSceneReplyDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<MoveSceneReplyDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling MoveSceneCommand:" << ex.what();
    }
    return result;
}

Result<MoveSceneReplyDTO> MoveSceneCommandHandler::restore()
{

    Result<MoveSceneReplyDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<MoveSceneReplyDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling MoveSceneCommand restore:" << ex.what();
    }
    return result;
}

Result<MoveSceneReplyDTO> MoveSceneCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                              const MoveSceneCommand &request)
{
    qDebug() << "MoveSceneCommandHandler::handleImpl called";

    Domain::Scene scene;

    if (m_newState.isEmpty())
    {

        // Validate the create scene command using the validator
        auto validator = MoveSceneCommandValidator(m_chapterRepository, m_sceneRepository);
        Result<void> validatorResult = validator.validate(request.req);

        if (Q_UNLIKELY(validatorResult.hasError()))
        {
            return Result<MoveSceneReplyDTO>(validatorResult.error());
        }

        // implement logic here which will not be repeated on restore
        scene = AutoMapper::AutoMapper::map<MoveSceneDTO, Domain::Scene>(request.req);
    }
    else
    {
        // implement logic here to load already filled newState for restore
        scene = AutoMapper::AutoMapper::map<MoveSceneDTO, Domain::Scene>(request.req);
    }

    m_chapterRepository->beginChanges();

    // play here with the repositories
    Q_UNIMPLEMENTED();

    m_chapterRepository->saveChanges();

    // dummy to compile:
    auto sceneResult = Result<Domain::Scene>(scene);

    // implement logic here to save to new state for restore
    auto moveSceneReplyDTO = AutoMapper::AutoMapper::map<Domain::Scene, MoveSceneReplyDTO>(sceneResult.value());

    m_newState = Result<MoveSceneReplyDTO>(moveSceneReplyDTO);

    // emit signal
    emit moveSceneChanged(moveSceneReplyDTO);

    // Return
    return Result<MoveSceneReplyDTO>(moveSceneReplyDTO);
}

Result<MoveSceneReplyDTO> MoveSceneCommandHandler::restoreImpl()
{

    MoveSceneReplyDTO moveSceneReplyDTO;
    // auto moveSceneReplyDTO = AutoMapper::AutoMapper::map<Domain::Scene, MoveSceneReplyDTO>(m_newState);

    emit moveSceneChanged(moveSceneReplyDTO);

    return Result<MoveSceneReplyDTO>(moveSceneReplyDTO);
}

bool MoveSceneCommandHandler::s_mappingRegistered = false;

void MoveSceneCommandHandler::registerMappings()
{
}
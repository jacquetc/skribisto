#include "update_scene_paragraph_command_handler.h"
#include "automapper/automapper.h"
#include "writing/update_scene_paragraph_dto.h"
#include "writing/validators/update_scene_paragraph_command_validator.h"
#include <QDebug>

using namespace Contracts::DTO::Writing;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Writing::Validators;
using namespace Application::Features::Writing::Commands;

UpdateSceneParagraphCommandHandler::UpdateSceneParagraphCommandHandler(
    QSharedPointer<InterfaceSceneRepository> sceneRepository,
    QSharedPointer<InterfaceSceneParagraphRepository> sceneParagraphRepository)
    : m_sceneRepository(sceneRepository), m_sceneParagraphRepository(sceneParagraphRepository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<SceneParagraphChangedDTO> UpdateSceneParagraphCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                            const UpdateSceneParagraphCommand &request)
{
    Result<SceneParagraphChangedDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneParagraphChangedDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateSceneParagraphCommand:" << ex.what();
    }
    return result;
}

Result<SceneParagraphChangedDTO> UpdateSceneParagraphCommandHandler::restore()
{

    Result<SceneParagraphChangedDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneParagraphChangedDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateSceneParagraphCommand restore:" << ex.what();
    }
    return result;
}

Result<SceneParagraphChangedDTO> UpdateSceneParagraphCommandHandler::handleImpl(
    QPromise<Result<void>> &progressPromise, const UpdateSceneParagraphCommand &request)
{
    qDebug() << "UpdateSceneParagraphCommandHandler::handleImpl called";

    // Domain::Writing writing;

    if (m_newState.isEmpty())
    {

        // Validate the create writing command using the validator
        auto validator = UpdateSceneParagraphCommandValidator(m_sceneRepository, m_sceneParagraphRepository);
        Result<void> validatorResult = validator.validate(request.req);

        if (Q_UNLIKELY(validatorResult.hasError()))
        {
            return Result<SceneParagraphChangedDTO>(validatorResult.error());
        }

        // implement logic here which will not be repeated on restore
        // writing = AutoMapper::AutoMapper::map<UpdateSceneParagraphDTO, Domain::Writing>(request.req);
    }
    else
    {
        // implement logic here to load already filled newState for restore
        // writing = AutoMapper::AutoMapper::map<UpdateSceneParagraphDTO, Domain::Writing>(request.req);
    }

    m_sceneRepository->beginChanges();

    // play here with the repositories
    Q_UNIMPLEMENTED();

    m_sceneRepository->saveChanges();

    // dummy to compile:
    // auto writingResult = Result<Domain::Writing>(writing);

    // implement logic here to save to new state for restore
    //    auto sceneParagraphChangedDTO =
    //        AutoMapper::AutoMapper::map<Domain::Writing, SceneParagraphChangedDTO>(writingResult.value());
    SceneParagraphChangedDTO sceneParagraphChangedDTO;
    m_newState = Result<SceneParagraphChangedDTO>(sceneParagraphChangedDTO);

    // emit signal
    emit updateSceneParagraphChanged(sceneParagraphChangedDTO);

    // Return
    return Result<SceneParagraphChangedDTO>(sceneParagraphChangedDTO);
}

Result<SceneParagraphChangedDTO> UpdateSceneParagraphCommandHandler::restoreImpl()
{

    SceneParagraphChangedDTO sceneParagraphChangedDTO;
    // auto sceneParagraphChangedDTO = AutoMapper::AutoMapper::map<Domain::Writing,
    // SceneParagraphChangedDTO>(m_newState);
    Q_UNIMPLEMENTED();

    emit updateSceneParagraphChanged(sceneParagraphChangedDTO);

    return Result<SceneParagraphChangedDTO>(sceneParagraphChangedDTO);
}

bool UpdateSceneParagraphCommandHandler::s_mappingRegistered = false;

void UpdateSceneParagraphCommandHandler::registerMappings()
{
}

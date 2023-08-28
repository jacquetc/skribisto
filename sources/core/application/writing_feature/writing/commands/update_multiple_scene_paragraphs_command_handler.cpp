#include "update_multiple_scene_paragraphs_command_handler.h"
#include "automapper/automapper.h"
#include "writing/update_multiple_scene_paragraphs_dto.h"
#include "writing/validators/update_multiple_scene_paragraphs_command_validator.h"
#include <QDebug>

using namespace Contracts::DTO::Writing;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Writing::Validators;
using namespace Application::Features::Writing::Commands;

UpdateMultipleSceneParagraphsCommandHandler::UpdateMultipleSceneParagraphsCommandHandler(
    InterfaceSceneRepository *sceneRepository, InterfaceSceneParagraphRepository *sceneParagraphRepository)
    : m_sceneRepository(sceneRepository), m_sceneParagraphRepository(sceneParagraphRepository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<MultipleSceneParagraphsChangedDTO> UpdateMultipleSceneParagraphsCommandHandler::handle(
    QPromise<Result<void>> &progressPromise, const UpdateMultipleSceneParagraphsCommand &request)
{
    Result<MultipleSceneParagraphsChangedDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result =
            Result<MultipleSceneParagraphsChangedDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateMultipleSceneParagraphsCommand:" << ex.what();
    }
    return result;
}

Result<MultipleSceneParagraphsChangedDTO> UpdateMultipleSceneParagraphsCommandHandler::restore()
{

    Result<MultipleSceneParagraphsChangedDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result =
            Result<MultipleSceneParagraphsChangedDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateMultipleSceneParagraphsCommand restore:" << ex.what();
    }
    return result;
}

Result<MultipleSceneParagraphsChangedDTO> UpdateMultipleSceneParagraphsCommandHandler::handleImpl(
    QPromise<Result<void>> &progressPromise, const UpdateMultipleSceneParagraphsCommand &request)
{
    qDebug() << "UpdateMultipleSceneParagraphsCommandHandler::handleImpl called";

    // Domain::Writing writing;

    if (m_newState.isEmpty())
    {

        // Validate the create writing command using the validator
        auto validator = UpdateMultipleSceneParagraphsCommandValidator(m_sceneRepository, m_sceneParagraphRepository);
        Result<void> validatorResult = validator.validate(request.req);

        if (Q_UNLIKELY(validatorResult.hasError()))
        {
            return Result<MultipleSceneParagraphsChangedDTO>(validatorResult.error());
        }

        // implement logic here which will not be repeated on restore
        // writing = AutoMapper::AutoMapper::map<UpdateMultipleSceneParagraphsDTO, Domain::Writing>(request.req);
    }
    else
    {
        // implement logic here to load already filled newState for restore
        // writing = AutoMapper::AutoMapper::map<UpdateMultipleSceneParagraphsDTO, Domain::Writing>(request.req);
    }

    m_sceneRepository->beginChanges();

    // play here with the repositories
    Q_UNIMPLEMENTED();

    m_sceneRepository->saveChanges();

    // dummy to compile:
    //    auto writingResult = Result<Domain::Writing>(writing);

    // implement logic here to save to new state for restore
    //    auto multipleSceneParagraphsChangedDTO =
    //        AutoMapper::AutoMapper::map<Domain::Writing, MultipleSceneParagraphsChangedDTO>(writingResult.value());
    MultipleSceneParagraphsChangedDTO multipleSceneParagraphsChangedDTO;
    m_newState = Result<MultipleSceneParagraphsChangedDTO>(multipleSceneParagraphsChangedDTO);

    // emit signal
    emit updateMultipleSceneParagraphsChanged(multipleSceneParagraphsChangedDTO);

    // Return
    return Result<MultipleSceneParagraphsChangedDTO>(multipleSceneParagraphsChangedDTO);
}

Result<MultipleSceneParagraphsChangedDTO> UpdateMultipleSceneParagraphsCommandHandler::restoreImpl()
{

    MultipleSceneParagraphsChangedDTO multipleSceneParagraphsChangedDTO;
    // auto multipleSceneParagraphsChangedDTO = AutoMapper::AutoMapper::map<Domain::Writing,
    // MultipleSceneParagraphsChangedDTO>(m_newState);
    Q_UNIMPLEMENTED();

    emit updateMultipleSceneParagraphsChanged(multipleSceneParagraphsChangedDTO);

    return Result<MultipleSceneParagraphsChangedDTO>(multipleSceneParagraphsChangedDTO);
}

bool UpdateMultipleSceneParagraphsCommandHandler::s_mappingRegistered = false;

void UpdateMultipleSceneParagraphsCommandHandler::registerMappings()
{
}

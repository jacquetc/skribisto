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

    if (m_originalState.isNull())
    {

        // Validate the create writing command using the validator
        auto validator = UpdateSceneParagraphCommandValidator(m_sceneRepository, m_sceneParagraphRepository);
        Result<void> validatorResult = validator.validate(request.req);

        if (Q_UNLIKELY(validatorResult.hasError()))
        {
            return Result<SceneParagraphChangedDTO>(validatorResult.error());
        }

        Result<Domain::SceneParagraph> sceneParagraphResult;

        // means no paragraph id was passed in, so we need to create a new SceneParagraph
        if (request.req.paragraphId() == -1)
        {

            bool sceneParagraphExists = m_sceneParagraphRepository->exists(request.req.paragraphUuid());
            m_originalState.reset(new OriginalState(
                {request.req.paragraphId(), request.req.paragraphUuid(), request.req.sceneId(), request.req.sceneUuid(),
                 request.req.paragraphIndexInScene(), request.req.oldCursorPosition(), "", sceneParagraphExists}));
        }
        else
        {
            sceneParagraphResult = m_sceneParagraphRepository->get(request.req.paragraphId());
            if (sceneParagraphResult.hasError())
            {
                return Result<SceneParagraphChangedDTO>(sceneParagraphResult.error());
            }

            m_originalState.reset(
                new OriginalState({request.req.paragraphId(), request.req.paragraphUuid(), request.req.sceneId(),
                                   request.req.sceneUuid(), request.req.paragraphIndexInScene(),
                                   request.req.oldCursorPosition(), sceneParagraphResult.value().content(), true}));
        }
    }
    else
    {
        // implement logic here to load already filled newState for restore
        // writing = AutoMapper::AutoMapper::map<UpdateSceneParagraphDTO, Domain::Writing>(request.req);
    }

    Result<Domain::SceneParagraph> sceneParagraphResult;

    //  paragraph id not known at the time
    if (request.req.paragraphId() == -1)
    {

        bool sceneParagraphExists = m_sceneParagraphRepository->exists(request.req.paragraphUuid());
        if (sceneParagraphExists)
        {
        }
        else
        // create a new paragraph and add it to the scene

        {
            // impose the same id if we are redoing
            int paragraphId = 0; // no id
            if (m_newState.isNull() == false)
            {
                paragraphId = m_newState->paragraphId;
            }

            Domain::SceneParagraph newParagraph(paragraphId, request.req.paragraphUuid(), QDateTime::currentDateTime(),
                                                QDateTime::currentDateTime(), request.req.text());
            sceneParagraphResult = m_sceneParagraphRepository->add(std::move(newParagraph));
            if (sceneParagraphResult.hasError())
            {
                return Result<SceneParagraphChangedDTO>(sceneParagraphResult.error());
            }
        }
    }
    else
    {

        sceneParagraphResult = m_sceneParagraphRepository->get(request.req.paragraphId());
        if (sceneParagraphResult.hasError())
        {
            return Result<SceneParagraphChangedDTO>(sceneParagraphResult.error());
        }
    }

    m_sceneRepository->beginChanges();

    // play here with the repositories

    Domain::SceneParagraph paragraph;

    m_sceneParagraphRepository
        ->add()

            m_sceneRepository->saveChanges();

    // dummy to compile:
    // auto writingResult = Result<Domain::Writing>(writing);

    // implement logic here to save to new state for restore
    //    auto sceneParagraphChangedDTO =
    //        AutoMapper::AutoMapper::map<Domain::Writing, SceneParagraphChangedDTO>(writingResult.value());
    SceneParagraphChangedDTO sceneParagraphChangedDTO;

    if (m_originalState.isNull())
    {

        m_newState.reset({request.req.paragraphId(), request.req.paragraphUuid(), request.req.sceneUuid(),
                          request.req.paragraphIndexInScene, request.req.text(), request.req.newCursorPosition()});
    }

    // emit signal
    emit sceneParagraphChanged(sceneParagraphChangedDTO);

    // Return
    return Result<SceneParagraphChangedDTO>(sceneParagraphChangedDTO);
}

Result<SceneParagraphChangedDTO> UpdateSceneParagraphCommandHandler::restoreImpl()
{

    SceneParagraphChangedDTO sceneParagraphChangedDTO;
    // auto sceneParagraphChangedDTO = AutoMapper::AutoMapper::map<Domain::Writing,
    // SceneParagraphChangedDTO>(m_newState);
    Q_UNIMPLEMENTED();

    emit sceneParagraphChanged(sceneParagraphChangedDTO);

    return Result<SceneParagraphChangedDTO>(sceneParagraphChangedDTO);
}

bool UpdateSceneParagraphCommandHandler::s_mappingRegistered = false;

void UpdateSceneParagraphCommandHandler::registerMappings()
{
}

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

            // warn in the unlikely case there is a scene paragraph with this uuid
            if (sceneParagraphExists)
            {
                qDebug() << "Warning: scene paragraph with uuid" << request.req.paragraphUuid()
                         << "already exists, but id was not passed in.";
            }
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
        // implement logic here to load already filled newState for redo
        // writing = AutoMapper::AutoMapper::map<UpdateSceneParagraphDTO, Domain::Writing>(request.req);
    }

    Result<Domain::SceneParagraph> sceneParagraphResult;

    //  paragraph id not known at the time
    if (m_originalState->paragraphId == -1)
    {

        if (m_originalState->paragraphExists)
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

            // add the new paragraph to the scene
            auto sceneResult = m_sceneRepository->get(m_originalState->sceneId);
            if (sceneResult.hasError())
            {
                return Result<SceneParagraphChangedDTO>(sceneResult.error());
            }

            Domain::Scene scene(sceneResult.value());
            QList<Domain::SceneParagraph> paragraphs = scene.paragraphs();
            // insert at index:

            paragraphs.insert(m_originalState->paragraphIndex, sceneParagraphResult.value());
            scene.setParagraphs(paragraphs);
            scene.setUpdateDate(QDateTime::currentDateTime());
            m_sceneRepository->update(std::move(scene));
        }
    }
    else // paragraph id known at the time
    {

        sceneParagraphResult = m_sceneParagraphRepository->get(request.req.paragraphId());
        if (sceneParagraphResult.hasError())
        {
            return Result<SceneParagraphChangedDTO>(sceneParagraphResult.error());
        }
    }

    m_sceneRepository->beginChanges();

    // play here with the repositories

    Domain::SceneParagraph paragraph(sceneParagraphResult.value());
    paragraph.setContent(request.req.text());
    paragraph.setUpdateDate(QDateTime::currentDateTime());

    m_sceneParagraphRepository->update(std::move(paragraph));

    m_sceneRepository->saveChanges();

    // dummy to compile:
    // auto writingResult = Result<Domain::Writing>(writing);

    // implement logic here to save to new state for restore
    //    auto sceneParagraphChangedDTO =
    //        AutoMapper::AutoMapper::map<Domain::Writing, SceneParagraphChangedDTO>(writingResult.value());

    if (m_newState.isNull())
    {

        m_newState.reset(new NewState({request.req.paragraphId(), request.req.paragraphUuid(), request.req.sceneId(),
                                       request.req.sceneUuid(), request.req.paragraphIndexInScene(),
                                       request.req.newCursorPosition(), request.req.text()}));
    }

    SceneParagraphChangedDTO sceneParagraphChangedDTO;
    sceneParagraphChangedDTO.setParagraphId(m_newState->paragraphId);
    sceneParagraphChangedDTO.setParagraphUuid(m_newState->paragraphUuid);
    sceneParagraphChangedDTO.setSceneId(m_newState->sceneId);
    sceneParagraphChangedDTO.setSceneUuid(m_newState->sceneUuid);
    sceneParagraphChangedDTO.setParagraphIndexInScene(m_newState->paragraphIndex);
    sceneParagraphChangedDTO.setNewCursorPosition(m_newState->newCursorPosition);
    sceneParagraphChangedDTO.setText(m_newState->text);

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

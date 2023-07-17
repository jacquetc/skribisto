#include "get_scene_paragraph_id_query_handler.h"
#include "automapper/automapper.h"
#include "writing/get_scene_paragraph_id_dto.h"
#include "writing/validators/get_scene_paragraph_id_query_validator.h"
#include <QDebug>

using namespace Contracts::DTO::Writing;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Writing::Validators;
using namespace Application::Features::Writing::Queries;

GetSceneParagraphIdQueryHandler::GetSceneParagraphIdQueryHandler(
    QSharedPointer<InterfaceSceneParagraphRepository> sceneParagraphRepository)
    : m_sceneParagraphRepository(sceneParagraphRepository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<GetSceneParagraphIdReplyDTO> GetSceneParagraphIdQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                            const GetSceneParagraphIdQuery &request)
{
    Result<GetSceneParagraphIdReplyDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<GetSceneParagraphIdReplyDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetSceneParagraphIdQuery:" << ex.what();
    }
    return result;
}

Result<GetSceneParagraphIdReplyDTO> GetSceneParagraphIdQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                                const GetSceneParagraphIdQuery &request)
{
    qDebug() << "GetSceneParagraphIdQueryHandler::handleImpl called";

    //    Domain::Writing writing;

    // Validate the query using the validator
    auto validator = GetSceneParagraphIdQueryValidator(m_sceneParagraphRepository);
    Result<void> validatorResult = validator.validate(request.req);

    if (Q_UNLIKELY(validatorResult.hasError()))
    {
        return Result<GetSceneParagraphIdReplyDTO>(validatorResult.error());
    }

    // implement logic here
    //    writing = AutoMapper::AutoMapper::map<GetSceneParagraphIdReplyDTO, Domain::Writing>(request.req);

    // play here with the repositories
    Q_UNIMPLEMENTED();

    //    auto writingDTO =
    //        AutoMapper::AutoMapper::map<Domain::Writing, GetSceneParagraphIdReplyDTO>(writingResult.value());
    GetSceneParagraphIdReplyDTO getSceneParagraphIdReplyDTO;
    // emit signal
    emit getSceneParagraphIdChanged(getSceneParagraphIdReplyDTO);

    // Return
    return Result<GetSceneParagraphIdReplyDTO>(getSceneParagraphIdReplyDTO);
}

bool GetSceneParagraphIdQueryHandler::s_mappingRegistered = false;

void GetSceneParagraphIdQueryHandler::registerMappings()
{
}

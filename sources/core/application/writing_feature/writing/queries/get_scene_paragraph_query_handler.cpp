#include "get_scene_paragraph_query_handler.h"
#include "automapper/automapper.h"
#include "writing/get_scene_paragraph_dto.h"
#include "writing/validators/get_scene_paragraph_query_validator.h"
#include <QDebug>

using namespace Contracts::DTO::Writing;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Writing::Validators;
using namespace Application::Features::Writing::Queries;

GetSceneParagraphQueryHandler::GetSceneParagraphQueryHandler(
    QSharedPointer<InterfaceSceneParagraphRepository> sceneParagraphRepository)
    : m_sceneParagraphRepository(sceneParagraphRepository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<GetSceneParagraphReplyDTO> GetSceneParagraphQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                        const GetSceneParagraphQuery &request)
{
    Result<GetSceneParagraphReplyDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<GetSceneParagraphReplyDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetSceneParagraphQuery:" << ex.what();
    }
    return result;
}

Result<GetSceneParagraphReplyDTO> GetSceneParagraphQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                            const GetSceneParagraphQuery &request)
{
    qDebug() << "GetSceneParagraphQueryHandler::handleImpl called";

    //    Domain::Writing writing;

    // Validate the query using the validator
    auto validator = GetSceneParagraphQueryValidator(m_sceneParagraphRepository);
    Result<void> validatorResult = validator.validate(request.req);

    if (Q_UNLIKELY(validatorResult.hasError()))
    {
        return Result<GetSceneParagraphReplyDTO>(validatorResult.error());
    }

    // implement logic here
    //    writing = AutoMapper::AutoMapper::map<GetSceneParagraphReplyDTO, Domain::Writing>(request.req);

    // play here with the repositories
    Q_UNIMPLEMENTED();

    //    auto writingDTO =
    //        AutoMapper::AutoMapper::map<Domain::Writing, GetSceneParagraphReplyDTO>(writingResult.value());
    GetSceneParagraphReplyDTO getSceneParagraphReplyDTO;
    // emit signal
    emit getSceneParagraphChanged(getSceneParagraphReplyDTO);

    // Return
    return Result<GetSceneParagraphReplyDTO>(getSceneParagraphReplyDTO);
}

bool GetSceneParagraphQueryHandler::s_mappingRegistered = false;

void GetSceneParagraphQueryHandler::registerMappings()
{
}

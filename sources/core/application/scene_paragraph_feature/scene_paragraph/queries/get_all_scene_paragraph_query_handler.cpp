#include "get_all_scene_paragraph_query_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_scene_paragraph_repository.h"

using namespace Application::Features::SceneParagraph::Queries;

GetAllSceneParagraphQueryHandler::GetAllSceneParagraphQueryHandler(
    QSharedPointer<InterfaceSceneParagraphRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<QList<SceneParagraphDTO>> GetAllSceneParagraphQueryHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllSceneParagraphQueryHandler::handle called";

    Result<QList<SceneParagraphDTO>> result;

    try
    {
        result = handleImpl(progressPromise);
    }
    catch (const std::exception &ex)
    {
        result = Result<QList<SceneParagraphDTO>>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllSceneParagraphQuery:" << ex.what();
    }
    return result;
}

Result<QList<SceneParagraphDTO>> GetAllSceneParagraphQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllSceneParagraphQueryHandler::handleImpl called";

    // do
    auto sceneParagraphResult = m_repository->getAll();

    if (Q_UNLIKELY(!sceneParagraphResult.isError()))
    {
        return Result<QList<SceneParagraphDTO>>(sceneParagraphResult.error());
    }

    // map
    QList<SceneParagraphDTO> dtoList;

    for (const Domain::SceneParagraph &sceneParagraph : sceneParagraphResult.value())
    {
        auto dto = AutoMapper::AutoMapper::map<Domain::SceneParagraph, SceneParagraphDTO>(sceneParagraph);
        dtoList.append(dto);
    }

    qDebug() << "GetAllSceneParagraphQueryHandler::handleImpl done";

    return Result<QList<SceneParagraphDTO>>(dtoList);
}

bool GetAllSceneParagraphQueryHandler::s_mappingRegistered = false;

void GetAllSceneParagraphQueryHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::SceneParagraph, Contracts::DTO::SceneParagraph::SceneParagraphDTO>(
        true);
}
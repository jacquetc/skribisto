// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_atelier_query_handler.h"
#include "repository/interface_atelier_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::Atelier::Queries;

GetAtelierQueryHandler::GetAtelierQueryHandler(InterfaceAtelierRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<AtelierDTO> GetAtelierQueryHandler::handle(QPromise<Result<void>> &progressPromise, const GetAtelierQuery &query)
{
    Result<AtelierDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<AtelierDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAtelierQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<AtelierDTO> GetAtelierQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                      const GetAtelierQuery &query)
{
    qDebug() << "GetAtelierQueryHandler::handleImpl called with id" << query.id;

    // do
    auto atelierResult = m_repository->get(query.id);

    QLN_RETURN_IF_ERROR(AtelierDTO, atelierResult)

    // map
    auto dto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Atelier, AtelierDTO>(atelierResult.value());

    qDebug() << "GetAtelierQueryHandler::handleImpl done";

    return Result<AtelierDTO>(dto);
}

bool GetAtelierQueryHandler::s_mappingRegistered = false;

void GetAtelierQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Atelier,
                                                           Contracts::DTO::Atelier::AtelierDTO>(true, true);
}
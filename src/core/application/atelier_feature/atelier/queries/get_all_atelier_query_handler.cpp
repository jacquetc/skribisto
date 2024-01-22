// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_all_atelier_query_handler.h"
#include "repository/interface_atelier_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::Atelier::Queries;

GetAllAtelierQueryHandler::GetAllAtelierQueryHandler(InterfaceAtelierRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<QList<AtelierDTO>> GetAllAtelierQueryHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllAtelierQueryHandler::handle called";

    Result<QList<AtelierDTO>> result;

    try
    {
        result = handleImpl(progressPromise);
    }
    catch (const std::exception &ex)
    {
        result = Result<QList<AtelierDTO>>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllAtelierQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<QList<AtelierDTO>> GetAllAtelierQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllAtelierQueryHandler::handleImpl called";

    // do
    auto atelierResult = m_repository->getAll();

    QLN_RETURN_IF_ERROR(QList<AtelierDTO>, atelierResult)

    // map
    QList<AtelierDTO> dtoList;

    for (const Skribisto::Domain::Atelier &atelier : atelierResult.value())
    {
        auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Atelier, AtelierDTO>(atelier);
        dtoList.append(dto);
    }

    qDebug() << "GetAllAtelierQueryHandler::handleImpl done";

    return Result<QList<AtelierDTO>>(dtoList);
}

bool GetAllAtelierQueryHandler::s_mappingRegistered = false;

void GetAllAtelierQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Atelier,
                                                           Contracts::DTO::Atelier::AtelierDTO>(true, true);
}
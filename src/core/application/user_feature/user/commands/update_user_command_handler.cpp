// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_user_command_handler.h"
#include "repository/interface_user_repository.h"
#include "user/validators/update_user_command_validator.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::User;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::User::Commands;
using namespace Skribisto::Contracts::CQRS::User::Validators;
using namespace Skribisto::Application::Features::User::Commands;

UpdateUserCommandHandler::UpdateUserCommandHandler(InterfaceUserRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<UserDTO> UpdateUserCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                 const UpdateUserCommand &request)
{
    Result<UserDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<UserDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateUserCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<UserDTO> UpdateUserCommandHandler::restore()
{
    Result<UserDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<UserDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateUserCommand restore:" << ex.what();
    }
    return result;
}

Result<UserDTO> UpdateUserCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                     const UpdateUserCommand &request)
{
    qDebug() << "UpdateUserCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateUserCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    QLN_RETURN_IF_ERROR(UserDTO, validatorResult)

    // save old state
    if (m_undoState.isEmpty())
    {
        Result<Skribisto::Domain::User> currentResult = m_repository->get(request.req.id());

        QLN_RETURN_IF_ERROR(UserDTO, currentResult)

        // map
        m_undoState = Result<UserDTO>(
            Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::User, UserDTO>(currentResult.value()));
    }
    auto updateDto = Qleany::Tools::AutoMapper::AutoMapper::map<UserDTO, UpdateUserDTO>(m_undoState.value());
    updateDto << request.req;

    // map
    auto user = Qleany::Tools::AutoMapper::AutoMapper::map<UpdateUserDTO, Skribisto::Domain::User>(updateDto);

    // set update timestamp only on first pass
    if (m_undoState.isEmpty())
    {
        user.setUpdateDate(QDateTime::currentDateTime());
    }

    // do
    auto userResult = m_repository->update(std::move(user));

    if (userResult.hasError())
    {
        return Result<UserDTO>(userResult.error());
    }

    // map
    auto userDto = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::User, UserDTO>(userResult.value());

    emit userUpdated(userDto);

    if (request.req.metaData().areDetailsSet())
    {
        emit userDetailsUpdated(userDto.id());
    }

    qDebug() << "UpdateUserCommandHandler::handleImpl done";

    return Result<UserDTO>(userDto);
}

Result<UserDTO> UpdateUserCommandHandler::restoreImpl()
{
    qDebug() << "UpdateUserCommandHandler::restoreImpl called with id" << m_undoState.value().uuid();

    // map
    auto user = Qleany::Tools::AutoMapper::AutoMapper::map<UserDTO, Skribisto::Domain::User>(m_undoState.value());

    // do
    auto userResult = m_repository->update(std::move(user));

    QLN_RETURN_IF_ERROR(UserDTO, userResult)

    // map
    auto userDto = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::User, UserDTO>(userResult.value());

    emit userUpdated(userDto);

    qDebug() << "UpdateUserCommandHandler::restoreImpl done";

    return Result<UserDTO>(userDto);
}

bool UpdateUserCommandHandler::s_mappingRegistered = false;

void UpdateUserCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::User, Contracts::DTO::User::UserDTO>(
        true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::User::UpdateUserDTO,
                                                           Contracts::DTO::User::UserDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::User::UpdateUserDTO,
                                                           Skribisto::Domain::User>();
}
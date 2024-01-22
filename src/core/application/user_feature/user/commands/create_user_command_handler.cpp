// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "create_user_command_handler.h"
#include "user/validators/create_user_command_validator.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Domain;
using namespace Skribisto::Contracts::DTO::User;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::User::Validators;
using namespace Skribisto::Application::Features::User::Commands;

CreateUserCommandHandler::CreateUserCommandHandler(InterfaceUserRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<UserDTO> CreateUserCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                 const CreateUserCommand &request)
{
    Result<UserDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<UserDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateUserCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<UserDTO> CreateUserCommandHandler::restore()
{
    Result<UserDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<UserDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateUserCommand restore:" << ex.what();
    }
    return result;
}

Result<UserDTO> CreateUserCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                     const CreateUserCommand &request)
{
    qDebug() << "CreateUserCommandHandler::handleImpl called";
    Skribisto::Domain::User user;
    CreateUserDTO createDTO = request.req;

    if (m_firstPass)
    {
        // Validate the create User command using the validator
        auto validator = CreateUserCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(createDTO);

        QLN_RETURN_IF_ERROR(UserDTO, validatorResult);

        // Map the create User command to a domain User object and
        // generate a UUID
        user = Qleany::Tools::AutoMapper::AutoMapper::map<CreateUserDTO, Skribisto::Domain::User>(createDTO);

        // allow for forcing the uuid
        if (user.uuid().isNull())
        {
            user.setUuid(QUuid::createUuid());
        }

        // Set the creation and update timestamps to the current date and time
        user.setCreationDate(QDateTime::currentDateTime());
        user.setUpdateDate(QDateTime::currentDateTime());
    }
    else
    {
        user = m_newEntity.value();
    }

    // Add the user to the repository

    m_repository->beginChanges();
    auto userResult = m_repository->add(std::move(user));

    QLN_RETURN_IF_ERROR_WITH_ACTION(UserDTO, userResult, m_repository->cancelChanges();)

    // Get the newly created User entity
    user = userResult.value();
    // Save the newly created entity
    m_newEntity = userResult;

    //  Manage relation to owner

    m_repository->saveChanges();

    m_newEntity = userResult;

    auto userDTO = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::User, UserDTO>(userResult.value());
    emit userCreated(userDTO);

    qDebug() << "User added:" << userDTO.id();

    m_firstPass = false;

    // Return the DTO of the newly created User as a Result object
    return Result<UserDTO>(userDTO);
}

Result<UserDTO> CreateUserCommandHandler::restoreImpl()
{
    int entityId = m_newEntity.value().id();
    auto deleteResult = m_repository->remove(entityId);

    QLN_RETURN_IF_ERROR(UserDTO, deleteResult)

    emit userRemoved(deleteResult.value());

    qDebug() << "User removed:" << deleteResult.value();

    return Result<UserDTO>(UserDTO());
}

bool CreateUserCommandHandler::s_mappingRegistered = false;

void CreateUserCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::User, Contracts::DTO::User::UserDTO>(
        true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::User::CreateUserDTO,
                                                           Skribisto::Domain::User>();
}
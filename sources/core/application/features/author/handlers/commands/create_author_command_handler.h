/**
 * @file create_author_command_handler.h
 * @brief Contains the definition of the CreateAuthorCommandHandler class.
 */

#pragma once

#include "application_global.h"
#include "author_dto.h"
#include "cqrs/author/commands/create_author_command.h"
#include "handler.h"
#include "persistence/interface_author_repository.h"
#include "result.h"

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Commands;

namespace Application::Features::Author::Commands
{
/**
 * @brief Handles the CreateAuthorCommand and persists the new author to the data store.
 */
class SKR_APPLICATION_EXPORT CreateAuthorCommandHandler : public Handler
{
    Q_OBJECT
  public:
    /**
     * @brief Constructs a new CreateAuthorCommandHandler object.
     * @param repositories A pointer to the interface repositories object.
     */
    CreateAuthorCommandHandler(QSharedPointer<InterfaceAuthorRepository> repository);

    /**
     * @brief Handles a CreateAuthorCommand object and returns the UUID of the newly created author.
     * @param request The CreateAuthorCommand object to handle.
     * @return A Result object containing the UUID of the newly created author, or an error message if the operation
     * failed.
     */
    Result<AuthorDTO> handle(QPromise<Result<void>> &progressPromise, const CreateAuthorCommand &request);
    Result<AuthorDTO> restore();

  signals:
    void authorCreated(Contracts::DTO::Author::AuthorDTO result);
    void authorRemoved(Contracts::DTO::Author::AuthorDTO result);

  private:
    QSharedPointer<InterfaceAuthorRepository> m_repository; // A pointer to the interface repositories object.
    Result<AuthorDTO> handleImpl(const CreateAuthorCommand &request);
    Result<AuthorDTO> restoreImpl();
    Result<AuthorDTO> m_oldState;
};
} // namespace Application::Features::Author::Commands

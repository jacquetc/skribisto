#pragma once

#include "application_global.h"
#include "author_dto.h"
#include "cqrs/author/commands/remove_author_command.h"
#include "handler.h"
#include "persistence/interface_author_repository.h"
#include "result.h"

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Commands;

namespace Application::Features::Author::Commands
{
class SKR_APPLICATION_EXPORT RemoveAuthorCommandHandler : public Handler
{
    Q_OBJECT
  public:
    RemoveAuthorCommandHandler(QSharedPointer<InterfaceAuthorRepository> repository);
    Result<AuthorDTO> handle(QPromise<Result<void>> &progressPromise, const RemoveAuthorCommand &request);
    Result<AuthorDTO> restore();

  signals:
    void authorCreated(Contracts::DTO::Author::AuthorDTO result);
    void authorRemoved(Contracts::DTO::Author::AuthorDTO result);

  private:
    QSharedPointer<InterfaceAuthorRepository> m_repository;
    Result<AuthorDTO> handleImpl(const RemoveAuthorCommand &request);
    Result<AuthorDTO> restoreImpl();
    Result<AuthorDTO> saveOldState();
    Result<AuthorDTO> m_oldState;
};
} // namespace Application::Features::Author::Commands

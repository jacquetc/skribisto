#pragma once

#include "application_author_export.h"
#include "author/author_dto.h"
#include "author/commands/create_author_command.h"
#include "persistence/interface_author_repository.h"

#include "persistence/interface_book_repository.h"

#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Commands;

namespace Application::Features::Author::Commands
{
class SKRIBISTO_APPLICATION_AUTHOR_EXPORT CreateAuthorCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateAuthorCommandHandler(InterfaceAuthorRepository *repository, InterfaceBookRepository *ownerRepository);

    Result<AuthorDTO> handle(QPromise<Result<void>> &progressPromise, const CreateAuthorCommand &request);
    Result<AuthorDTO> restore();

  signals:
    void authorCreated(Contracts::DTO::Author::AuthorDTO authorDto);
    void authorRemoved(int id);

  private:
    InterfaceAuthorRepository *m_repository;
    InterfaceBookRepository *m_ownerRepository;
    Result<AuthorDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateAuthorCommand &request);
    Result<AuthorDTO> restoreImpl();
    Domain::Book m_oldOwnerEntity;
    Result<Domain::Author> m_newEntity;
    Domain::Book m_ownerEntityNewState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Author::Commands
#pragma once

#include "application_author_export.h"
#include "author/author_dto.h"
#include "author/commands/update_author_command.h"

#include "persistence/interface_author_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Commands;

namespace Application::Features::Author::Commands
{
class SKRIBISTO_APPLICATION_AUTHOR_EXPORT UpdateAuthorCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateAuthorCommandHandler(QSharedPointer<InterfaceAuthorRepository> repository);
    Result<AuthorDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateAuthorCommand &request);
    Result<AuthorDTO> restore();

  signals:
    void authorUpdated(Contracts::DTO::Author::AuthorDTO authorDto);

  private:
    QSharedPointer<InterfaceAuthorRepository> m_repository;
    Result<AuthorDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdateAuthorCommand &request);
    Result<AuthorDTO> restoreImpl();
    Result<AuthorDTO> saveOldState();
    Result<AuthorDTO> m_newState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Author::Commands
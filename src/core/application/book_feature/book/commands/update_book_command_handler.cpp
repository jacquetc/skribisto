// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_book_command_handler.h"
#include "book/validators/update_book_command_validator.h"
#include "repository/interface_book_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Book;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Book::Commands;
using namespace Skribisto::Contracts::CQRS::Book::Validators;
using namespace Skribisto::Application::Features::Book::Commands;

UpdateBookCommandHandler::UpdateBookCommandHandler(InterfaceBookRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<BookDTO> UpdateBookCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                 const UpdateBookCommand &request)
{
    Result<BookDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<BookDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateBookCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<BookDTO> UpdateBookCommandHandler::restore()
{
    Result<BookDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<BookDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateBookCommand restore:" << ex.what();
    }
    return result;
}

Result<BookDTO> UpdateBookCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                     const UpdateBookCommand &request)
{
    qDebug() << "UpdateBookCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateBookCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    QLN_RETURN_IF_ERROR(BookDTO, validatorResult)

    // save old state
    if (m_undoState.isEmpty())
    {
        Result<Skribisto::Domain::Book> currentResult = m_repository->get(request.req.id());

        QLN_RETURN_IF_ERROR(BookDTO, currentResult)

        // map
        m_undoState = Result<BookDTO>(
            Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Book, BookDTO>(currentResult.value()));
    }
    auto updateDto = Qleany::Tools::AutoMapper::AutoMapper::map<BookDTO, UpdateBookDTO>(m_undoState.value());
    updateDto << request.req;

    // map
    auto book = Qleany::Tools::AutoMapper::AutoMapper::map<UpdateBookDTO, Skribisto::Domain::Book>(updateDto);

    // set update timestamp only on first pass
    if (m_undoState.isEmpty())
    {
        book.setUpdateDate(QDateTime::currentDateTime());
    }

    // do
    auto bookResult = m_repository->update(std::move(book));

    if (bookResult.hasError())
    {
        return Result<BookDTO>(bookResult.error());
    }

    // map
    auto bookDto = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Book, BookDTO>(bookResult.value());

    emit bookUpdated(bookDto);

    if (request.req.metaData().areDetailsSet())
    {
        emit bookDetailsUpdated(bookDto.id());
    }

    qDebug() << "UpdateBookCommandHandler::handleImpl done";

    return Result<BookDTO>(bookDto);
}

Result<BookDTO> UpdateBookCommandHandler::restoreImpl()
{
    qDebug() << "UpdateBookCommandHandler::restoreImpl called with id" << m_undoState.value().uuid();

    // map
    auto book = Qleany::Tools::AutoMapper::AutoMapper::map<BookDTO, Skribisto::Domain::Book>(m_undoState.value());

    // do
    auto bookResult = m_repository->update(std::move(book));

    QLN_RETURN_IF_ERROR(BookDTO, bookResult)

    // map
    auto bookDto = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Book, BookDTO>(bookResult.value());

    emit bookUpdated(bookDto);

    qDebug() << "UpdateBookCommandHandler::restoreImpl done";

    return Result<BookDTO>(bookDto);
}

bool UpdateBookCommandHandler::s_mappingRegistered = false;

void UpdateBookCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Book, Contracts::DTO::Book::BookDTO>(
        true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Book::UpdateBookDTO,
                                                           Contracts::DTO::Book::BookDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Book::UpdateBookDTO,
                                                           Skribisto::Domain::Book>();
}
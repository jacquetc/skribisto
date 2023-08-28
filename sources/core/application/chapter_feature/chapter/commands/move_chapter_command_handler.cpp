#include "move_chapter_command_handler.h"
#include "automapper/automapper.h"
#include "chapter/move_chapter_dto.h"
#include "chapter/validators/move_chapter_command_validator.h"
#include <QDebug>

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Validators;
using namespace Application::Features::Chapter::Commands;

MoveChapterCommandHandler::MoveChapterCommandHandler(InterfaceBookRepository *bookRepository,
                                                     InterfaceChapterRepository *chapterRepository)
    : m_bookRepository(bookRepository), m_chapterRepository(chapterRepository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<MoveChapterReplyDTO> MoveChapterCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                              const MoveChapterCommand &request)
{
    Result<MoveChapterReplyDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<MoveChapterReplyDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling MoveChapterCommand:" << ex.what();
    }
    return result;
}

Result<MoveChapterReplyDTO> MoveChapterCommandHandler::restore()
{

    Result<MoveChapterReplyDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<MoveChapterReplyDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling MoveChapterCommand restore:" << ex.what();
    }
    return result;
}

Result<MoveChapterReplyDTO> MoveChapterCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                  const MoveChapterCommand &request)
{
    qDebug() << "MoveChapterCommandHandler::handleImpl called";

    Domain::Chapter chapter;

    if (m_newState.isEmpty())
    {

        // Validate the create chapter command using the validator
        auto validator = MoveChapterCommandValidator(m_bookRepository, m_chapterRepository);
        Result<void> validatorResult = validator.validate(request.req);

        if (Q_UNLIKELY(validatorResult.hasError()))
        {
            return Result<MoveChapterReplyDTO>(validatorResult.error());
        }

        // implement logic here which will not be repeated on restore
        chapter = AutoMapper::AutoMapper::map<MoveChapterDTO, Domain::Chapter>(request.req);
    }
    else
    {
        // implement logic here to load already filled newState for restore
        chapter = AutoMapper::AutoMapper::map<MoveChapterDTO, Domain::Chapter>(request.req);
    }

    m_bookRepository->beginChanges();

    // play here with the repositories
    Q_UNIMPLEMENTED();

    m_bookRepository->saveChanges();

    // dummy to compile:
    auto chapterResult = Result<Domain::Chapter>(chapter);

    // implement logic here to save to new state for restore
    auto moveChapterReplyDTO = AutoMapper::AutoMapper::map<Domain::Chapter, MoveChapterReplyDTO>(chapterResult.value());

    m_newState = Result<MoveChapterReplyDTO>(moveChapterReplyDTO);

    // emit signal
    emit moveChapterChanged(moveChapterReplyDTO);

    // Return
    return Result<MoveChapterReplyDTO>(moveChapterReplyDTO);
}

Result<MoveChapterReplyDTO> MoveChapterCommandHandler::restoreImpl()
{

    MoveChapterReplyDTO moveChapterReplyDTO;
    // auto moveChapterReplyDTO = AutoMapper::AutoMapper::map<Domain::Chapter, MoveChapterReplyDTO>(m_newState);
    Q_UNIMPLEMENTED();

    emit moveChapterChanged(moveChapterReplyDTO);

    return Result<MoveChapterReplyDTO>(moveChapterReplyDTO);
}

bool MoveChapterCommandHandler::s_mappingRegistered = false;

void MoveChapterCommandHandler::registerMappings()
{
}
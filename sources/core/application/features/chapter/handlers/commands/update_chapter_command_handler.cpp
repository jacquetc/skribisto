#include "update_chapter_command_handler.h"
#include "automapper/automapper.h"
#include "cqrs/chapter/validators/update_chapter_command_validator.h"
#include "persistence/interface_chapter_repository.h"

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Commands;
using namespace Contracts::CQRS::Chapter::Validators;
using namespace Application::Features::Chapter::Commands;

UpdateChapterCommandHandler::UpdateChapterCommandHandler(QSharedPointer<InterfaceChapterRepository> repository)
    : Handler(), m_repository(repository)
{
}
Result<ChapterDTO> UpdateChapterCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                       const UpdateChapterCommand &request)
{
    Result<ChapterDTO> result;

    try
    {
        result = handleImpl(request);
    }
    catch (const std::exception &ex)
    {
        result = Result<ChapterDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateChapterCommand:" << ex.what();
    }

    return result;
}

Result<ChapterDTO> UpdateChapterCommandHandler::restore()
{
    Result<ChapterDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<ChapterDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateChapterCommand restore:" << ex.what();
    }

    return result;
}

Result<ChapterDTO> UpdateChapterCommandHandler::handleImpl(const UpdateChapterCommand &request)
{
    qDebug() << "UpdateChapterCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateChapterCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);
    if (validatorResult.hasError())
    {
        return Result<ChapterDTO>(validatorResult.error());
    }

    // map
    auto chapter = AutoMapper::AutoMapper::map<Domain::Chapter, UpdateChapterDTO>(request.req);

    // set update timestamp only on first pass
    if (m_oldState.isEmpty())
    {
        chapter.setUpdateDate(QDateTime::currentDateTime());
    }

    // save old state
    if (m_oldState.isEmpty())
    {
        Result<Domain::Chapter> saveResult = m_repository->get(request.req.id());
        if (saveResult.hasError())
        {
            qDebug() << "Error getting chapter from repository:" << saveResult.error().message();
            return Result<ChapterDTO>(saveResult.error());
        }

        // map
        m_oldState = Result<ChapterDTO>(AutoMapper::AutoMapper::map<ChapterDTO, Domain::Chapter>(saveResult.value()));
    }
    // do
    auto chapterResult = m_repository->update(std::move(chapter));
    if (chapterResult.hasError())
    {
        return Result<ChapterDTO>(chapterResult.error());
    }

    // map
    auto chapterDto = AutoMapper::AutoMapper::map<ChapterDTO, Domain::Chapter>(chapterResult.value());

    emit chapterUpdated(chapterDto);

    qDebug() << "UpdateChapterCommandHandler::handleImpl done";

    return Result<ChapterDTO>(chapterDto);
}

Result<ChapterDTO> UpdateChapterCommandHandler::restoreImpl()
{
    qDebug() << "UpdateChapterCommandHandler::restoreImpl called with id" << m_oldState.value().uuid();
    // map
    auto chapter = AutoMapper::AutoMapper::map<Domain::Chapter, ChapterDTO>(m_oldState.value());

    // do
    auto chapterResult = m_repository->update(std::move(chapter));
    if (chapterResult.hasError())
    {
        return Result<ChapterDTO>(chapterResult.error());
    }

    // map
    auto chapterDto = AutoMapper::AutoMapper::map<ChapterDTO, Domain::Chapter>(chapterResult.value());

    emit chapterUpdated(chapterDto);

    qDebug() << "UpdateChapterCommandHandler::restoreImpl done";

    return Result<ChapterDTO>(chapterDto);
}

// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_chapter_command_handler.h"
#include "chapter/validators/update_chapter_command_validator.h"
#include "repository/interface_chapter_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Chapter;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Chapter::Commands;
using namespace Skribisto::Contracts::CQRS::Chapter::Validators;
using namespace Skribisto::Application::Features::Chapter::Commands;

UpdateChapterCommandHandler::UpdateChapterCommandHandler(InterfaceChapterRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<ChapterDTO> UpdateChapterCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                       const UpdateChapterCommand &request)
{
    Result<ChapterDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<ChapterDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateChapterCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
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
        result = Result<ChapterDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateChapterCommand restore:" << ex.what();
    }
    return result;
}

Result<ChapterDTO> UpdateChapterCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                           const UpdateChapterCommand &request)
{
    qDebug() << "UpdateChapterCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateChapterCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    QLN_RETURN_IF_ERROR(ChapterDTO, validatorResult)

    // save old state
    if (m_undoState.isEmpty())
    {
        Result<Skribisto::Domain::Chapter> currentResult = m_repository->get(request.req.id());

        QLN_RETURN_IF_ERROR(ChapterDTO, currentResult)

        // map
        m_undoState = Result<ChapterDTO>(
            Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Chapter, ChapterDTO>(currentResult.value()));
    }
    auto updateDto = Qleany::Tools::AutoMapper::AutoMapper::map<ChapterDTO, UpdateChapterDTO>(m_undoState.value());
    updateDto << request.req;

    // map
    auto chapter = Qleany::Tools::AutoMapper::AutoMapper::map<UpdateChapterDTO, Skribisto::Domain::Chapter>(updateDto);

    // set update timestamp only on first pass
    if (m_undoState.isEmpty())
    {
        chapter.setUpdateDate(QDateTime::currentDateTime());
    }

    // do
    auto chapterResult = m_repository->update(std::move(chapter));

    if (chapterResult.hasError())
    {
        return Result<ChapterDTO>(chapterResult.error());
    }

    // map
    auto chapterDto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Chapter, ChapterDTO>(chapterResult.value());

    emit chapterUpdated(chapterDto);

    if (request.req.metaData().areDetailsSet())
    {
        emit chapterDetailsUpdated(chapterDto.id());
    }

    qDebug() << "UpdateChapterCommandHandler::handleImpl done";

    return Result<ChapterDTO>(chapterDto);
}

Result<ChapterDTO> UpdateChapterCommandHandler::restoreImpl()
{
    qDebug() << "UpdateChapterCommandHandler::restoreImpl called with id" << m_undoState.value().uuid();

    // map
    auto chapter =
        Qleany::Tools::AutoMapper::AutoMapper::map<ChapterDTO, Skribisto::Domain::Chapter>(m_undoState.value());

    // do
    auto chapterResult = m_repository->update(std::move(chapter));

    QLN_RETURN_IF_ERROR(ChapterDTO, chapterResult)

    // map
    auto chapterDto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Chapter, ChapterDTO>(chapterResult.value());

    emit chapterUpdated(chapterDto);

    qDebug() << "UpdateChapterCommandHandler::restoreImpl done";

    return Result<ChapterDTO>(chapterDto);
}

bool UpdateChapterCommandHandler::s_mappingRegistered = false;

void UpdateChapterCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Chapter,
                                                           Contracts::DTO::Chapter::ChapterDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Chapter::UpdateChapterDTO,
                                                           Contracts::DTO::Chapter::ChapterDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Chapter::UpdateChapterDTO,
                                                           Skribisto::Domain::Chapter>();
}
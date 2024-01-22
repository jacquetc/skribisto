// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "remove_chapter_command_handler.h"
#include "chapter/validators/remove_chapter_command_validator.h"
#include "repository/interface_chapter_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Chapter;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Chapter::Commands;
using namespace Skribisto::Application::Features::Chapter::Commands;
using namespace Skribisto::Contracts::CQRS::Chapter::Validators;

RemoveChapterCommandHandler::RemoveChapterCommandHandler(InterfaceChapterRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<int> RemoveChapterCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                const RemoveChapterCommand &request)
{
    Result<int> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveChapterCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<int> RemoveChapterCommandHandler::restore()
{
    Result<int> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveChapterCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveChapterCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                    const RemoveChapterCommand &request)
{
    int chapterId = request.id;

    // Validate the command using the validator
    auto validator = RemoveChapterCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(chapterId);

    QLN_RETURN_IF_ERROR(int, validatorResult);

    Result<Skribisto::Domain::Chapter> chapterResult = m_repository->get(chapterId);

    QLN_RETURN_IF_ERROR(int, chapterResult)

    // save old entity
    m_oldState = chapterResult.value();

    auto deleteResult = m_repository->removeInCascade(QList<int>() << chapterId);

    QLN_RETURN_IF_ERROR(int, deleteResult)

    // repositories handle remove signals
    // emit chapterRemoved(deleteResult.value());

    qDebug() << "Chapter removed:" << chapterId;

    return Result<int>(chapterId);
}

Result<int> RemoveChapterCommandHandler::restoreImpl()
{
    // no restore possible
    return Result<int>(0);
}

bool RemoveChapterCommandHandler::s_mappingRegistered = false;

void RemoveChapterCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Chapter,
                                                           Contracts::DTO::Chapter::ChapterDTO>(true, true);
}
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "atelier_controller.h"

#include "atelier/queries/get_all_atelier_query_handler.h"
#include "atelier/queries/get_atelier_query_handler.h"
#include "atelier/queries/get_atelier_with_details_query_handler.h"
#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include <QCoroSignal>

using namespace Skribisto::Controller;
using namespace Skribisto::Controller::Atelier;
using namespace Skribisto::Application::Features::Atelier::Queries;
using namespace Qleany::Tools::UndoRedo;
using namespace Qleany::Contracts::Repository;

QPointer<AtelierController> AtelierController::s_instance = nullptr;

AtelierController::AtelierController(InterfaceRepositoryProvider *repositoryProvider,
                                     ThreadedUndoRedoSystem *undo_redo_system,
                                     QSharedPointer<EventDispatcher> eventDispatcher)
    : QObject{nullptr}
{
    m_repositoryProvider = repositoryProvider;

    // connections for undo commands:
    m_undo_redo_system = undo_redo_system;
    m_eventDispatcher = eventDispatcher;

    s_instance = this;
}

AtelierController *AtelierController::instance()
{
    return s_instance.data();
}

QCoro::Task<AtelierDTO> AtelierController::get(int id) const
{
    auto queryCommand = new QueryCommand("get");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetAtelierQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceAtelierRepository *>(m_repositoryProvider->repository("Atelier"));
        GetAtelierQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->atelier()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "atelier");

    // async wait for result signal
    const std::optional<AtelierDTO> optional_result =
        co_await qCoro(m_eventDispatcher->atelier(), &AtelierSignals::getReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return AtelierDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<AtelierWithDetailsDTO> AtelierController::getWithDetails(int id) const
{
    auto queryCommand = new QueryCommand("getWithDetails");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetAtelierQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceAtelierRepository *>(m_repositoryProvider->repository("Atelier"));
        GetAtelierWithDetailsQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->atelier()->getWithDetailsReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "atelier");

    // async wait for result signal
    const std::optional<AtelierWithDetailsDTO> optional_result = co_await qCoro(
        m_eventDispatcher.get()->atelier(), &AtelierSignals::getWithDetailsReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return AtelierWithDetailsDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<QList<AtelierDTO>> AtelierController::getAll() const
{
    auto queryCommand = new QueryCommand("getAll");

    queryCommand->setQueryFunction([&](QPromise<Result<void>> &progressPromise) {
        auto interface = static_cast<InterfaceAtelierRepository *>(m_repositoryProvider->repository("Atelier"));
        GetAllAtelierQueryHandler handler(interface);
        auto result = handler.handle(progressPromise);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->atelier()->getAllReplied(result.value());
        }
        return Result<void>(result.error());
    });
    m_undo_redo_system->push(queryCommand, "atelier");

    // async wait for result signal
    const std::optional<QList<AtelierDTO>> optional_result =
        co_await qCoro(m_eventDispatcher->atelier(), &AtelierSignals::getAllReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return QList<AtelierDTO>() << AtelierDTO();
    }

    co_return optional_result.value();
}

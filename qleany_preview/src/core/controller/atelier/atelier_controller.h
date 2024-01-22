// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "atelier/atelier_dto.h"
#include "atelier/atelier_with_details_dto.h"
#include "controller_export.h"
#include "event_dispatcher.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Skribisto::Contracts::DTO::Atelier;

namespace Skribisto::Controller::Atelier
{

class SKRIBISTO_CONTROLLER_EXPORT AtelierController : public QObject
{
    Q_OBJECT
  public:
    explicit AtelierController(InterfaceRepositoryProvider *repositoryProvider,
                               ThreadedUndoRedoSystem *undo_redo_system,
                               QSharedPointer<EventDispatcher> eventDispatcher);

    static AtelierController *instance();

    Q_INVOKABLE QCoro::Task<AtelierDTO> get(int id) const;

    Q_INVOKABLE QCoro::Task<AtelierWithDetailsDTO> getWithDetails(int id) const;

  public slots:

  private:
    static QPointer<AtelierController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    AtelierController() = delete;
    AtelierController(const AtelierController &) = delete;
    AtelierController &operator=(const AtelierController &) = delete;
};

} // namespace Skribisto::Controller::Atelier
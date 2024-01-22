// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"
#include "event_dispatcher.h"
#include "workspace/workspace_dto.h"
#include "workspace/workspace_with_details_dto.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Skribisto::Contracts::DTO::Workspace;

namespace Skribisto::Controller::Workspace
{

class SKRIBISTO_CONTROLLER_EXPORT WorkspaceController : public QObject
{
    Q_OBJECT
  public:
    explicit WorkspaceController(InterfaceRepositoryProvider *repositoryProvider,
                                 ThreadedUndoRedoSystem *undo_redo_system,
                                 QSharedPointer<EventDispatcher> eventDispatcher);

    static WorkspaceController *instance();

    Q_INVOKABLE QCoro::Task<WorkspaceDTO> get(int id) const;

    Q_INVOKABLE QCoro::Task<WorkspaceWithDetailsDTO> getWithDetails(int id) const;

    Q_INVOKABLE QCoro::Task<QList<WorkspaceDTO>> getAll() const;

  public slots:

  private:
    static QPointer<WorkspaceController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    WorkspaceController() = delete;
    WorkspaceController(const WorkspaceController &) = delete;
    WorkspaceController &operator=(const WorkspaceController &) = delete;
};

} // namespace Skribisto::Controller::Workspace
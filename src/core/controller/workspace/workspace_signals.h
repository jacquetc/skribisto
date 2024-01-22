// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"

#include "workspace/workspace_with_details_dto.h"

#include "workspace/workspace_dto.h"

#include <QObject>

namespace Skribisto::Controller
{

using namespace Skribisto::Contracts::DTO::Workspace;

class SKRIBISTO_CONTROLLER_EXPORT WorkspaceSignals : public QObject
{
    Q_OBJECT
  public:
    explicit WorkspaceSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void created(WorkspaceDTO dto);
    void updated(WorkspaceDTO dto);
    void allRelationsInvalidated(int id);
    void getReplied(WorkspaceDTO dto);
    void getWithDetailsReplied(WorkspaceWithDetailsDTO dto);
    void getAllReplied(QList<WorkspaceDTO> dtoList);
};
} // namespace Skribisto::Controller
// Adapted for MoveItems use case

#pragma once
#include "binder_item_management_dtos.h"
#include "database/db_context.h"
#include "database/snapshot_types.h"
#include "move_items_uc/i_move_items_uow.h"
#include "undo_redo/undo_redo_command.h"
#include <memory>

namespace Skribisto::BinderItemManagement
{

class MoveItemsUseCase
{

  public:
    explicit MoveItemsUseCase(std::unique_ptr<IMoveItemsUnitOfWork> uow);
    bool execute(const MoveDto &moveDto);
    Skribisto::Common::UndoRedo::Result<void> undo();
    Skribisto::Common::UndoRedo::Result<void> redo();

  private:
    std::unique_ptr<IMoveItemsUnitOfWork> m_uow;
    Common::Database::EntityTreeSnapshot m_undoSnapshot;
    MoveDto m_moveDto;
};

} // namespace Skribisto::BinderItemManagement

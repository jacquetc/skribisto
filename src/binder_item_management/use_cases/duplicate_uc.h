// Adapted for Duplicate use case

#pragma once
#include "binder_item_management_dtos.h"
#include "database/db_context.h"
#include "database/snapshot_types.h"
#include "duplicate_uc/i_duplicate_uow.h"
#include "undo_redo/undo_redo_command.h"
#include <memory>

namespace Skribisto::BinderItemManagement
{

class DuplicateUseCase
{

  public:
    explicit DuplicateUseCase(std::unique_ptr<IDuplicateUnitOfWork> uow);
    [[nodiscard]] DuplicateReturnDto execute(const DuplicateDto &duplicateDto);
    Skribisto::Common::UndoRedo::Result<void> undo();
    Skribisto::Common::UndoRedo::Result<void> redo();

  private:
    std::unique_ptr<IDuplicateUnitOfWork> m_uow;
    Common::Database::EntityTreeSnapshot m_undoSnapshot;
    DuplicateDto m_duplicateDto;
    QList<int> m_createdItemIds;
    QList<int> m_createdContentIds;
};

} // namespace Skribisto::BinderItemManagement

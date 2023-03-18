#pragma once

#include "undo_redo_command.h"

namespace Presenter::UndoRedo
{

class QueryCommand : public UndoRedoCommand
{
    Q_OBJECT
  public:
    QueryCommand(const QString &text);

    void setQueryFunction(const std::function<Result<void>()> &function);

  private:
    // Overrides the redo() method of UndoRedoCommand
    Result<void> redo() override;

    // Overrides the undo() method of UndoRedoCommand
    Result<void> undo() override;

  private:
    std::function<Result<void>()>
        m_queryFunction; /*!< The function to be executed asynchronously when the redo() method is called. */
};
} // namespace Presenter::UndoRedo

#pragma once
#include "undo_redo_command.h"

namespace Presenter::UndoRedo
{

template <class Handler, class Request> class AlterCommand : public UndoRedoCommand
{

  public:
    AlterCommand(const QString &text, Handler *handler, const Request &request)
        : UndoRedoCommand(text), m_handler(handler), m_request(request)
    {
    }
    // UndoRedoCommand interface
  public:
    Result<void> undo() override
    {
        return Result<void>(m_handler->restore().error());
    }
    Result<void> redo() override
    {
        return Result<void>(m_handler->handle(m_request).error());
    }

  private:
    Handler *m_handler;
    Request m_request;
};

} // namespace Presenter::UndoRedo

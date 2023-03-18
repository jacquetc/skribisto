#pragma once

#include "undo_redo_command.h"
#include <QAction>
#include <QHash>
#include <QObject>
#include <QQueue>
#include <QSharedPointer>

namespace Presenter::UndoRedo
{
class UndoRedoSystem : public QObject
{
    Q_OBJECT
  public:
    UndoRedoSystem(QObject *parent = nullptr);

    Q_INVOKABLE void run();

    Q_INVOKABLE bool canUndo() const;

    Q_INVOKABLE bool canRedo() const;

    Q_INVOKABLE void undo();

    Q_INVOKABLE void redo();

    Q_INVOKABLE void push(Presenter::UndoRedo::UndoRedoCommand *command,
                          const Presenter::UndoRedo::UndoRedoCommand::Scope &scope);

    Q_INVOKABLE void clear();

    Q_INVOKABLE void setUndoLimit(int limit);

    Q_INVOKABLE int undoLimit() const;

    Q_INVOKABLE QString undoText() const;

    Q_INVOKABLE QString redoText() const;

  private slots:
    void onCommandFinished();

  signals:

    void stateChanged();

    void finished();
    void errorSent(Error error);

  private:
    void executeNextCommand(const UndoRedoCommand::Scope &scope);

    int m_undoLimit;    /*!< The maximum number of undo commands that can be stored in the undo-redo system. */
    int m_currentIndex; /*!< The current index in the command history. */

    QQueue<QSharedPointer<UndoRedoCommand>> m_generalCommandQueue;
    QHash<UndoRedoCommand::Scope, QQueue<QSharedPointer<UndoRedoCommand>>> m_scopedCommandQueueHash;
    QHash<UndoRedoCommand::Scope, QSharedPointer<UndoRedoCommand>> m_currentCommandHash;
};
} // namespace Presenter::UndoRedo

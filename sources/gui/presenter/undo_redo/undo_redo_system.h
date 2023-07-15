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
    UndoRedoSystem(QObject *parent, Scopes scopes);

    Q_INVOKABLE void run();

    Q_INVOKABLE bool canUndo() const;

    Q_INVOKABLE bool canRedo() const;

    Q_INVOKABLE void undo();

    Q_INVOKABLE void redo();

    Q_INVOKABLE void push(Presenter::UndoRedo::UndoRedoCommand *command, const QString &commandScope);

    Q_INVOKABLE void clear();

    Q_INVOKABLE void setUndoLimit(int limit);

    Q_INVOKABLE int undoLimit() const;

    Q_INVOKABLE QString undoText() const;

    Q_INVOKABLE QString redoText() const;

    Q_INVOKABLE QStringList undoRedoTextList() const;

    Q_INVOKABLE int currentIndex() const;

    QStringList queuedCommandTextListByScope(const QString &scopeFlagString) const;
    bool isRunning() const;
  private slots:
    void onCommandFinished();

  signals:

    void stateChanged();

    void finished();
    void errorSent(Error error);
    /*!
     * \brief A signal that is emitted when the undo redo system is about to start redoing.
     * actions.
     */
    void redoing(Scope scope, bool active);
    /*!
     * \brief A signal that is emitted when the undo redo system is about to start undoing.
     * actions.
     */
    void undoing(Scope scope, bool active);

  private:
    void executeNextCommand(const ScopeFlag &scopeFlag);
    bool isCommandAllowedToRun(QSharedPointer<UndoRedoCommand> command, const ScopeFlag &currentScopeFlag);

    int m_undoLimit;    /*!< The maximum number of undo commands that can be stored in the undo-redo system. */
    int m_currentIndex; /*!< The current index in the command history. */
    Scopes m_scopes;

    QQueue<QSharedPointer<UndoRedoCommand>> m_generalCommandQueue;
    QHash<ScopeFlag, QQueue<QSharedPointer<UndoRedoCommand>>> m_scopedCommandQueueHash;
    QHash<ScopeFlag, QSharedPointer<UndoRedoCommand>> m_currentCommandHash;
};
} // namespace Presenter::UndoRedo

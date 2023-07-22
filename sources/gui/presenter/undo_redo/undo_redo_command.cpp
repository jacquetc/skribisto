#include "undo_redo_command.h"
#include "QtConcurrent/qtconcurrentrun.h"

#include <QFuture>

using namespace Presenter::UndoRedo;
/*!
 * \class UndoRedoCommand
 * \inmodule Presenter::UndoRedo
 * \brief A base class for implementing undo and redo commands.
 * Represents a base class for undo-redo commands in the application. Derived classes should implement undo() and redo()
 * methods to define the behavior of the command during undo and redo operations.
 */

/*!
 * \brief Constructs an UndoRedoCommand with the specified \a text.
 */
UndoRedoCommand::UndoRedoCommand(const QString &text) : QObject(nullptr), m_text(text), m_status(Status::Waiting)
{
    m_watcher = new QFutureWatcher<Result<void>>(this);
    connect(m_watcher, &QFutureWatcher<void>::finished, this, &UndoRedoCommand::onFinished);
    connect(m_watcher, &QFutureWatcher<void>::finished, this, &UndoRedoCommand::progressFinished);
    connect(m_watcher, &QFutureWatcher<void>::progressRangeChanged, this, &UndoRedoCommand::progressRangeChanged);
    connect(m_watcher, &QFutureWatcher<void>::progressTextChanged, this, &UndoRedoCommand::progressTextChanged);
    connect(m_watcher, &QFutureWatcher<void>::progressValueChanged, this, &UndoRedoCommand::progressValueChanged);
}

/*!
 * \brief Constructs an UndoRedoCommand with the specified \a text.
 */
void UndoRedoCommand::asyncUndo()
{
    m_status = Status::Running;
    emit undoing(m_scope, true);
    QFuture<Result<void>> future = QtConcurrent::run(&UndoRedoCommand::undo, this);
    m_watcher->setFuture(future);
}

/*!
 * \brief Performs a redo operation asynchronously.
 */
void UndoRedoCommand::asyncRedo()
{
    m_status = Status::Running;
    emit redoing(m_scope, true);
    QFuture<Result<void>> future = QtConcurrent::run(&UndoRedoCommand::redo, this);
    m_watcher->setFuture(future);
}

/*!
 * \brief Returns true if the command is currently running, otherwise false.
 */
bool UndoRedoCommand::isRunning() const
{
    return m_status == Status::Running;
}
bool UndoRedoCommand::isWaiting() const
{
    return m_status == Status::Waiting;
}
bool UndoRedoCommand::isFinished() const
{
    return m_status == Status::Finished;
}

/*!
 * \brief Handles the finished signal from the asynchronous undo or redo operation.
 */
void UndoRedoCommand::onFinished()
{
    Result<void> result = m_watcher->result();
    if (result.hasError())
    {
        this->setObsolete(true);
        emit errorSent(result.error());
    }
    m_status = Status::Finished;
    emit redoing(m_scope, false);
    emit undoing(m_scope, false);
    emit progressFinished();
    emit finished(result.isOk());
}

QUuid UndoRedoCommand::stackId() const
{
    return m_stackId;
}

void UndoRedoCommand::setStackId(const QUuid &newStackId)
{
    m_stackId = newStackId;
}

bool UndoRedoCommand::isSystem() const
{
    return m_isSystem;
}

void UndoRedoCommand::setIsSystem(bool newIsSystem)
{
    m_isSystem = newIsSystem;
}

/*!
 * \brief Returns true if the command is obsolete, otherwise false.
 * The command will then be deleted from the stack.
 */
bool UndoRedoCommand::obsolete() const
{
    return m_obsolete;
}

/*!
 * \brief Sets the obsolete status of the command to \a newObsolete.
 */
void UndoRedoCommand::setObsolete(bool newObsolete)
{
    m_obsolete = newObsolete;
}

/*!
 * \brief Merge with another command. Redo of current command must becomes the equivalent of both redoes. Same for
 * undoes.  Returns true if the command can be merged with \a other, otherwise false. To be implemented if needed.
 */
bool UndoRedoCommand::mergeWith(const UndoRedoCommand *other)
{
    return false;
}

/*!
 * \brief Returns the text of the command.
 */
QString UndoRedoCommand::text() const
{
    return m_text;
}

/*!
 * \brief Sets the text of the command to \a newText.
 */
void UndoRedoCommand::setText(const QString &newText)
{
    m_text = newText;
}

/*!
 * \brief Returns the scope of the command.
 */
Scope UndoRedoCommand::scope() const
{
    return m_scope;
}

/*!
 * \brief Sets the scope of the command to \a newScope.
 */
void UndoRedoCommand::setScope(Scope newScope)
{
    m_scope = newScope;
}

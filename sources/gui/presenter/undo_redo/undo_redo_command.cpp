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
UndoRedoCommand::UndoRedoCommand(const QString &text)
    : QObject(nullptr), m_text(text), m_running(false), m_finished(false)
{
    m_watcher = new QFutureWatcher<Result<void>>(this);
    connect(m_watcher, &QFutureWatcher<void>::finished, this, &UndoRedoCommand::onFinished);
    connect(m_watcher, &QFutureWatcher<void>::progressRangeChanged, this, &UndoRedoCommand::progressRangeChanged);
    connect(m_watcher, &QFutureWatcher<void>::progressTextChanged, this, &UndoRedoCommand::progressTextChanged);
    connect(m_watcher, &QFutureWatcher<void>::progressValueChanged, this, &UndoRedoCommand::progressValueChanged);
}

/*!
 * \brief Constructs an UndoRedoCommand with the specified \a text.
 */
void UndoRedoCommand::asyncUndo()
{
    m_running = true;
    QFuture<Result<void>> future = QtConcurrent::run(&UndoRedoCommand::undo, this);
    m_watcher->setFuture(future);
}

/*!
 * \brief Performs a redo operation asynchronously.
 */
void UndoRedoCommand::asyncRedo()
{
    m_running = true;
    QFuture<Result<void>> future = QtConcurrent::run(&UndoRedoCommand::redo, this);
    m_watcher->setFuture(future);
}

/*!
 * \brief Returns true if the command is currently running, otherwise false.
 */
bool UndoRedoCommand::isRunning() const
{
    return m_running;
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
    m_running = false;
    m_finished = true;
    emit finished();
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
UndoRedoCommand::Scope UndoRedoCommand::scope() const
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

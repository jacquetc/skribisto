#include "undo_redo_system.h"
#include "undo_redo/query_command.h"

#include <QEventLoop>

using namespace Presenter::UndoRedo;
/*!
 * \class UndoRedoSystem
 * \inmodule Presenter::UndoRedo
 * \brief A QObject that manages the undo and redo command history.
 */

/*!
 * \brief Constructs an UndoRedoSystem with the specified \a parent.
 */
UndoRedoSystem::UndoRedoSystem(QObject *parent) : QObject(parent), m_currentIndex(-1), m_undoLimit(10)
{
}

/*!
 * \brief Initializes the event loop for the UndoRedoSystem.
 */
void UndoRedoSystem::run()
{
    // Create an event loop for the thread
    QEventLoop eventLoop;

    // Connect the thread started signal to the event loop quit slot
    connect(this, &UndoRedoSystem::finished, &eventLoop, &QEventLoop::quit);

    // Execute the event loop
    eventLoop.exec();
}

/*!
 * \brief Returns true if an undo operation can be performed, otherwise false.
 */
bool UndoRedoSystem::canUndo() const
{
    return m_currentIndex >= 0 && !m_generalCommandQueue[m_currentIndex]->isRunning();
}

/*!
 * \brief Returns true if a redo operation can be performed, otherwise false.
 */
bool UndoRedoSystem::canRedo() const
{
    return m_currentIndex < m_generalCommandQueue.size() - 1 && !m_generalCommandQueue[m_currentIndex + 1]->isRunning();
}

/*!
 * \brief Performs an undo operation if it can be performed.
 */
void UndoRedoSystem::undo()
{
    if (canUndo())
    {
        m_generalCommandQueue[m_currentIndex--]->asyncUndo();
        emit stateChanged();
    }
}

/*!
 * \brief Performs a redo operation if it can be performed.
 */
void UndoRedoSystem::redo()
{
    if (canRedo())
    {

        m_generalCommandQueue[++m_currentIndex]->asyncRedo();
        emit stateChanged();
    }
}

/*!
 * \brief Adds an \a command to the UndoRedoSystem with the specified \a scope.
 */
void UndoRedoSystem::push(UndoRedoCommand *command, const UndoRedoCommand::Scope &scope)
{
    command->setParent(this);
    command->setScope(scope);

    QQueue<QSharedPointer<UndoRedoCommand>> &queue = m_scopedCommandQueueHash[scope];
    // Enqueue the command
    queue.enqueue(QSharedPointer<UndoRedoCommand>(command));

    // If the system is not currently executing a command, start executing the next command
    if (!m_currentCommandHash[scope])
    {
        executeNextCommand(scope);
    }

    // Emit the stateChanged signal
    emit stateChanged();
}

/*!
 * \brief Clears the UndoRedoSystem command history.
 */
void UndoRedoSystem::clear()
{
    // Clear the general command queue, not the scoped command queue
    this->m_generalCommandQueue.clear();
    // Set the current index to -1
    this->m_currentIndex = -1;
    // Emit the stateChanged signal
    emit stateChanged();
}

/*!
 * \brief Sets the undo limit to \a limit.
 */
void UndoRedoSystem::setUndoLimit(int limit)
{
    m_undoLimit = limit;
    // Remove excess commands from the general command queue if necessary
    while (m_generalCommandQueue.size() > m_undoLimit)
    {
        m_generalCommandQueue.dequeue();
        m_currentIndex--;
    }
    // Emit the stateChanged signal
    emit stateChanged();
}

/*!
 * \brief Returns the undo limit. Defaults to 10.
 */
int UndoRedoSystem::undoLimit() const
{
    return m_undoLimit;
}

/*!
 * \brief Returns the text of the undo command, or an empty QString if there is no command to undo.
 */
QString UndoRedoSystem::undoText() const
{
    if (m_currentIndex >= 0)
    {
        return m_generalCommandQueue[m_currentIndex]->text();
    }
    else
    {
        return QString();
    }
}

/*!
 * \brief Returns the text of the next redo command in the queue, or an empty QString if there is no command to redo.
 */
QString UndoRedoSystem::redoText() const
{
    if (m_currentIndex < m_generalCommandQueue.size() - 1)
    {
        return m_generalCommandQueue[m_currentIndex + 1]->text();
    }
    else
    {
        return QString();
    }
}

/*!
 * \brief Handles the finished() signal from an UndoRedoCommand and updates the UndoRedoSystem state.
 */
void UndoRedoSystem::onCommandFinished()
{

    auto command = dynamic_cast<UndoRedoCommand *>(QObject::sender());

    const UndoRedoCommand::Scope &scope = command->scope();

    // Set the current command to nullptr
    m_currentCommandHash.insert(scope, QSharedPointer<UndoRedoCommand>());

    if (command->obsolete())
    {
        m_generalCommandQueue.removeLast();
        m_currentIndex--;
    }

    // If there are commands in the queue, execute the next one
    if (!m_scopedCommandQueueHash[scope].isEmpty())
    {
        executeNextCommand(scope);
    }

    // Emit the stateChanged signal
    emit stateChanged();
}

/*!
 * \brief Executes the next command in the queue for the specified \a scope.
 */
void UndoRedoSystem::executeNextCommand(const UndoRedoCommand::Scope &scope)
{

    // keep in store only the true UndoRedoCommands, not QueryCommand
    if (qSharedPointerDynamicCast<QueryCommand>(m_scopedCommandQueueHash[scope].head()).isNull())
    {
        // Remove any redo commands that are after the current index
        m_generalCommandQueue.erase(m_generalCommandQueue.begin() + m_currentIndex + 1, m_generalCommandQueue.end());

        // Add the new command to the end of the list

        m_generalCommandQueue.enqueue(m_scopedCommandQueueHash[scope].head());
        // Increment the current index
        m_currentIndex++;

        // Remove excess commands from the general command queue if necessary
        while (m_generalCommandQueue.size() > m_undoLimit)
        {
            m_generalCommandQueue.dequeue();
            m_currentIndex--;
        }
    }

    // Dequeue the next command
    QQueue<QSharedPointer<UndoRedoCommand>> &queue = m_scopedCommandQueueHash[scope];
    QSharedPointer<UndoRedoCommand> command = queue.dequeue();

    m_currentCommandHash.insert(scope, command);

    // Connect the finished signal to the onCommandFinished slot
    connect(command.data(), &UndoRedoCommand::finished, this, &UndoRedoSystem::onCommandFinished, Qt::UniqueConnection);

    // Connect the errorSent signal to the errorSent signal
    connect(command.data(), &UndoRedoCommand::errorSent, this, &UndoRedoSystem::errorSent, Qt::UniqueConnection);

    // Execute the command asynchronously
    command->asyncRedo();
}

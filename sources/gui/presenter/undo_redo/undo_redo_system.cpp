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
UndoRedoSystem::UndoRedoSystem(QObject *parent, Scopes scopes)
    : QObject(parent), m_currentIndex(-1), m_undoLimit(10), m_scopes(scopes)
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
    bool hasACommandRunning = false;
    for (const auto &command : m_generalCommandQueue)
    {
        if (command->isRunning())
        {
            hasACommandRunning = true;
            break;
        }
    }

    return m_currentIndex >= 0 && !hasACommandRunning;
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
void UndoRedoSystem::push(UndoRedoCommand *command, const QString &commandScope)
{
    command->setParent(this);

    Scope scope = m_scopes.createScopeFromString(commandScope);
    command->setScope(scope);

    auto commandSharedPointer = QSharedPointer<UndoRedoCommand>(command);

    // Add the command to the appropriate queues
    for (ScopeFlag scopeFlag : m_scopes.flags())
    {
        if (scope.hasScopeFlag(scopeFlag))
        {
            QQueue<QSharedPointer<UndoRedoCommand>> &commandQueue = m_scopedCommandQueueHash[scopeFlag];
            commandQueue.enqueue(commandSharedPointer);

            // If the system is not currently executing a command for the given scope flag, start executing the next
            // command
            if (m_currentCommandHash[scopeFlag].isNull())
            {
                executeNextCommand(scopeFlag);
            }
        }
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
 * \brief Returns the text of the next redo command in the queue, or an empty QString if there is no command to
 * redo.
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

QStringList UndoRedoSystem::undoRedoTextList() const
{
    QStringList resultList;

    for (const auto &command : m_generalCommandQueue)
    {
        if (!command.isNull())
            resultList << command->text();
    }
    return resultList;
}

int UndoRedoSystem::currentIndex() const
{
    return m_currentIndex;
}

bool UndoRedoSystem::isRunning() const
{

    for (ScopeFlag scopeFlag : m_scopes.flags())
    {

        if (!m_scopedCommandQueueHash[scopeFlag].isEmpty())
        {
            return true;
        }

        if (!m_currentCommandHash.value(scopeFlag, QSharedPointer<UndoRedoCommand>()).isNull() &&
            !m_currentCommandHash.value(scopeFlag, QSharedPointer<UndoRedoCommand>())->isFinished())
        {
            return true;
        }
    }

    for (const auto &command : m_generalCommandQueue)
    {
        if (command->isRunning())
        {
            return true;
        }
    }

    return false;
}

QStringList UndoRedoSystem::queuedCommandTextListByScope(const QString &scopeFlagString) const
{
    QStringList resultList;

    ScopeFlag scopeFlag = m_scopes.flags(scopeFlagString);

    const QQueue<QSharedPointer<UndoRedoCommand>> &queue = m_scopedCommandQueueHash[scopeFlag];
    for (const QSharedPointer<UndoRedoCommand> &command : queue)
    {
        resultList << command->text();
    }
    return resultList;
}

/*!
 * \brief Handles the finished() signal from an UndoRedoCommand and updates the UndoRedoSystem state.
 */
void UndoRedoSystem::onCommandFinished()
{
    auto *senderObject = QObject::sender();
    auto *command = dynamic_cast<UndoRedoCommand *>(senderObject);
    if (command == nullptr)
    {
        qFatal("Command pointer is null!");
    }
    const Scope &commandScope = command->scope();

    // Set the current command to nullptr

    for (const ScopeFlag &scopeFlag : commandScope.flags())
    {

        m_currentCommandHash.insert(scopeFlag, QSharedPointer<UndoRedoCommand>());
    }

    if (nullptr == qobject_cast<QueryCommand *>(QObject::sender()))
    {
        if (command->obsolete())
        {
            m_generalCommandQueue.removeLast();
            m_currentIndex--;
        }
    }

    // If there are commands in the queue, execute the next one

    for (ScopeFlag scopeFlag : commandScope.flags())
    {
        if (!m_scopedCommandQueueHash[scopeFlag].isEmpty())
        {
            executeNextCommand(scopeFlag);
        }
    }

    // Emit the stateChanged signal
    emit stateChanged();
}

/*!
 * \brief Executes the next command in the queue for the specified \a scope.
 */
void UndoRedoSystem::executeNextCommand(const ScopeFlag &scopeFlag)
{
    // Dequeue the next command
    QQueue<QSharedPointer<UndoRedoCommand>> &queue = m_scopedCommandQueueHash[scopeFlag];
    QSharedPointer<UndoRedoCommand> command = queue.dequeue();

    m_currentCommandHash.insert(scopeFlag, command);
    bool allowedToRun = isCommandAllowedToRun(command, scopeFlag);

    // keep in general command queue only the true AlterCommands, not QueryCommand, and only if it's allowed to run
    if (qSharedPointerDynamicCast<QueryCommand>(command).isNull() && allowedToRun)
    {

        // Remove any redo commands that are after the current index
        m_generalCommandQueue.erase(m_generalCommandQueue.begin() + m_currentIndex + 1, m_generalCommandQueue.end());

        // Add the new command to the end of the list if not already there:

        if (!m_generalCommandQueue.contains(command))
        {
            m_generalCommandQueue.enqueue(command);
            // Increment the current index
            m_currentIndex++;

            // Remove excess commands from the general command queue if necessary
            while (m_generalCommandQueue.size() > m_undoLimit)
            {
                m_generalCommandQueue.dequeue();
                m_currentIndex--;
            }
        }
    }

    // if not already running (from another scope)
    if (allowedToRun)
    {
        // Connect the finished signal to the onCommandFinished slot
        connect(command.data(), &UndoRedoCommand::finished, this, &UndoRedoSystem::onCommandFinished,
                Qt::UniqueConnection);

        // Connect the errorSent signal to the errorSent signal
        connect(command.data(), &UndoRedoCommand::errorSent, this, &UndoRedoSystem::errorSent, Qt::UniqueConnection);

        // Execute the command asynchronously

        command->asyncRedo();
    }
}

/*!
 * \brief Check if the command \a is allowed to run. It verifies if all queues corresponding to the command scope have
 * this command as current, allowing it to be run.
 */
bool UndoRedoSystem::isCommandAllowedToRun(QSharedPointer<UndoRedoCommand> command, const ScopeFlag &currentScopeFlag)
{
    // get command scopes
    const Scope &commandScope = command->scope();

    if (commandScope.flags().count() == 1)
    {
        return true;
    }

    bool result = true;
    // check if this command is current for each scope
    for (ScopeFlag scopeFlag : commandScope.flags())
    {

        if (m_scopedCommandQueueHash[scopeFlag].contains(command))
        {
            return false;
        }

        if (currentScopeFlag == scopeFlag)
        {
            continue;
        }

        // check if the command is already running or finished for this scope
        bool isAlreadyRunning = false;
        if (!m_currentCommandHash.value(scopeFlag).isNull())
        {
            isAlreadyRunning = (m_currentCommandHash.value(scopeFlag) == command);
        }

        if (isAlreadyRunning && command->isRunning())
        {
            // if the command is already running, but not finished yet, it cannot be allowed to run again
            return false;
        }
        else if (isAlreadyRunning && command->isFinished())
        {
            // if the command is already finished, it cannot be allowed to run again
            return false;
        }
        else if (isAlreadyRunning && command->isWaiting())
        {
            // if the command is waiting, it can be allowed to run
            result |= true;
        }
    }

    // if the command was already running or finished for all relevant scopes, or if it has no relevant scopes, it
    // cannot be allowed to run
    return result;
}

#include "undo_redo_system.h"
#include "undo_redo/query_command.h"

#include <QEventLoop>
#include <QTimer>

using namespace Presenter::UndoRedo;
/*!
 * \class UndoRedoSystem
 * \inmodule Presenter::UndoRedo
 * \brief A QObject that manages the undo and redo command history.
 */

/*!
 * \brief Constructs an UndoRedoSystem with the specified \a parent.
 */
UndoRedoSystem::UndoRedoSystem(QObject *parent, Scopes scopes) : QObject(parent), m_undoLimit(10), m_scopes(scopes)
{
    qRegisterMetaType<Presenter::UndoRedo::Scope>();

    // Create the main stack
    m_stackHash.insert(QUuid(), QSharedPointer<UndoRedoStack>(new UndoRedoStack(this)));
    m_activeStack = m_stackHash[QUuid()];
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
    for (const auto &command : m_activeStack->queue())
    {
        if (command->isRunning())
        {
            hasACommandRunning = true;
            break;
        }
    }
    return m_activeStack->currentIndex() >= 0 && !hasACommandRunning;
}

/*!
 * \brief Returns true if a redo operation can be performed, otherwise false.
 */
bool UndoRedoSystem::canRedo() const
{
    return m_activeStack->currentIndex() < m_activeStack->queue().size() - 1 &&
           !m_activeStack->queue()[m_activeStack->currentIndex() + 1]->isRunning();
}

/*!
 * \brief Performs an undo operation if it can be performed.
 */
void UndoRedoSystem::undo()
{
    if (canUndo())
    {
        auto command = m_activeStack->queue()[m_activeStack->currentIndex()];
        m_activeStack->decrementCurrentIndex();
        // Connect the finished signal to the onCommandUndoFinished slot,  single shot connection to avoid unwanted
        // signal
        connect(command.data(), &UndoRedoCommand::finished, this, &UndoRedoSystem::onCommandUndoFinished,
                Qt::SingleShotConnection);

        for (ScopeFlag scopeFlag : command->scope().flags())
        {
            if (!m_scopedCommandQueueHash[scopeFlag].isEmpty())
            {
                m_currentCommandHash.insert(scopeFlag, command);
            }
        }

        command->asyncUndo();
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
        m_activeStack->incrementCurrentIndex();
        auto command = m_activeStack->queue()[m_activeStack->currentIndex()];
        // Connect the finished signal to the onCommandRedoFinished slot,  single shot connection to avoid unwanted
        // signal
        connect(command.data(), &UndoRedoCommand::finished, this, &UndoRedoSystem::onCommandRedoFinished,
                Qt::SingleShotConnection);

        for (ScopeFlag scopeFlag : command->scope().flags())
        {
            if (!m_scopedCommandQueueHash[scopeFlag].isEmpty())
            {
                m_currentCommandHash.insert(scopeFlag, command);
            }
        }

        command->asyncRedo();
        emit stateChanged();
    }
}

/*!
 * \brief Adds an \a command to the UndoRedoSystem with the specified \a scope.
 */
void UndoRedoSystem::push(UndoRedoCommand *command, const QString &commandScope, const QUuid &stackId)
{
    command->setParent(this);

    Scope scope = m_scopes.createScopeFromString(commandScope);
    command->setScope(scope);
    command->setStackId(stackId);

    auto commandSharedPointer = QSharedPointer<UndoRedoCommand>(command);

    // ensure the stack exists, else create it
    if (!m_stackHash.contains(stackId))
    {
        m_stackHash.insert(stackId, QSharedPointer<UndoRedoStack>(new UndoRedoStack(this, stackId)));
    }

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
                executeNextCommandDo(scopeFlag, stackId);
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

    for (auto stack : m_stackHash.values())
    {
        stack->queue().clear();
    }

    // Set the current index to -1

    for (auto stack : m_stackHash.values())
    {
        stack->setCurrentIndex(-1);
    }
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
    for (auto stack : m_stackHash.values())
    {
        while (stack->queue().size() > m_undoLimit)
        {
            stack->queue().dequeue();
            stack->decrementCurrentIndex();
        }
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
    if (m_activeStack->currentIndex() >= 0)
    {
        return m_activeStack->queue()[m_activeStack->currentIndex()]->text();
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
    if (m_activeStack->currentIndex() < m_activeStack->queue().size() - 1)
    {
        return m_activeStack->queue()[m_activeStack->currentIndex() + 1]->text();
    }
    else
    {
        return QString();
    }
}

QStringList UndoRedoSystem::undoRedoTextList() const
{
    QStringList resultList;

    for (const auto &command : m_activeStack->queue())
    {
        if (!command.isNull())
            resultList << command->text();
    }
    return resultList;
}

int UndoRedoSystem::currentIndex() const
{
    return m_activeStack->currentIndex();
}

/*!
 * \brief UndoRedoSystem::setCurrentIndex
 * \param index
 * \param stackId
 *
 *
 */
void UndoRedoSystem::setCurrentIndex(int index)
{
    // since undo redo is asynchronous, each undo or redo much wait for the previous one to finish
    // so we can't just set the current index to the new index
    // instead we need to undo or redo until we reach the desired index.
    // If the desired index is less than the current index, we undo until we reach the desired index, waiting for the
    // undo to finish each time.
    // If the desired index is greater than the current index, we redo until we reach the
    // desired index waiting for the redo to finish each time.
    // If the desired index is equal to the current index, we do nothing.

    if (index == m_activeStack->currentIndex())
        return;

    if (index >= m_activeStack->queue().size())
    {
        qWarning() << "UndoRedoSystem::setCurrentIndex: index out of range, setting to last index";
        index = m_activeStack->queue().size() - 1;
    }

    if (index < 0)
    {
        qWarning() << "UndoRedoSystem::setCurrentIndex: index out of range, setting to 0";
        index = 0;
    }

    if (!m_readyAfterSettingIndex)
    {
        qWarning() << "UndoRedoSystem::setCurrentIndex: not ready after setting index, using a delay system";

        // we need to wait for the system to be ready using QEvenLoop

        QTimer timer;
        QEventLoop loop;

        connect(&timer, &QTimer::timeout, &loop, &QEventLoop::quit);
        timer.start(200); // 200ms
        loop.exec();

        if (!m_readyAfterSettingIndex)
        {
            qFatal() << "UndoRedoSystem::setCurrentIndex: not ready after setting index, aborting";
            return;
        }
    }

    m_readyAfterSettingIndex = false;

    int movingIndex = m_activeStack->currentIndex();

    if (index < m_activeStack->currentIndex())
    {
        // we append the command to a undo queue
        // when the command is done, it will emit a signal
        // we will then undo the next command in the queue

        while (movingIndex > index)
        {
            auto command = m_activeStack->queue()[movingIndex];
            connect(command.data(), &UndoRedoCommand::finished, this, &UndoRedoSystem::executeNextCommandUndo,
                    Qt::SingleShotConnection);
            m_setIndexCommandQueue.enqueue(command);
            movingIndex--;
        }

        // execute the first command
        executeNextCommandUndo();
    }
    else if (index > m_activeStack->currentIndex())
    {
        while (movingIndex < index)
        {
            movingIndex++;
            auto command = m_activeStack->queue()[movingIndex];
            connect(command.data(), &UndoRedoCommand::finished, this, &UndoRedoSystem::executeNextCommandRedo,
                    Qt::SingleShotConnection);
            m_setIndexCommandQueue.enqueue(command);
        }

        // execute the first command
        executeNextCommandRedo();
    }
}

void UndoRedoSystem::executeNextCommandUndo()
{
    if (m_setIndexCommandQueue.isEmpty())
    {
        m_readyAfterSettingIndex = true;
    }
    // if there are commands in the queue, undo the next one
    else
    {
        qInfo() << "UndoRedoSystem::executeNextCommandUndo";
        auto command = m_setIndexCommandQueue.dequeue();
        command->asyncUndo();
        m_activeStack->decrementCurrentIndex();
        qInfo() << m_activeStack->currentIndex();
    }
}

void UndoRedoSystem::executeNextCommandRedo()
{
    if (m_setIndexCommandQueue.isEmpty())
    {
        m_readyAfterSettingIndex = true;
    }
    // if there are commands in the queue, redo the next one
    else
    {
        qInfo() << "UndoRedoSystem::executeNextCommandRedo";
        auto command = m_setIndexCommandQueue.dequeue();
        command->asyncRedo();
        m_activeStack->incrementCurrentIndex();
        qInfo() << m_activeStack->currentIndex();
    }
}

void UndoRedoSystem::setActiveStack(const QUuid &stackId)
{
    // if stackId is not in the stack hash, create one
    if (!m_stackHash.contains(stackId))
    {
        m_stackHash.insert(stackId, QSharedPointer<UndoRedoStack>(new UndoRedoStack(this, stackId)));
    }

    if (m_activeStack->id() != stackId)
    {
        m_activeStack = m_stackHash.value(stackId);

        // to reinit the undo redo actions
        emit stateChanged();
    }
}

QUuid UndoRedoSystem::activeStackId() const
{
    return m_activeStack->id();
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

    for (auto stack : m_stackHash.values())
    {

        for (const auto &command : stack->queue())
        {
            if (command->isRunning())
            {
                return true;
            }
        }
    }

    if (!m_readyAfterSettingIndex)
    {
        return true;
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

void UndoRedoSystem::onCommandUndoFinished(bool isSuccessful)
{
    auto *senderObject = QObject::sender();
    auto *command = dynamic_cast<UndoRedoCommand *>(senderObject);
    if (command == nullptr)
    {
        qFatal("Command pointer is null!");
    }
    const Scope &commandScope = command->scope();
    const QUuid &stackId = command->stackId();
    auto stack = m_stackHash.value(stackId, QSharedPointer<UndoRedoStack>());

    // Remove the command from the current command hash
    for (const ScopeFlag &scopeFlag : commandScope.flags())
    {

        m_currentCommandHash.insert(scopeFlag, QSharedPointer<UndoRedoCommand>());
    }

    if (!isSuccessful)
    {
        // Remove any redo commands that are after the current index
        stack->queue().erase(stack->queue().begin() + stack->currentIndex(), stack->queue().end());
    }

    // Emit the stateChanged signal
    emit stateChanged();
}

void UndoRedoSystem::onCommandRedoFinished(bool isSuccessful)
{
    auto *senderObject = QObject::sender();
    auto *command = dynamic_cast<UndoRedoCommand *>(senderObject);
    if (command == nullptr)
    {
        qFatal("Command pointer is null!");
    }
    const Scope &commandScope = command->scope();
    const QUuid &stackId = command->stackId();
    auto stack = m_stackHash.value(stackId, QSharedPointer<UndoRedoStack>());

    // Remove the command from the current command hash
    for (const ScopeFlag &scopeFlag : commandScope.flags())
    {

        m_currentCommandHash.insert(scopeFlag, QSharedPointer<UndoRedoCommand>());
    }

    if (!isSuccessful)
    {
        // Remove any redo commands that are after the current index
        stack->queue().erase(stack->queue().begin() + stack->currentIndex() + 1, stack->queue().end());
    }

    // Emit the stateChanged signal
    emit stateChanged();
}

/*!
 * \brief Handles the finished() signal from an UndoRedoCommand when it's doing a task after being pushed and updates
 * the UndoRedoSystem state.
 */
void UndoRedoSystem::onCommandDoFinished(bool isSuccessful)
{
    auto *senderObject = QObject::sender();
    auto *command = dynamic_cast<UndoRedoCommand *>(senderObject);
    if (command == nullptr)
    {
        qFatal("Command pointer is null!");
    }
    const Scope &commandScope = command->scope();
    const QUuid &stackId = command->stackId();
    auto stack = m_stackHash.value(stackId, QSharedPointer<UndoRedoStack>());

    // Remove the command from the current command hash
    for (const ScopeFlag &scopeFlag : commandScope.flags())
    {

        m_currentCommandHash.insert(scopeFlag, QSharedPointer<UndoRedoCommand>());
    }

    // If the command is a query command, remove it from the general command queue
    if (nullptr != qobject_cast<QueryCommand *>(QObject::sender()) || command->obsolete())
    {
        stack->queue().removeLast();
        stack->decrementCurrentIndex();
    }

    // If there are commands in the queue, execute the next one
    if (isSuccessful)
    {
        for (ScopeFlag scopeFlag : commandScope.flags())
        {
            if (!m_scopedCommandQueueHash[scopeFlag].isEmpty())
            {
                executeNextCommandDo(scopeFlag, stackId);
            }
        }
    }
    else // if execution was unsuccessful, clear the command queue
    {
        // empties the command scope queue
        for (ScopeFlag scopeFlag : commandScope.flags())
        {
            m_scopedCommandQueueHash[scopeFlag].clear();
        }
    }

    // Emit the stateChanged signal
    emit stateChanged();
}

/*!
 * \brief Executes the next command in the queue for the specified \a scope.
 */
void UndoRedoSystem::executeNextCommandDo(const ScopeFlag &scopeFlag, const QUuid &stackId)
{
    // Dequeue the next command
    QQueue<QSharedPointer<UndoRedoCommand>> &queue = m_scopedCommandQueueHash[scopeFlag];
    QSharedPointer<UndoRedoCommand> command = queue.dequeue();

    m_currentCommandHash.insert(scopeFlag, command);
    bool allowedToRun = isCommandAllowedToRun(command, scopeFlag);

    // get stack
    QSharedPointer<UndoRedoStack> stack = m_stackHash.value(stackId);

    // if not already running (from another scope)
    if (allowedToRun)
    {
        // Connect the finished signal to the onCommandDoFinished slot,  single shot connection to avoid unwanted signal
        connect(command.data(), &UndoRedoCommand::finished, this, &UndoRedoSystem::onCommandDoFinished,
                Qt::SingleShotConnection);

        // Connect the errorSent signal to the errorSent signal
        connect(command.data(), &UndoRedoCommand::errorSent, this, &UndoRedoSystem::errorSent, Qt::UniqueConnection);

        // Execute the command asynchronously
        command->asyncRedo();


        // merge with previous command if possible
        if (!stack->queue().isEmpty() && stack->currentIndex() > 0)
        {
            QSharedPointer<UndoRedoCommand> previousCommand = stack->queue()[stack->currentIndex() - 1];
            if (previousCommand->mergeWith(command.data()))
            {
                previousCommand->setObsolete(true);
                stack->queue().removeLast();
                stack->decrementCurrentIndex();
            }
        }

        // connecting after the initial asyncRedo to ovoid unwanted redoing signal when the command is added to the
        // queue
        connect(command.data(), &UndoRedoCommand::undoing, this, &UndoRedoSystem::undoing, Qt::UniqueConnection);
        connect(command.data(), &UndoRedoCommand::redoing, this, &UndoRedoSystem::redoing, Qt::UniqueConnection);
    }

    // keep in general command queue only the true AlterCommands, not QueryCommand, and only if it's allowed to run.
    // If the command is a system command, it mustn't be added to the general command queue
    if (qSharedPointerDynamicCast<QueryCommand>(command).isNull() && allowedToRun && !command->isSystem())
    {

        // Remove any redo commands that are after the current index
        stack->queue().erase(stack->queue().begin() + stack->currentIndex() + 1, stack->queue().end());

        // Add the new command to the end of the list if not already there:

        if (!stack->queue().contains(command))
        {
            stack->queue().enqueue(command);
            // Increment the current index
            stack->incrementCurrentIndex();

            // Remove excess commands from the general command queue if necessary
            while (stack->queue().size() > m_undoLimit)
            {
                stack->queue().dequeue();
                stack->decrementCurrentIndex();
            }
        }
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

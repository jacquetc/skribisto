#include "threaded_undo_redo_system.h"

using namespace Presenter::UndoRedo;

ThreadedUndoRedoSystem *ThreadedUndoRedoSystem::m_instance = nullptr;

/*!
 * \class ThreadedUndoRedoSystem
 * \inmodule Presenter::UndoRedo
 * \brief A QObject encapsulating a threaded UndoRedoSystem that manages the undo and redo command history
 * asynchronously.
 *
 * Implements a threaded undo-redo system by encapsulating the UndoRedoSystem functionality in a
 * separate thread. This class ensures that undo and redo operations are performed asynchronously without blocking the
 * main thread.
 */

/*!
 * \brief Constructs an ThreadedUndoRedoSystem with the specified \a parent.
 */
ThreadedUndoRedoSystem::ThreadedUndoRedoSystem(QObject *parent) : QObject(parent)
{
    QMutexLocker locker(&m_mutex);
    // singleton
    if (m_instance)
        return;
    m_instance = this;

    // Create a new UndoRedoSystem instance
    m_undoRedoSystem = new UndoRedoSystem();

    // Move the UndoRedoSystem to a new thread
    m_thread = new QThread(this);
    m_undoRedoSystem->moveToThread(m_thread);

    // Connect the thread started signal to the startUndoRedoSystem slot
    connect(m_thread, &QThread::started, this, &ThreadedUndoRedoSystem::startUndoRedoSystem);

    // Start the thread
    m_thread->start();
}

ThreadedUndoRedoSystem::~ThreadedUndoRedoSystem()
{
    QMutexLocker locker(&m_mutex);
    // Stop the thread
    m_thread->quit();
    m_thread->wait();

    // Delete the UndoRedoSystem instance and the thread
    m_undoRedoSystem->deleteLater();
    // m_thread->deleteLater();
}

/*!
 * \brief Returns the singleton instance of ThreadedUndoRedoSystem.
 * \note If no instance exists, a fatal error occurs.
 */
ThreadedUndoRedoSystem *ThreadedUndoRedoSystem::instance()
{
    if (!m_instance)
        qFatal("No ThreadedUndoRedoSystem instance found");
    return m_instance;
}

/*!
 * \brief Starts the UndoRedoSystem when the encapsulated thread is started.
 */
void ThreadedUndoRedoSystem::startUndoRedoSystem()
{
    QMutexLocker locker(&m_mutex);
    // Connect the UndoRedoSystem's stateChanged signal to this class's stateChanged signal
    connect(m_undoRedoSystem, &UndoRedoSystem::stateChanged, this,
            &ThreadedUndoRedoSystem::onUndoRedoSystemStateChanged);
    connect(m_undoRedoSystem, &UndoRedoSystem::errorSent, this, &ThreadedUndoRedoSystem::onErrorSent);

    QMetaObject::invokeMethod(m_undoRedoSystem, "run", Qt::QueuedConnection);
}

/*!
 * \brief Returns true if an undo operation can be performed, otherwise false.
 */
bool ThreadedUndoRedoSystem::canUndo() const
{
    QMutexLocker locker(&m_mutex);
    bool result = false;
    QMetaObject::invokeMethod(m_undoRedoSystem, "canUndo", Qt::QueuedConnection, Q_RETURN_ARG(bool, result));
    return result;
}

/*!
 * \brief Returns true if a redo operation can be performed, otherwise false.
 */
bool ThreadedUndoRedoSystem::canRedo() const
{
    QMutexLocker locker(&m_mutex);
    bool result = false;
    QMetaObject::invokeMethod(m_undoRedoSystem, "canRedo", Qt::QueuedConnection, Q_RETURN_ARG(bool, result));
    return result;
}

/*!
 * \brief Performs an undo operation.
 */
void ThreadedUndoRedoSystem::undo()
{
    QMutexLocker locker(&m_mutex);
    QMetaObject::invokeMethod(m_undoRedoSystem, "undo", Qt::QueuedConnection);
}

/*!
 * \brief Performs a redo operation.
 */
void ThreadedUndoRedoSystem::redo()
{
    QMutexLocker locker(&m_mutex);
    QMetaObject::invokeMethod(m_undoRedoSystem, "redo", Qt::QueuedConnection);
}

/*!
 * \brief Adds a new command to the command history with the specified \a scope.
 */
void ThreadedUndoRedoSystem::push(UndoRedoCommand *command, const UndoRedoCommand::Scope &scope)
{
    QMutexLocker locker(&m_mutex);
    command->moveToThread(m_undoRedoSystem->thread());
    QMetaObject::invokeMethod(m_undoRedoSystem, "push", Qt::QueuedConnection,
                              Q_ARG(Presenter::UndoRedo::UndoRedoCommand *, command),
                              Q_ARG(Presenter::UndoRedo::UndoRedoCommand::Scope, scope));
}

/*!
 * \brief Clears the command history.
 */
void ThreadedUndoRedoSystem::clear()
{

    QMutexLocker locker(&m_mutex);
    QMetaObject::invokeMethod(m_undoRedoSystem, "clear", Qt::QueuedConnection);
}

/*!
 * \brief Handles the UndoRedoSystem stateChanged signal and emits the stateChanged signal for this class.
 */
void ThreadedUndoRedoSystem::onUndoRedoSystemStateChanged()
{
    QMutexLocker locker(&m_mutex);
    // Emit the stateChanged signal
    emit stateChanged();
}

void ThreadedUndoRedoSystem::onErrorSent(const Error &error)
{

    QMutexLocker locker(&m_mutex);
    // Emit the stateChanged signal
    emit errorSent(error);
}

/*!
 * \brief Sets the undo limit to \a limit.
 */
void ThreadedUndoRedoSystem::setUndoLimit(int limit)
{
    QMutexLocker locker(&m_mutex);
    QMetaObject::invokeMethod(m_undoRedoSystem, "setUndoLimit", Qt::QueuedConnection, Q_ARG(int, limit));
}

/*!
 * \brief Returns the undo limit.
 */
int ThreadedUndoRedoSystem::undoLimit() const
{
    QMutexLocker locker(&m_mutex);
    int result = 0;
    QMetaObject::invokeMethod(m_undoRedoSystem, "undoLimit", Qt::QueuedConnection, Q_RETURN_ARG(int, result));
    return result;
}

/*!
 * \brief Returns the text of the last undo command.
 */
QString ThreadedUndoRedoSystem::undoText() const
{
    QMutexLocker locker(&m_mutex);
    QString result;
    QMetaObject::invokeMethod(m_undoRedoSystem, "undoText", Qt::QueuedConnection, Q_RETURN_ARG(QString, result));
    return result;
}

/*!
 * \brief Returns the text of the last redo command.
 */
QString ThreadedUndoRedoSystem::redoText() const
{
    QMutexLocker locker(&m_mutex);
    QString result;
    QMetaObject::invokeMethod(m_undoRedoSystem, "redoText", Qt::QueuedConnection, Q_RETURN_ARG(QString, result));
    return result;
}

/*!
 * \brief Creates and returns a redo QAction with the specified \a parent and \a prefix.
 */
QAction *ThreadedUndoRedoSystem::createRedoAction(QObject *parent, const QString &prefix) const
{
    QMutexLocker locker(&m_mutex);
    QAction *action = new QAction(parent);
    bool canRedo = false;
    QMetaObject::invokeMethod(m_undoRedoSystem, "canRedo", Qt::BlockingQueuedConnection, Q_RETURN_ARG(bool, canRedo));
    QString text;
    if (canRedo)
    {
        QMetaObject::invokeMethod(m_undoRedoSystem, "redoText", Qt::BlockingQueuedConnection,
                                  Q_RETURN_ARG(QString, text));
        if (!text.isEmpty())
        {
            text = prefix.isEmpty() ? tr("Redo %1").arg(text) : prefix.arg(text);
            action->setText(text);
        }
    }

    connect(this, &ThreadedUndoRedoSystem::stateChanged, action, [action, this, prefix]() {
        QString text;
        QMetaObject::invokeMethod(m_undoRedoSystem, "redoText", Qt::BlockingQueuedConnection,
                                  Q_RETURN_ARG(QString, text));
        action->setText(prefix.isEmpty() ? tr("Redo %1").arg(text) : prefix.arg(text));
        bool canRedo = false;
        QMetaObject::invokeMethod(m_undoRedoSystem, "canRedo", Qt::BlockingQueuedConnection,
                                  Q_RETURN_ARG(bool, canRedo));
        action->setEnabled(canRedo);
    });
    action->setEnabled(canRedo);
    action->setShortcut(QKeySequence::Redo);

    connect(action, &QAction::triggered, this, &ThreadedUndoRedoSystem::redo);
    return action;
}

/*!
 * \brief Creates and returns an undo QAction with the specified \a parent and \a prefix.
 */
QAction *ThreadedUndoRedoSystem::createUndoAction(QObject *parent, const QString &prefix) const
{
    QMutexLocker locker(&m_mutex);
    QAction *action = new QAction(parent);
    bool canUndo = false;
    QMetaObject::invokeMethod(m_undoRedoSystem, "canUndo", Qt::BlockingQueuedConnection, Q_RETURN_ARG(bool, canUndo));
    QString text;
    if (canUndo)
    {
        QMetaObject::invokeMethod(m_undoRedoSystem, "undoText", Qt::BlockingQueuedConnection,
                                  Q_RETURN_ARG(QString, text));
        if (!text.isEmpty())
        {
            text = prefix.isEmpty() ? tr("Undo %1").arg(text) : prefix.arg(text);

            action->setText(text);
        }
    }

    connect(this, &ThreadedUndoRedoSystem::stateChanged, action, [action, this, prefix]() {
        QString text;
        QMetaObject::invokeMethod(m_undoRedoSystem, "undoText", Qt::BlockingQueuedConnection,
                                  Q_RETURN_ARG(QString, text));
        action->setText(prefix.isEmpty() ? tr("Undo %1").arg(text) : prefix.arg(text));
        bool canUndo = false;
        QMetaObject::invokeMethod(m_undoRedoSystem, "canUndo", Qt::BlockingQueuedConnection,
                                  Q_RETURN_ARG(bool, canUndo));
        action->setEnabled(canUndo);
    });
    action->setEnabled(canUndo);
    action->setShortcut(QKeySequence::Undo);
    connect(action, &QAction::triggered, this, &ThreadedUndoRedoSystem::undo);
    return action;
}

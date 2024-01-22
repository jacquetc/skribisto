// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "undo_redo_controller.h"

#include <QCoroSignal>

using namespace Skribisto::Controller;
using namespace Skribisto::Controller::UndoRedo;
using namespace Qleany::Tools::UndoRedo;

QPointer<UndoRedoController> UndoRedoController::s_instance = nullptr;

UndoRedoController::UndoRedoController(ThreadedUndoRedoSystem *undo_redo_system,
                                       QSharedPointer<EventDispatcher> eventDispatcher)
    : QObject{nullptr}
{

    // connections for undo commands:
    m_undo_redo_system = undo_redo_system;
    m_eventDispatcher = eventDispatcher;

    auto *undoRedoSignals = m_eventDispatcher->undoRedo();

    connect(m_undo_redo_system, &ThreadedUndoRedoSystem::stateChanged, undoRedoSignals, &UndoRedoSignals::stateChanged);
    connect(m_undo_redo_system, &ThreadedUndoRedoSystem::redoing, undoRedoSignals, &UndoRedoSignals::redoing);
    connect(m_undo_redo_system, &ThreadedUndoRedoSystem::undoing, undoRedoSignals, &UndoRedoSignals::undoing);

    s_instance = this;
}

UndoRedoController *UndoRedoController::instance()
{
    return s_instance.data();
}

bool UndoRedoController::canUndo() const
{
    return m_undo_redo_system->canUndo();
}

bool UndoRedoController::canRedo() const
{
    return m_undo_redo_system->canRedo();
}

void UndoRedoController::setUndoLimit(int limit)
{
    m_undo_redo_system->setUndoLimit(limit);
}

int UndoRedoController::undoLimit() const
{
    return m_undo_redo_system->undoLimit();
}

QString UndoRedoController::undoText() const
{
    return m_undo_redo_system->undoText();
}

QString UndoRedoController::redoText() const
{
    return m_undo_redo_system->redoText();
}

QStringList UndoRedoController::undoRedoTextList() const
{
    return m_undo_redo_system->undoRedoTextList();
}

int UndoRedoController::currentIndex() const
{
    return m_undo_redo_system->currentIndex();
}

QUuid UndoRedoController::activeStackId() const
{
    return m_undo_redo_system->activeStackId();
}

QStringList UndoRedoController::queuedCommandTextListByScope(const QString &scopeFlagString) const
{
    return m_undo_redo_system->queuedCommandTextListByScope(scopeFlagString);
}

bool UndoRedoController::isRunning() const
{
    return m_undo_redo_system->isRunning();
}

int UndoRedoController::numberOfCommands() const
{
    return m_undo_redo_system->numberOfCommands();
}

QAction *UndoRedoController::createUndoAction(QObject *parent, const QString &prefix) const
{
    return m_undo_redo_system->createUndoAction(parent, prefix);
}

QAction *UndoRedoController::createRedoAction(QObject *parent, const QString &prefix) const
{
    return m_undo_redo_system->createRedoAction(parent, prefix);
}

void UndoRedoController::undo()
{
    return m_undo_redo_system->undo();
}

void UndoRedoController::redo()
{
    return m_undo_redo_system->redo();
}

void UndoRedoController::clear()
{
    return m_undo_redo_system->clear();
}

void UndoRedoController::setCurrentIndex(int index)
{
    return m_undo_redo_system->setCurrentIndex(index);
}

void UndoRedoController::setActiveStack(const QUuid &stackId)
{
    return m_undo_redo_system->setActiveStack(stackId);
}
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_undo.h"
#include "event_dispatcher.h"
#include "undo_redo/undo_redo_controller.h"

using namespace Skribisto::Controller;
using namespace Skribisto::Controller::UndoRedo;
using namespace Skribisto::Presenter;

SingleUndo::SingleUndo(QObject *parent) : QObject{parent}
{
    m_action = UndoRedoController::instance()->createUndoAction(this, tr("Undo: %1"));

    m_enabled = m_action->isEnabled();
    connect(m_action, &QAction::enabledChanged, this, [this](bool newEnabled) {
        if (m_enabled == newEnabled)
            return;
        m_enabled = newEnabled;
        emit enabledChanged();
    });

    m_text = m_action->text();
    connect(m_action, &QAction::changed, this, [this]() {
        const QString &newText = m_action->text();
        if (m_text == newText)
            return;
        m_text = newText;
        emit textChanged();
    });

    connect(EventDispatcher::instance()->undoRedo(), &UndoRedoSignals::undoing, this, [this](Scope scope, bool active) {
        if (m_enabled == active)
            return;
        m_enabled = active;
        emit enabledChanged();
    });
}

bool SingleUndo::enabled() const
{
    return m_enabled;
}

QString SingleUndo::text() const
{
    return m_text;
}

void SingleUndo::undo()
{
    UndoRedoController::instance()->undo();
}
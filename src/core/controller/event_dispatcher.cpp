#include "event_dispatcher.h"

using namespace Skribisto::Controller;

QPointer<EventDispatcher> EventDispatcher::s_instance = nullptr;

EventDispatcher::EventDispatcher() : QObject{nullptr}
{
    m_errorSignals = new ErrorSignals(this);
    m_progressSignals = new ProgressSignals(this);

    m_undoRedoSignals = new UndoRedoSignals(this);

    m_userSignals = new UserSignals(this);
    m_bookSignals = new BookSignals(this);
    m_workspaceSignals = new WorkspaceSignals(this);
    m_atelierSignals = new AtelierSignals(this);
    m_chapterSignals = new ChapterSignals(this);
    m_sceneSignals = new SceneSignals(this);
    m_sceneParagraphSignals = new SceneParagraphSignals(this);
    m_systemSignals = new SystemSignals(this);

    s_instance = this;
}

EventDispatcher *EventDispatcher::instance()
{
    return s_instance;
}

UserSignals *EventDispatcher::user() const
{
    return m_userSignals;
}

BookSignals *EventDispatcher::book() const
{
    return m_bookSignals;
}

WorkspaceSignals *EventDispatcher::workspace() const
{
    return m_workspaceSignals;
}

AtelierSignals *EventDispatcher::atelier() const
{
    return m_atelierSignals;
}

ChapterSignals *EventDispatcher::chapter() const
{
    return m_chapterSignals;
}

SceneSignals *EventDispatcher::scene() const
{
    return m_sceneSignals;
}

SceneParagraphSignals *EventDispatcher::sceneParagraph() const
{
    return m_sceneParagraphSignals;
}

SystemSignals *EventDispatcher::system() const
{
    return m_systemSignals;
}

ErrorSignals *EventDispatcher::error() const
{
    return m_errorSignals;
}

ProgressSignals *EventDispatcher::progress() const
{
    return m_progressSignals;
}

UndoRedoSignals *EventDispatcher::undoRedo() const
{
    return m_undoRedoSignals;
}

#include "event_dispatcher.h"

using namespace Presenter;

QScopedPointer<EventDispatcher> EventDispatcher::s_instance = QScopedPointer<EventDispatcher>(nullptr);

EventDispatcher::EventDispatcher(QObject *parent) : QObject{parent}
{
    m_chapterSignals = new ChapterSignals(this);
    m_sceneSignals = new SceneSignals(this);
    m_sceneParagraphSignals = new SceneParagraphSignals(this);

    s_instance.reset(this);
}

EventDispatcher *EventDispatcher::instance()
{
    return s_instance.data();
}

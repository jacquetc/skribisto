#pragma once

#include "entities/chapter_signals.h"
#include "entities/scene_paragraph_signals.h"
#include "entities/scene_signals.h"
#include <QObject>

using namespace Presenter::Entities;

namespace Presenter
{

class EventDispatcher : public QObject
{
    Q_OBJECT
  public:
    explicit EventDispatcher(QObject *parent = nullptr);
    static EventDispatcher *instance();

    ChapterSignals *chapter() const;
    SceneSignals *scene() const;
    SceneParagraphSignals *sceneParagraph() const;

  private:
    static QScopedPointer<EventDispatcher> s_instance;
    ChapterSignals *m_chapterSignals;
    SceneSignals *m_sceneSignals;
    SceneParagraphSignals *m_sceneParagraphSignals;

    EventDispatcher() = delete;
    EventDispatcher(const EventDispatcher &) = delete;
    EventDispatcher &operator=(const EventDispatcher &) = delete;
};

} // namespace Presenter

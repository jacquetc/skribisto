#pragma once

#include "controller_export.h"
#include "error_signals.h"
#include "progress_signals.h"

#include "undo_redo/undo_redo_signals.h"

#include "user/user_signals.h"

#include "book/book_signals.h"

#include "workspace/workspace_signals.h"

#include "atelier/atelier_signals.h"

#include "chapter/chapter_signals.h"

#include "scene/scene_signals.h"

#include "scene_paragraph/scene_paragraph_signals.h"

#include "system/system_signals.h"

#include <QObject>
#include <QPointer>

namespace Skribisto::Controller
{
class SKRIBISTO_CONTROLLER_EXPORT EventDispatcher : public QObject
{
    Q_OBJECT
  public:
    explicit EventDispatcher();
    static EventDispatcher *instance();

    Q_INVOKABLE ErrorSignals *error() const;
    Q_INVOKABLE ProgressSignals *progress() const;

    Q_INVOKABLE UndoRedoSignals *undoRedo() const;

    Q_INVOKABLE UserSignals *user() const;

    Q_INVOKABLE BookSignals *book() const;

    Q_INVOKABLE WorkspaceSignals *workspace() const;

    Q_INVOKABLE AtelierSignals *atelier() const;

    Q_INVOKABLE ChapterSignals *chapter() const;

    Q_INVOKABLE SceneSignals *scene() const;

    Q_INVOKABLE SceneParagraphSignals *sceneParagraph() const;

    Q_INVOKABLE SystemSignals *system() const;

  private:
    static QPointer<EventDispatcher> s_instance;
    ErrorSignals *m_errorSignals;
    ProgressSignals *m_progressSignals;

    UndoRedoSignals *m_undoRedoSignals;

    UserSignals *m_userSignals;

    BookSignals *m_bookSignals;

    WorkspaceSignals *m_workspaceSignals;

    AtelierSignals *m_atelierSignals;

    ChapterSignals *m_chapterSignals;

    SceneSignals *m_sceneSignals;

    SceneParagraphSignals *m_sceneParagraphSignals;

    SystemSignals *m_systemSignals;

    EventDispatcher(const EventDispatcher &) = delete;
    EventDispatcher &operator=(const EventDispatcher &) = delete;
};
} // namespace Skribisto::Controller
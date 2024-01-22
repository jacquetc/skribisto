// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"
#include "event_dispatcher.h"

#include <QAction>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Tools::UndoRedo;

namespace Skribisto::Controller::UndoRedo
{

class SKRIBISTO_CONTROLLER_EXPORT UndoRedoController : public QObject
{
    Q_OBJECT
  public:
    explicit UndoRedoController(ThreadedUndoRedoSystem *undo_redo_system,
                                QSharedPointer<EventDispatcher> eventDispatcher);

    static UndoRedoController *instance();

    Q_INVOKABLE bool canUndo() const;

    Q_INVOKABLE bool canRedo() const;

    Q_INVOKABLE void setUndoLimit(int limit);

    Q_INVOKABLE int undoLimit() const;

    Q_INVOKABLE QString undoText() const;

    Q_INVOKABLE QString redoText() const;

    Q_INVOKABLE QStringList undoRedoTextList() const;

    Q_INVOKABLE int currentIndex() const;

    Q_INVOKABLE QUuid activeStackId() const;

    Q_INVOKABLE QStringList queuedCommandTextListByScope(const QString &scopeFlagString) const;

    Q_INVOKABLE bool isRunning() const;

    Q_INVOKABLE int numberOfCommands() const;

    QAction *createUndoAction(QObject *parent, const QString &prefix = QString()) const;
    QAction *createRedoAction(QObject *parent, const QString &prefix = QString()) const;

  public slots:

    void undo();
    void redo();
    void clear();
    void setCurrentIndex(int index);
    void setActiveStack(const QUuid &stackId = QUuid());

  private:
    static QPointer<UndoRedoController> s_instance;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    UndoRedoController() = delete;
    UndoRedoController(const UndoRedoController &) = delete;
    UndoRedoController &operator=(const UndoRedoController &) = delete;
};

} // namespace Skribisto::Controller::UndoRedo
#pragma once

#include "presenter_global.h"
#include "undo_redo_system.h"
#include <QMutex>
#include <QObject>
#include <QThread>

namespace Presenter::UndoRedo
{
class SKR_PRESENTER_EXPORT ThreadedUndoRedoSystem : public QObject
{
    Q_OBJECT
  public:
    ThreadedUndoRedoSystem(QObject *parent, const Scopes &scopes);

    ~ThreadedUndoRedoSystem();

    static ThreadedUndoRedoSystem *instance();

    bool canUndo() const;

    bool canRedo() const;

    void undo();

    void redo();

    void push(UndoRedoCommand *command, const QString &commandScope, const QUuid &stackId = QUuid());

    void clear();

    void setUndoLimit(int limit);

    int undoLimit() const;

    QString undoText() const;

    QString redoText() const;

    QStringList undoRedoTextList() const;

    int currentIndex() const;

    void setCurrentIndex(int index, const QUuid &stackId = QUuid());

    void setActiveStack(const QUuid &stackId = QUuid());

    QUuid activeStackId() const;

    QAction *createUndoAction(QObject *parent, const QString &prefix = QString()) const;
    QAction *createRedoAction(QObject *parent, const QString &prefix = QString()) const;
  signals:
    /*!
     * \brief A signal that is emitted when the undo redo system state has changed. Useful for the undo and redo
     * actions.
     */
    void stateChanged();
    /*!
     * \brief A signal that is emitted when a command results in an error.
     * actions.
     */
    void errorSent(Error error);
    /*!
     * \brief A signal that is emitted when the undo redo system is about to start redoing.
     * actions.
     */
    void redoing(Scope scope, bool active);
    /*!
     * \brief A signal that is emitted when the undo redo system is about to start undoing.
     * actions.
     */
    void undoing(Scope scope, bool active);

  private slots:
    void startUndoRedoSystem();

    void onUndoRedoSystemStateChanged();
    void onErrorSent(const Error &error);

  private:
    static ThreadedUndoRedoSystem *m_instance;
    mutable QMutex m_mutex;
    UndoRedoSystem *m_undoRedoSystem = nullptr;
    QThread *m_thread = nullptr;
};
} // namespace Presenter::UndoRedo

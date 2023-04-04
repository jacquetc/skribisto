#pragma once

#include "result.h"
#include <QFutureWatcher>
#include <QObject>
#include <QPromise>

namespace Presenter::UndoRedo
{
class UndoRedoCommand : public QObject
{
    Q_OBJECT
  public:
    enum Scope
    {
        Author,
        All
    };
    Q_ENUM(Scope)

    UndoRedoCommand(const QString &text);

    virtual Result<void> undo() = 0;

    virtual void redo(QPromise<Result<void>> &progressPromise) = 0;

    void asyncUndo();

    void asyncRedo();

    bool isRunning() const;

    UndoRedoCommand::Scope scope() const;

    void setScope(UndoRedoCommand::Scope newScope);

    QString text() const;

    void setText(const QString &newText);

    bool obsolete() const;

    void setObsolete(bool newObsolete);

  signals:
    void finished();
    /*!
     * \brief A signal that is emitted when a command results in an error.
     * actions.
     */
    void errorSent(Error error);
    void progressRangeChanged(int minimum, int maximum);
    void progressTextChanged(const QString &progressText);
    void progressValueChanged(int progressValue);

  private slots:
    void onFinished();

  private:
    QFutureWatcher<Result<void>> *m_watcher;
    bool m_obsolete;                /*!< A boolean representing the obsolete state of the command. */
    bool m_finished;                /*!< A boolean representing the finished state of the command. */
    bool m_running;                 /*!< A boolean representing the running state of the command. */
    QString m_text;                 /*!< A QString representing the text description of the command. */
    UndoRedoCommand::Scope m_scope; /*!< The command's scope as an UndoRedoCommand::Scope enumeration value. */
};
} // namespace Presenter::UndoRedo
Q_DECLARE_METATYPE(Presenter::UndoRedo::UndoRedoCommand::Scope)

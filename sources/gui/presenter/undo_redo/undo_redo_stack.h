#pragma once

#include "undo_redo_command.h"

#include <QObject>
#include <QQueue>
#include <QSharedPointer>

namespace Presenter::UndoRedo
{

class UndoRedoStack : public QObject
{
    Q_OBJECT
  public:
    explicit UndoRedoStack(QObject *parent = nullptr);
    explicit UndoRedoStack(QObject *parent, const QUuid &id);

    QQueue<QSharedPointer<UndoRedoCommand>> &queue();
    void setQueue(const QQueue<QSharedPointer<UndoRedoCommand>> &newQueue);

    QUuid id() const;
    void setId(const QUuid &newId);

    int currentIndex() const;
    void setCurrentIndex(int newCurrentIndex);
    void incrementCurrentIndex();
    void decrementCurrentIndex();

  signals:

  private:
    QUuid m_id;
    QQueue<QSharedPointer<UndoRedoCommand>> m_queue;
    int m_currentIndex = -1;
};
} // namespace Presenter::UndoRedo

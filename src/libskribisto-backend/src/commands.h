#ifndef COMMANDS_H
#define COMMANDS_H

#include <QHash>
#include <QObject>
#include <QUndoCommand>
#include "skribisto_backend_global.h"
#include "interfaces/invokablecommandgroupinterface.h"

#define commands Commands::instance()


class SKRBACKENDEXPORT Commands : public QObject
{
    Q_OBJECT
public:
    explicit Commands(QObject *parent, QUndoStack *undoStack);

    static Commands* instance()
    {
        return m_instance;
    }
    QUndoStack *undoStack() const;

    void subscribe(InvokableCommandGroupInterface *object);

    void invoke(const QString &address, const QString &action, const QVariantMap &parameters);

signals:

private:
    static Commands *m_instance;
    QUndoStack *m_undoStack;
    QHash<QString, InvokableCommandGroupInterface *> m_addressWithObjectHash;
};


#endif // COMMANDS_H

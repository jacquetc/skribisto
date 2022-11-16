#include "commands.h"

Commands::Commands(QObject *parent, QUndoStack *undoStack)
    : QObject{parent}, m_undoStack(undoStack)
{
    m_instance = this;

}

QUndoStack *Commands::undoStack() const
{
return m_undoStack;

}

void Commands::subscribe(InvokableCommandGroupInterface *object)
{

    m_addressWithObjectHash.insert(object->address() , object);
}

void Commands::invoke(const QString &address, const QString &action, const QVariantMap &parameters)
{
    if(!m_addressWithObjectHash.contains(address)){
        return;
    }

    InvokableCommandGroupInterface *commandGroup = m_addressWithObjectHash.value(address);

    Command *command = commandGroup->getCommand(action, parameters);

    if(command){
        m_undoStack->push(command);
    }

}

Commands *Commands::m_instance = nullptr;


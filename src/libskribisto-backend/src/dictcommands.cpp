#include "dictcommands.h"
#include "commands.h"
#include "skrdata.h"

DictCommands::DictCommands(QObject *parent, QUndoStack *undoStack)
    : QObject{parent}, m_undoStack(undoStack)
{
    m_instance = this;
    commands->subscribe(this);
}
// ----------------------------------------------------------------------------


DictCommands *DictCommands::m_instance = nullptr;


// ----------------------------------------------------------------------------



QUndoStack *DictCommands::undoStack() const
{
    return m_undoStack;
}

// ----------------------------------------------------------------------------


void DictCommands::addWordToProjectDict(int projectId, const QString &word)
{
    m_undoStack->push(new AddWordToProjectDictCommand(projectId, word));

}

// ----------------------------------------------------------------------------



Command *DictCommands::getCommand(const QString &action, const QVariantMap &parameters)
{
//TODO: fill
}


//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

AddWordToProjectDictCommand::AddWordToProjectDictCommand(int projectId, const QString &word) : Command("Add word to project dictionary"),
    m_projectId(projectId), m_word(word)
{

}

void AddWordToProjectDictCommand::undo()
{
    skrdata->projectDictHub()->removeWordFromProjectDict(m_projectId, m_word);
}

void AddWordToProjectDictCommand::redo()
{
    skrdata->projectDictHub()->addWordToProjectDict(m_projectId, m_word);
}

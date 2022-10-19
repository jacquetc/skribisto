#include "tagcommands.h"
#include "skrresult.h"

#include <skrdata.h>

TagCommands::TagCommands(QObject *parent, QUndoStack *undoStack)
    : QObject{parent}, m_undoStack(undoStack)
{
    m_instance = this;

}
TagCommands *TagCommands::m_instance = nullptr;

QUndoStack *TagCommands::undoStack() const
{
return m_undoStack;
}

int TagCommands::addTag(int projectId, const QString &name, const QColor &tagColor, const QColor &textColor)
{

    m_undoStack->beginMacro("Add tag " + name);

    auto *command = new AddTagCommand(projectId, name);
    m_undoStack->push(command);
    int tagId = command->result();

    setTagColor(projectId, tagId, tagColor);
    setTagTextColor(projectId, tagId, textColor);

    m_undoStack->endMacro();

    return tagId;
}


//---------------------------------------------------------------------------

int TagCommands::addTag(int projectId, const QString &name)
{
    auto *command = new AddTagCommand(projectId, name);
    m_undoStack->push(command);
    return command->result();
}

//---------------------------------------------------------------------------


void TagCommands::modifyTag(int projectId, int tagId, const QString &name, const QColor &tagColor, const QColor &textColor)
{
    m_undoStack->beginMacro("Modify tag " + name);

    setTagName(projectId, tagId, name);
    setTagColor(projectId, tagId, tagColor);
    setTagTextColor(projectId, tagId, textColor);

    m_undoStack->endMacro();
}

//---------------------------------------------------------------------------

QList<int> TagCommands::addSeveralTags(int projectId, QList<QString> nameList)
{
    QList<int> tagIdList;
    m_undoStack->beginMacro("Add several tags");

    for(const QString &name : nameList){

        auto *command = new AddTagCommand(projectId, name);
        m_undoStack->push(command);
        tagIdList.append(command->result());

    }

    m_undoStack->endMacro();


    return tagIdList;
}


//---------------------------------------------------------------------------

void TagCommands::setTagName(int projectId, int tagId, const QString &name)
{
    m_undoStack->push(new SetTagNameCommand(projectId, tagId, name));
}

//---------------------------------------------------------------------------


void TagCommands::setTagColor(int projectId, int tagId, const QColor &color)
{
    m_undoStack->push(new SetTagColorCommand(projectId, tagId, color));

}

void TagCommands::setTagTextColor(int projectId, int tagId, const QColor &color)
{
    m_undoStack->push(new SetTagTextColorCommand(projectId, tagId, color));

}

void TagCommands::removeTag(int projectId, int tagId)
{
    m_undoStack->push(new RemoveTagCommand(projectId, tagId));

}

void TagCommands::removeSeveralTags(int projectId, QList<int> tagIds)
{
    m_undoStack->beginMacro("Remove several tags");

    for(int tagId : tagIds){
        m_undoStack->push(new RemoveTagCommand(projectId, tagId));

    }

    m_undoStack->endMacro();

}

//-------------------------------------------------------------
//-------------------------------------------------------------
//-------------------------------------------------------------

AddTagCommand::AddTagCommand(int projectId, const QString &name) : Command("Add tag"),
   m_projectId(projectId), m_newId(-1), m_name(name)
{

}

void AddTagCommand::undo()
{
    skrdata->tagHub()->removeTag(m_projectId, m_newId);
}

void AddTagCommand::redo()
{

    SKRResult result =  skrdata->tagHub()->addTag(m_projectId, m_name);

    if(m_newId == -1){
        // store id
        m_newId = result.getData("tagId", -1).toInt();
    }
    else {
        // reapply id
        int temporaryId = result.getData("tagId", -1).toInt();
        if(temporaryId != m_newId){
            skrdata->tagHub()->setTagId(m_projectId, temporaryId, m_newId);
        }
    }
}

int AddTagCommand::result()
{
    return m_newId;
}
//-------------------------------------------------------------
//-------------------------------------------------------------
//-------------------------------------------------------------

SetTagNameCommand::SetTagNameCommand(int projectId, int tagId, const QString &name): Command("Set tag name to " + name),
    m_newName(name), m_projectId(projectId), m_tagId(tagId)
{
}

void SetTagNameCommand::undo()
{
    skrdata->tagHub()->setTagName(m_projectId, m_tagId, m_oldName);

}

void SetTagNameCommand::redo()
{
    m_oldName = skrdata->tagHub()->getTagName(m_projectId, m_tagId);
    skrdata->tagHub()->setTagName(m_projectId, m_tagId, m_newName);
}

//-------------------------------------------------------------
//-------------------------------------------------------------
//-------------------------------------------------------------

SetTagColorCommand::SetTagColorCommand(int projectId, int tagId, const QColor &color) : Command("Set tag color"),
    m_newColor(color), m_projectId(projectId), m_tagId(tagId)
{

}

void SetTagColorCommand::undo()
{
    skrdata->tagHub()->setTagColor(m_projectId, m_tagId, m_oldColor.name());

}

void SetTagColorCommand::redo()
{
    m_oldColor = QColor(skrdata->tagHub()->getTagColor(m_projectId, m_tagId));
    skrdata->tagHub()->setTagColor(m_projectId, m_tagId, m_newColor.name());

}

//-------------------------------------------------------------
//-------------------------------------------------------------
//-------------------------------------------------------------

SetTagTextColorCommand::SetTagTextColorCommand(int projectId, int tagId, const QColor &color) : Command("Set tag text color"),
    m_newColor(color), m_projectId(projectId), m_tagId(tagId)
{

}

void SetTagTextColorCommand::undo()
{
    skrdata->tagHub()->setTagTextColor(m_projectId, m_tagId, m_oldColor.name());

}

void SetTagTextColorCommand::redo()
{

    m_oldColor = QColor(skrdata->tagHub()->getTagTextColor(m_projectId, m_tagId));
    skrdata->tagHub()->setTagTextColor(m_projectId, m_tagId, m_newColor.name());

}

//-------------------------------------------------------------
//-------------------------------------------------------------
//-------------------------------------------------------------


RemoveTagCommand::RemoveTagCommand(int projectId, int tagId) : Command("Remove tag"),
    m_projectId(projectId), m_tagId(tagId)
{

}

void RemoveTagCommand::undo()
{
    SKRResult result =  skrdata->tagHub()->addTag(m_projectId);

    int temporaryId = result.getData("tagId", -1).toInt();
    if(temporaryId != m_tagId){
        skrdata->tagHub()->setTagId(m_projectId, temporaryId, m_tagId);
    }
    if(!m_savedTagValues.isEmpty()){
        skrdata->tagHub()->restoreId(m_projectId, m_tagId, m_savedTagValues);
    }

    for(int treeItemId : m_relatedTreeItemIds){
        skrdata->tagHub()->setTagRelationship(m_projectId, treeItemId, m_tagId);
    }
}

void RemoveTagCommand::redo()
{
    m_relatedTreeItemIds = skrdata->tagHub()->getItemIdsFromTag(m_projectId, m_tagId);
    m_savedTagValues = skrdata->tagHub()->saveId(m_projectId, m_tagId);

     skrdata->tagHub()->removeTag(m_projectId, m_tagId);

}


QString TagCommands::address() const
{
    return "tags";

}

Command *TagCommands::getCommand(const QString &action, const QVariantMap &parameters)
{
    if(action == "set_project_name"){
        return new SetTagNameCommand(parameters.value("projectId").toInt(), parameters.value("tagId").toInt(), parameters.value("name").toString());
    }

    return nullptr;
}

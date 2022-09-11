#include "projecttreecommands.h"

ProjectTreeCommands::ProjectTreeCommands(QObject *parent, QUndoStack *undoStack, ProjectTreeModel *treeModel)
    : QObject{parent}, m_undoStack(undoStack), m_treeModel(treeModel)
{
    m_instance = this;

}
ProjectTreeCommands *ProjectTreeCommands::m_instance = nullptr;

void ProjectTreeCommands::addItemAfter(int projectId, int targetId, const QString &type, const QVariantMap &properties)
{
    m_undoStack->push(new AddItemAfterCommand(projectId, targetId, type, properties, m_treeModel));
}

void ProjectTreeCommands::addSeveralItemsAfter(int projectId, int targetId, const QString &type, int count, const QVariantMap &properties)
{
    m_undoStack->beginMacro("Add several items after");

    for(int i = 0 ; i < count; i++){
        m_undoStack->push(new AddItemAfterCommand(projectId, targetId, type, properties, m_treeModel));
    }

    m_undoStack->endMacro();
}

void ProjectTreeCommands::addItemBefore(int projectId, int targetId, const QString &type, const QVariantMap &properties)
{
    m_undoStack->push(new AddItemBeforeCommand(projectId, targetId, type, properties, m_treeModel));
}

void ProjectTreeCommands::addSeveralItemsBefore(int projectId, int targetId, const QString &type, int count, const QVariantMap &properties)
{
    m_undoStack->beginMacro("Add several items before");

    for(int i = 0 ; i < count; i++){
        m_undoStack->push(new AddItemBeforeCommand(projectId, targetId, type, properties, m_treeModel));
    }

    m_undoStack->endMacro();
}

void ProjectTreeCommands::addSubItem(int projectId, int targetId, const QString &type, const QVariantMap &properties)
{
    m_undoStack->push(new AddSubItemCommand(projectId, targetId, type, properties, m_treeModel));
}

void ProjectTreeCommands::addSeveralSubItems(int projectId, int targetId, const QString &type, int count, const QVariantMap &properties)
{
    m_undoStack->beginMacro("Add several sub-items");

    for(int i = 0 ; i < count; i++){
        m_undoStack->push(new AddSubItemCommand(projectId, targetId, type, properties, m_treeModel));
    }

    m_undoStack->endMacro();
}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::setItemProperties(int projectId, int targetId, const QVariantMap &properties, bool isSystem)
{

    m_undoStack->beginMacro("Set tree item properties");

    QMapIterator<QString, QVariant> iter(properties);

    while (iter.hasNext()){
        iter.next();
        m_undoStack->push(new SetItemPropertyCommand(projectId, targetId, iter.key(), iter.value(), isSystem));
    }


    m_undoStack->endMacro();


}


QUndoStack *ProjectTreeCommands::undoStack() const
{
    return m_undoStack;
}

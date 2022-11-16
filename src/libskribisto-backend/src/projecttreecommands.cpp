#include "projecttreecommands.h"
#include "commands.h"
#include "interfaces/newprojecttemplateinterface.h"
#include "interfaces/pageinterface.h"

ProjectTreeCommands::ProjectTreeCommands(QObject *parent, QUndoStack *undoStack, ProjectTreeModel *treeModel)
    : QObject{parent}, m_undoStack(undoStack), m_treeModel(treeModel)
{
    m_instance = this;
    commands->subscribe(this);
    skrpluginhub->addPluginType<NewProjectTemplateInterface>();

}
ProjectTreeCommands *ProjectTreeCommands::m_instance = nullptr;


void ProjectTreeCommands::renameItem(int projectId, int targetId, const QString &newName)
{
    m_undoStack->push(new RenameItemCommand(projectId, targetId, newName));
}

//------------------------------------------------------------------------------------

int ProjectTreeCommands::addItemAfter(int projectId, int targetId, const QString &type, const QString &title, const QVariantMap &properties)
{
    m_undoStack->beginMacro("Add an item after");

    auto *command = new AddItemAfterCommand(projectId, targetId, type, properties, m_treeModel);
    m_undoStack->push(command);
    int newId = command->result();
    renameItem(projectId, newId, title);
    m_undoStack->endMacro();

    return newId;
}

//------------------------------------------------------------------------------------

QList<int> ProjectTreeCommands::addSeveralItemsAfter(int projectId, int targetId, const QString &type, int count, const QStringList &titles, const QVariantMap &properties)
{
    QList<int> newIds;

    m_undoStack->beginMacro("Add several items after");

    auto *command = new AddItemAfterCommand(projectId, targetId, type, properties, m_treeModel);
    m_undoStack->push(command);
    int newId = command->result();
    newIds.append(newId);
    renameItem(projectId, newId, titles.at(0));

    for(int i = 1 ; i < count; i++){
        command = new AddItemAfterCommand(projectId, newId, type, properties, m_treeModel);
        m_undoStack->push(command);
        newId = command->result();
        newIds.append(newId);


        if(i >= titles.size()){
            renameItem(projectId, newId, "");
        }
        else {
            renameItem(projectId, newId, titles.at(i));
        }
    }

    m_undoStack->endMacro();

    return newIds;
}

//------------------------------------------------------------------------------------

int ProjectTreeCommands::addItemBefore(int projectId, int targetId, const QString &type, const QString &title, const QVariantMap &properties)
{
    m_undoStack->beginMacro("Add an item before");

    auto *command = new AddItemBeforeCommand(projectId, targetId, type, properties, m_treeModel);
    m_undoStack->push(command);
    int newId = command->result();
    renameItem(projectId, newId, title);

    m_undoStack->endMacro();

    return newId;
}

//------------------------------------------------------------------------------------

QList<int>  ProjectTreeCommands::addSeveralItemsBefore(int projectId, int targetId, const QString &type, int count, const QStringList &titles, const QVariantMap &properties)
{
    QList<int> newIds;

    m_undoStack->beginMacro("Add several items before");

    auto *command = new AddItemBeforeCommand(projectId, targetId, type, properties, m_treeModel);
    m_undoStack->push(command);
    int newId = command->result();
    newIds.append(newId);
    renameItem(projectId, newId, titles.at(0));

    for(int i = 0 ; i < count; i++){
        auto *command = new AddItemAfterCommand(projectId, newId, type, properties, m_treeModel);
        m_undoStack->push(command);
        newId = command->result();
        newIds.append(newId);


        if(i >= titles.size()){
            renameItem(projectId, newId, "");
        }
        else {
            renameItem(projectId, newId, titles.at(i));
        }
    }

    m_undoStack->endMacro();

    return newIds;
}

//------------------------------------------------------------------------------------

int ProjectTreeCommands::addSubItem(int projectId, int targetId, const QString &type, const QString &title, const QVariantMap &properties)
{
    m_undoStack->beginMacro("Add a sub-item");

    auto *command = new AddSubItemCommand(projectId, targetId, type, properties, m_treeModel);
    m_undoStack->push(command);
    int newId = command->result();
    renameItem(projectId, newId, title);

    m_undoStack->endMacro();

    return newId;

}
//------------------------------------------------------------------------------------


QList<int>   ProjectTreeCommands::addSeveralSubItems(int projectId, int targetId, const QString &type, int count, const QStringList &titles, const QVariantMap &properties)
{
    QList<int> newIds;

    m_undoStack->beginMacro("Add several sub-items");

    for(int i = 0 ; i < count; i++){
        auto *command = new AddSubItemCommand(projectId, targetId, type, properties, m_treeModel);
        m_undoStack->push(command);
        int newId = command->result();
        newIds.append(newId);

        if(i >= titles.size()){
            renameItem(projectId, newId, "");
        }
        else {
            renameItem(projectId, newId, titles.at(i));
        }

    }

    m_undoStack->endMacro();

    return newIds;
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

//------------------------------------------------------------------------------------


void ProjectTreeCommands::moveItemsAbove(int sourceProjectId, QList<int> sourceIds, int targetProjectId, int targetId)
{

    m_undoStack->push(new MoveItemsCommand(sourceProjectId, sourceIds, targetProjectId, targetId, MoveItemsCommand::Above));



}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::moveItemsBelow(int sourceProjectId, QList<int> sourceIds, int targetProjectId, int targetId)
{

    m_undoStack->push(new MoveItemsCommand(sourceProjectId, sourceIds, targetProjectId, targetId, MoveItemsCommand::Below));

}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::moveItemsAsChildOf(int sourceProjectId, QList<int> sourceIds, int targetProjectId, int targetId)
{

    m_undoStack->push(new MoveItemsCommand(sourceProjectId, sourceIds, targetProjectId, targetId, MoveItemsCommand::AsChildOf));

}

//------------------------------------------------------------------------------------


int ProjectTreeCommands::addNote(int projectId, const QString &name, int targetFolderId)
{
    m_undoStack->beginMacro("Add note");

    int noteFolder = -1;
    if(targetFolderId == -1){
        QList<int> folders = skrdata->treeHub()->getIdsWithInternalTitle(projectId, "note_folder");

        if(folders.isEmpty()){
            noteFolder = this->addSubItem(projectId, 0, "FOLDER", tr("Notes"));
        }
        else {
            noteFolder = folders.first();
        }


    }
    else {
        noteFolder = targetFolderId;
    }


    int newId = this->addSubItem(projectId, noteFolder, "TEXT", name);



    m_undoStack->endMacro();

    return newId;
}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::sendItemToTrash(int projectId, int targetId)
{
    m_undoStack->push(new TrashItemCommand(projectId, targetId, true));

}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::sendSeveralItemsToTrash(int projectId, QList<int> targetIds)
{

    QList<int> filteredOutTargets = skrdata->treeHub()->filterOutChildren(projectId, targetIds);

    m_undoStack->beginMacro("Trash several items");

    for(int targetId : filteredOutTargets){
        m_undoStack->push(new TrashItemCommand(projectId, targetId, true));
    }

    m_undoStack->endMacro();

}

//------------------------------------------------------------------------------------

int ProjectTreeCommands::restoreItemFromTrash(int projectId, int targetId, int forcedOriginalParentId, int forcedOriginalRow)
{
    auto *command = new TrashItemCommand(projectId, targetId, false, forcedOriginalParentId, forcedOriginalRow);

    m_undoStack->push(command);
    if(!command->result()){
        return targetId;
    }

    return 0;
}

//------------------------------------------------------------------------------------

QList<int> ProjectTreeCommands::restoreSeveralItemsFromTrash(int projectId, QList<int> targetIds)
{
    QList<int> results;

    QList<int> filteredOutTargets = skrdata->treeHub()->filterOutChildren(projectId, targetIds);

    m_undoStack->beginMacro("Restore several items");

    for(int i = filteredOutTargets.size() - 1 ; i >= 0 ; i-- ){
        int targetId = filteredOutTargets.at(i);
        auto *command = new TrashItemCommand(projectId, targetId, false);
        m_undoStack->push(command);

        if(!command->result()){
            results.prepend(targetId);
        }
    }

    m_undoStack->endMacro();

    return results;
}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::setContent(int projectId, int targetId, const QString &content, bool isSecondary)
{

    if(isSecondary){
        skrdata->treeHub()->setSecondaryContent(projectId, targetId, content);
    }
    else {
        skrdata->treeHub()->setPrimaryContent(projectId, targetId, content);
    }
}

//------------------------------------------------------------------------------------


void ProjectTreeCommands::emptyTrash(int projectId)
{
    m_undoStack->clear();

    QList<int> idList = skrdata->treeHub()->getAllTrashedIds(projectId);

    for(int id : idList){
        skrdata->treeHub()->removeTreeItem(projectId, id);
    }

}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::updateCharAndWordCount(int projectId, int treeItemId, const QString &pageType, bool sameThread)
{

    QList<PageInterface *> pluginList = skrdata->pluginHub()->pluginsByType<PageInterface>();

    for (PageInterface *plugin: qAsConst(pluginList)) {
        if (pageType == plugin->pageType()) {
            plugin->updateCharAndWordCount(projectId, treeItemId, sameThread);
            break;
        }
    }
}

//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------


int ProjectTreeCommands::addTreeItemInTemplate(int projectId, int sortOrder, int indent, const QString &type, const QString &title, const QString &internalTitle,  const QVariantMap &custom_properties, bool renumber)
{


    QList<PageInterface *> pluginList =
            skrpluginhub->pluginsByType<PageInterface>();


    SKRResult result = skrdata->treeHub()->addTreeItem(projectId, sortOrder, indent, type, title, internalTitle, renumber);
    int newTreeItemId = result.getData("treeItemId", -1).toInt();

    for(auto *plugin : pluginList){
        if(plugin->pageType() == type){
            QVariantMap properties = plugin->propertiesForCreationOfTreeItem(custom_properties);

            QVariantMap::const_iterator i = properties.constBegin();
            while (i != properties.constEnd()) {
                SKRResult result = skrdata->treePropertyHub()->setProperty(projectId, newTreeItemId, i.key() , i.value().toString(), true );
                ++i;
            }

            break;
        }
    }
    emit skrdata->treeHub()->treeReset(projectId);

    return newTreeItemId;
}

//------------------------------------------------------------------------------------


QUndoStack *ProjectTreeCommands::undoStack() const
{
    return m_undoStack;
}

Command *ProjectTreeCommands::getCommand(const QString &action, const QVariantMap &parameters)
{
    if(action == "rename"){
        return new RenameItemCommand(parameters.value("projectId").toInt(), parameters.value("treeItemId").toInt(), parameters.value("name").toString());
    }
    return nullptr;

}

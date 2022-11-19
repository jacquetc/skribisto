#include "projecttreecommands.h"
#include "commands.h"
#include "interfaces/newprojecttemplateinterface.h"

ProjectTreeCommands::ProjectTreeCommands(QObject *parent, QUndoStack *undoStack, ProjectTreeModel *treeModel)
    : QObject{parent}, m_undoStack(undoStack), m_treeModel(treeModel)
{
    m_instance = this;
    commands->subscribe(this);
    skrpluginhub->addPluginType<NewProjectTemplateInterface>();
    m_pageInterfacePluginList = skrdata->pluginHub()->pluginsByType<PageInterface>();


}
ProjectTreeCommands *ProjectTreeCommands::m_instance = nullptr;


void ProjectTreeCommands::renameItem(const TreeItemAddress &targetTreeItemAddress, const QString &newName)
{
    m_undoStack->push(new RenameItemCommand(targetTreeItemAddress, newName));
}

//------------------------------------------------------------------------------------

TreeItemAddress ProjectTreeCommands::addItemAfter(const TreeItemAddress &targetTreeItemAddress, const QString &type, const QString &title, const QVariantMap &properties)
{
    m_undoStack->beginMacro("Add an item after");

    auto *command = new AddItemAfterCommand(targetTreeItemAddress, type, properties, m_treeModel);
    m_undoStack->push(command);
    TreeItemAddress newAddress = command->result();
    renameItem(newAddress, title);
    m_undoStack->endMacro();

    return newAddress;
}

//------------------------------------------------------------------------------------

QList<TreeItemAddress> ProjectTreeCommands::addSeveralItemsAfter(const TreeItemAddress &targetTreeItemAddress, const QString &type, int count, const QStringList &titles, const QVariantMap &properties)
{
    QList<TreeItemAddress> treeItemAddresses;

    m_undoStack->beginMacro("Add several items after");

    auto *command = new AddItemAfterCommand(targetTreeItemAddress, type, properties, m_treeModel);
    m_undoStack->push(command);
    TreeItemAddress newAddress = command->result();
    treeItemAddresses.append(newAddress);
    renameItem(newAddress, titles.at(0));

    for(int i = 1 ; i < count; i++){
        command = new AddItemAfterCommand(newAddress, type, properties, m_treeModel);
        m_undoStack->push(command);
        newAddress = command->result();
        treeItemAddresses.append(newAddress);


        if(i >= titles.size()){
            renameItem(newAddress, "");
        }
        else {
            renameItem(newAddress, titles.at(i));
        }
    }

    m_undoStack->endMacro();

    return treeItemAddresses;
}

//------------------------------------------------------------------------------------

TreeItemAddress ProjectTreeCommands::addItemBefore(const TreeItemAddress &targetTreeItemAddress, const QString &type, const QString &title, const QVariantMap &properties)
{
    m_undoStack->beginMacro("Add an item before");

    auto *command = new AddItemBeforeCommand(targetTreeItemAddress, type, properties, m_treeModel);
    m_undoStack->push(command);
    TreeItemAddress newAddress = command->result();
    renameItem(newAddress, title);

    m_undoStack->endMacro();

    return newAddress;
}

//------------------------------------------------------------------------------------

QList<TreeItemAddress>  ProjectTreeCommands::addSeveralItemsBefore(const TreeItemAddress &targetTreeItemAddress, const QString &type, int count, const QStringList &titles, const QVariantMap &properties)
{
    QList<TreeItemAddress> treeItemAddresses;

    m_undoStack->beginMacro("Add several items before");

    auto *command = new AddItemBeforeCommand(targetTreeItemAddress, type, properties, m_treeModel);
    m_undoStack->push(command);
    TreeItemAddress newAddress = command->result();
    treeItemAddresses.append(newAddress);
    renameItem(newAddress, titles.at(0));

    for(int i = 0 ; i < count; i++){
        auto *command = new AddItemAfterCommand(newAddress, type, properties, m_treeModel);
        m_undoStack->push(command);
        newAddress = command->result();
        treeItemAddresses.append(newAddress);


        if(i >= titles.size()){
            renameItem(newAddress, "");
        }
        else {
            renameItem(newAddress, titles.at(i));
        }
    }

    m_undoStack->endMacro();

    return treeItemAddresses;
}

//------------------------------------------------------------------------------------

TreeItemAddress ProjectTreeCommands::addSubItem(const TreeItemAddress &targetTreeItemAddress, const QString &type, const QString &title, const QVariantMap &properties)
{
    m_undoStack->beginMacro("Add a sub-item");

    auto *command = new AddSubItemCommand(targetTreeItemAddress, type, properties, m_treeModel);
    m_undoStack->push(command);
    TreeItemAddress newAddress = command->result();
    renameItem(newAddress, title);

    m_undoStack->endMacro();

    return newAddress;

}
//------------------------------------------------------------------------------------


QList<TreeItemAddress>   ProjectTreeCommands::addSeveralSubItems(const TreeItemAddress &targetTreeItemAddress, const QString &type, int count, const QStringList &titles, const QVariantMap &properties)
{
    QList<TreeItemAddress> treeItemAddresses;

    m_undoStack->beginMacro("Add several sub-items");

    for(int i = 0 ; i < count; i++){
        auto *command = new AddSubItemCommand(targetTreeItemAddress, type, properties, m_treeModel);
        m_undoStack->push(command);
        TreeItemAddress newAddress = command->result();
        treeItemAddresses.append(newAddress);

        if(i >= titles.size()){
            renameItem(newAddress, "");
        }
        else {
            renameItem(newAddress, titles.at(i));
        }

    }

    m_undoStack->endMacro();

    return treeItemAddresses;
}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::setItemProperties(const TreeItemAddress &targetTreeItemAddress, const QVariantMap &properties, bool isSystem)
{

    m_undoStack->beginMacro("Set tree item properties");

    QMapIterator<QString, QVariant> iter(properties);

    while (iter.hasNext()){
        iter.next();
        m_undoStack->push(new SetItemPropertyCommand(targetTreeItemAddress, iter.key(), iter.value(), isSystem));
    }


    m_undoStack->endMacro();

    
}

//------------------------------------------------------------------------------------


void ProjectTreeCommands::moveItemsAbove(QList<TreeItemAddress> sourceTreeItemAddresses, const TreeItemAddress &targetTreeItemAddress)
{

    m_undoStack->push(new MoveItemsCommand(sourceTreeItemAddresses, targetTreeItemAddress, MoveItemsCommand::Above));



}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::moveItemsBelow(QList<TreeItemAddress> sourceTreeItemAddresses, const TreeItemAddress &targetTreeItemAddress)
{

    m_undoStack->push(new MoveItemsCommand(sourceTreeItemAddresses, targetTreeItemAddress, MoveItemsCommand::Below));

}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::moveItemsAsChildOf(QList<TreeItemAddress> sourceTreeItemAddresses, const TreeItemAddress &targetTreeItemAddress)
{

    m_undoStack->push(new MoveItemsCommand(sourceTreeItemAddresses, targetTreeItemAddress, MoveItemsCommand::AsChildOf));

}

//------------------------------------------------------------------------------------


TreeItemAddress ProjectTreeCommands::addNote(int projectId, const QString &name, const TreeItemAddress &targetFolderAddress)
{
    m_undoStack->beginMacro("Add note");

    TreeItemAddress noteFolderAddress;
    if(!targetFolderAddress.isValid()){
        QList<TreeItemAddress> folders = skrdata->treeHub()->getIdsWithInternalTitle(projectId, "note_folder");

        if(folders.isEmpty()){
            noteFolderAddress = this->addSubItem(TreeItemAddress(projectId, 0), "FOLDER", tr("Notes"));
        }
        else {
            noteFolderAddress = folders.first();
        }


    }
    else {
        noteFolderAddress = targetFolderAddress;
    }


    TreeItemAddress newId = this->addSubItem(noteFolderAddress, "TEXT", name);



    m_undoStack->endMacro();

    return newId;
}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::sendItemToTrash(const TreeItemAddress &targetTreeItemAddress)
{
    m_undoStack->push(new TrashItemCommand(targetTreeItemAddress, true));

}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::sendSeveralItemsToTrash(QList<TreeItemAddress> targetTreeItemAddresses)
{

    QList<TreeItemAddress> filteredOutTargets = skrdata->treeHub()->filterOutChildren(targetTreeItemAddresses);

    m_undoStack->beginMacro("Trash several items");

    for(const TreeItemAddress &targetAddress : filteredOutTargets){
        m_undoStack->push(new TrashItemCommand(targetAddress, true));
    }

    m_undoStack->endMacro();

}

//------------------------------------------------------------------------------------

TreeItemAddress ProjectTreeCommands::restoreItemFromTrash(const TreeItemAddress &targetTreeItemAddress, const TreeItemAddress &forcedOriginalParentAddress , int forcedOriginalRow)
{
    auto *command = new TrashItemCommand(targetTreeItemAddress, false, forcedOriginalParentAddress, forcedOriginalRow);

    m_undoStack->push(command);
    if(!command->result()){
        return targetTreeItemAddress;
    }

    return TreeItemAddress();
}

//------------------------------------------------------------------------------------

QList<TreeItemAddress> ProjectTreeCommands::restoreSeveralItemsFromTrash(QList<TreeItemAddress> targetTreeItemAddresses)
{
    QList<TreeItemAddress> results;

    QList<TreeItemAddress> filteredOutTargets = skrdata->treeHub()->filterOutChildren(targetTreeItemAddresses);

    m_undoStack->beginMacro("Restore several items");

    for(int i = filteredOutTargets.size() - 1 ; i >= 0 ; i-- ){
        TreeItemAddress targetId = filteredOutTargets.at(i);
        auto *command = new TrashItemCommand(targetId, false);
        m_undoStack->push(command);

        if(!command->result()){
            results.prepend(targetId);
        }
    }

    m_undoStack->endMacro();

    return results;
}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::setContent(const TreeItemAddress &targetTreeItemAddress, const QString &content, bool isSecondary)
{

    if(isSecondary){
        skrdata->treeHub()->setSecondaryContent(targetTreeItemAddress, content);
    }
    else {
        skrdata->treeHub()->setPrimaryContent(targetTreeItemAddress, content);
    }
}

//------------------------------------------------------------------------------------


void ProjectTreeCommands::emptyTrash(int projectId)
{
    m_undoStack->clear();

    QList<TreeItemAddress> idList = skrdata->treeHub()->getAllTrashedIds(projectId);

    for(const TreeItemAddress &address : idList){
        skrdata->treeHub()->removeTreeItem(address);
    }

}

//------------------------------------------------------------------------------------

void ProjectTreeCommands::updateCharAndWordCount(const TreeItemAddress &treeItemAddress, const QString &pageType, bool sameThread)
{

    for (PageInterface *plugin: qAsConst(m_pageInterfacePluginList)) {
        if (pageType == plugin->pageType()) {
            plugin->updateCharAndWordCount(treeItemAddress, sameThread);
            break;
        }
    }
}

//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------


TreeItemAddress ProjectTreeCommands::addTreeItemInTemplate(int projectId, int sortOrder, int indent, const QString &type, const QString &title, const QString &internalTitle,  const QVariantMap &custom_properties, bool renumber)
{
    auto *command = new AddRawItemCommand(projectId, sortOrder, indent, type, title, internalTitle, custom_properties, renumber, m_treeModel);
    m_undoStack->push(command);
    TreeItemAddress newAddress = command->result();
//    SKRResult result = skrdata->treeHub()->addTreeItem(projectId, sortOrder, indent, type, title, internalTitle, renumber);
//    TreeItemAddress newTreeItemAddress = result.getData("treeItemAddress", QVariant::fromValue(TreeItemAddress())).value<TreeItemAddress>();

//    for(auto *plugin : m_pageInterfacePluginList){
//        if(plugin->pageType() == type){
//            QVariantMap properties = plugin->propertiesForCreationOfTreeItem(custom_properties);

//            QVariantMap::const_iterator i = properties.constBegin();
//            while (i != properties.constEnd()) {
//                SKRResult result = skrdata->treePropertyHub()->setProperty(newTreeItemAddress, i.key() , i.value().toString(), true );
//                ++i;
//            }

//            break;
//        }
//    }

    //TODO: not the more efficient way to avoid crashes:
    //skrdata->treeHub()->treeReset(projectId);

    return newAddress;
}

//------------------------------------------------------------------------------------


QUndoStack *ProjectTreeCommands::undoStack() const
{
    return m_undoStack;
}

Command *ProjectTreeCommands::getCommand(const QString &action, const QVariantMap &parameters)
{
    if(action == "rename"){
        return new RenameItemCommand(parameters.value("projectAddress").value<TreeItemAddress>(), parameters.value("name").toString());
    }
    return nullptr;

}

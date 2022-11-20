#ifndef PROJECTTREECOMMANDS_H
#define PROJECTTREECOMMANDS_H

#include "treemodels/projecttreemodel.h"
#include "skribisto_backend_global.h"
#include "interfaces/invokablecommandgroupinterface.h"
#include "interfaces/pageinterface.h"

#include <QObject>
#define projectTreeCommands ProjectTreeCommands::instance()

class SKRBACKENDEXPORT ProjectTreeCommands : public QObject, public InvokableCommandGroupInterface
{

    Q_OBJECT
public:
    explicit ProjectTreeCommands(QObject *parent, QUndoStack *undoStack, ProjectTreeModel *treeModel);

    static ProjectTreeCommands* instance()
    {
        return m_instance;
    }

    QUndoStack *undoStack() const;

    void renameItem(const TreeItemAddress &targetTreeItemAddress,  const QString &newName);

    TreeItemAddress addItemAfter(const TreeItemAddress &targetTreeItemAddress, const QString &type, const QString &title, const QVariantMap &properties = QVariantMap());
    TreeItemAddress addItemBefore(const TreeItemAddress &targetTreeItemAddress, const QString &type, const QString &title, const QVariantMap &properties = QVariantMap());
    TreeItemAddress addSubItem(const TreeItemAddress &targetTreeItemAddress, const QString &type, const QString &title, const QVariantMap &properties = QVariantMap());
    TreeItemAddress addTreeItemInTemplate(int projectId, int sortOrder, int indent, const QString &type, const QString &title, const QString &internalTitle = QString(),  const QVariantMap &custom_properties = QVariantMap(), bool renumber = false);


    QList<TreeItemAddress> addSeveralSubItems(const TreeItemAddress &targetTreeItemAddress, const QString &type, int count, const QStringList &titles, const QVariantMap &properties);
    QList<TreeItemAddress> addSeveralItemsAfter(const TreeItemAddress &targetTreeItemAddress, const QString &type, int count, const QStringList &titles, const QVariantMap &properties);
    QList<TreeItemAddress> addSeveralItemsBefore(const TreeItemAddress &targetTreeItemAddress, const QString &type, int count, const QStringList &titles, const QVariantMap &properties);

    void setItemProperties(const TreeItemAddress &targetTreeItemAddress, const QVariantMap &properties, bool isSystem = true);

    void moveItemsAbove(QList<TreeItemAddress> sourceTreeItemAddresses, const TreeItemAddress &targetTreeItemAddress);
    void moveItemsBelow(QList<TreeItemAddress> sourceTreeItemAddresses, const TreeItemAddress &targetTreeItemAddress);
    void moveItemsAsChildOf(QList<TreeItemAddress> sourceTreeItemAddresses, const TreeItemAddress &targetTreeItemAddress);

    TreeItemAddress addNote(int projectId, const QString &name, const TreeItemAddress &targetFolderAddress = TreeItemAddress());
    void sendItemToTrash(const TreeItemAddress &targetTreeItemAddress);
    void sendSeveralItemsToTrash(QList<TreeItemAddress> targetTreeItemAddresses);

    ///
    /// \brief restoreItemFromTrash
    /// \param projectId
    /// \param targetId
    /// \param forcedOriginalParentId
    /// \param forcedOriginalRow
    /// \return 0 if ok, or the targetId if can't restore by lack of valid orignal parent and/or row
    ///
    TreeItemAddress restoreItemFromTrash(const TreeItemAddress &targetTreeItemAddress, const TreeItemAddress &forcedOriginalParentAddress = TreeItemAddress(), int forcedOriginalRow = -1);
    QList<TreeItemAddress> restoreSeveralItemsFromTrash(QList<TreeItemAddress> targetTreeItemAddresses);

    void setContent(const TreeItemAddress &targetTreeItemAddress, const QString &&content, bool isSecondary = false);

    void emptyTrash(int projectId);

    void updateCharAndWordCount(const TreeItemAddress &treeItemAddress, const QString& pageType, bool sameThread);

signals:
private:
    static ProjectTreeCommands *m_instance;
QUndoStack *m_undoStack;
ProjectTreeModel *m_treeModel;
QList<PageInterface *> m_pageInterfacePluginList;


// InvokableCommandGroupInterface interface
public:
QString address() const override
{
    return "project_tree";
}
Command *getCommand(const QString &action, const QVariantMap &parameters) override;
};

#endif // PROJECTTREECOMMANDS_H

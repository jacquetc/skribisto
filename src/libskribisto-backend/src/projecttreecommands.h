#ifndef PROJECTTREECOMMANDS_H
#define PROJECTTREECOMMANDS_H

#include "treemodels/projecttreemodel.h"
#include "skribisto_backend_global.h"
#include "interfaces/invokablecommandgroupinterface.h"

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

    void renameItem(int projectId, int targetId,  const QString &newName);

    int addItemAfter(int projectId, int targetId, const QString &type, const QString &title, const QVariantMap &properties = QVariantMap());
    int addItemBefore(int projectId, int targetId, const QString &type, const QString &title, const QVariantMap &properties = QVariantMap());
    int addSubItem(int projectId, int targetId, const QString &type, const QString &title, const QVariantMap &properties = QVariantMap());
    int addTreeItemInTemplate(int projectId, int sortOrder, int indent, const QString &type, const QString &title, const QString &internalTitle = QString(),  const QVariantMap &custom_properties = QVariantMap(), bool renumber = false);


    QList<int> addSeveralSubItems(int projectId, int targetId, const QString &type, int count, const QStringList &titles, const QVariantMap &properties);
    QList<int> addSeveralItemsAfter(int projectId, int targetId, const QString &type, int count, const QStringList &titles, const QVariantMap &properties);
    QList<int> addSeveralItemsBefore(int projectId, int targetId, const QString &type, int count, const QStringList &titles, const QVariantMap &properties);

    void setItemProperties(int projectId, int targetId, const QVariantMap &properties, bool isSystem = true);

    void moveItemsAbove(int sourceProjectId, QList<int> sourceIds, int targetProjectId, int targetId);
    void moveItemsBelow(int sourceProjectId, QList<int> sourceIds, int targetProjectId, int targetId);
    void moveItemsAsChildOf(int sourceProjectId, QList<int> sourceIds, int targetProjectId, int targetId);

    int addNote(int projectId, const QString &name, int targetFolderId = -1);
    void sendItemToTrash(int projectId, int targetId);
    void sendSeveralItemsToTrash(int projectId, QList<int> targetIds);

    ///
    /// \brief restoreItemFromTrash
    /// \param projectId
    /// \param targetId
    /// \param forcedOriginalParentId
    /// \param forcedOriginalRow
    /// \return 0 if ok, or the targetId if can't restore by lack of valid orignal parent and/or row
    ///
    int restoreItemFromTrash(int projectId, int targetId, int forcedOriginalParentId = -1, int forcedOriginalRow = -1);
    QList<int> restoreSeveralItemsFromTrash(int projectId, QList<int> targetIds);

    void setContent(int projectId, int targetId, const QString &content, bool isSecondary = false);

    void emptyTrash(int projectId);

    void updateCharAndWordCount(int projectId, int treeItemId, const QString& pageType, bool sameThread);

signals:
private:
    static ProjectTreeCommands *m_instance;
QUndoStack *m_undoStack;
ProjectTreeModel *m_treeModel;


// InvokableCommandGroupInterface interface
public:
QString address() const override
{
    return "project_tree";
}
Command *getCommand(const QString &action, const QVariantMap &parameters) override;
};

#endif // PROJECTTREECOMMANDS_H

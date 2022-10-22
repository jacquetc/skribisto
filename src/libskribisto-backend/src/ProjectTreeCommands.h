#ifndef PROJECTTREECOMMANDS_H
#define PROJECTTREECOMMANDS_H

#include "treemodels/projecttreemodel.h"
#include "skribisto_backend_global.h"

#include <QObject>
#define projectTreeCommands ProjectTreeCommands::instance()

class SKRBACKENDEXPORT ProjectTreeCommands : public QObject
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


    QList<int> addSeveralSubItems(int projectId, int targetId, const QString &type, int count, const QStringList &titles, const QVariantMap &properties);
    QList<int> addSeveralItemsAfter(int projectId, int targetId, const QString &type, int count, const QStringList &titles, const QVariantMap &properties);
    QList<int> addSeveralItemsBefore(int projectId, int targetId, const QString &type, int count, const QStringList &titles, const QVariantMap &properties);

    void setItemProperties(int projectId, int targetId, const QVariantMap &properties, bool isSystem = true);

    void moveItemsAbove(int sourceProjectId, QList<int> sourceIds, int targetProjectId, int targetId);
    void moveItemsBelow(int sourceProjectId, QList<int> sourceIds, int targetProjectId, int targetId);
    void moveItemsAsChildOf(int sourceProjectId, QList<int> sourceIds, int targetProjectId, int targetId);

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

    void emptyTrash(int projectId);

signals:
private:
    static ProjectTreeCommands *m_instance;
QUndoStack *m_undoStack;
ProjectTreeModel *m_treeModel;
};

#endif // PROJECTTREECOMMANDS_H

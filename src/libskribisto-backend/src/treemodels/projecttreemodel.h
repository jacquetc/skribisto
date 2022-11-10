#ifndef PROJECTTREEMODEL_H
#define PROJECTTREEMODEL_H

#include "projecttreeitem.h"
#include "skribisto_backend_global.h"
#include "interfaces/pagetypeiconinterface.h"
#include "skrdata.h"
#include "command.h"
#include "skrresult.h"

#include <QAbstractItemModel>
#include <QUndoCommand>

#define projectTreeModel ProjectTreeModel::instance()

class SKRBACKENDEXPORT ProjectTreeModel : public QAbstractItemModel
{
    Q_OBJECT
    friend class AddItemAfterCommand;
    friend class AddItemBeforeCommand;
    friend class AddSubItemCommand;
    friend class TrashItemCommand;

public:
    explicit ProjectTreeModel(QObject *parent = nullptr);


    static ProjectTreeModel* instance()
    {
        return m_instance;
    }

    // Header:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;

    // Basic functionality:
    QModelIndex index(int row, int column,
                      const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;

    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    int columnCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    // Editable:
    bool setData(const QModelIndex &index, const QVariant &value,
                 int role = Qt::EditRole) override;

    Qt::ItemFlags flags(const QModelIndex& index) const override;

    // Add data:
    bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    bool insertColumns(int column, int count, const QModelIndex &parent = QModelIndex()) override;

    // Remove data:
    bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;
    bool removeColumns(int column, int count, const QModelIndex &parent = QModelIndex()) override;
    QHash<int, QByteArray> roleNames() const override;
    QModelIndex getModelIndex(int projectId, int treeItemId) const;

public:


protected:


private slots:

    void populate();
    void clear();
    void exploitSignalFromSKRData(int                projectId,
                                  int                treeItemId,
                                  ProjectTreeItem::Roles role);

private:
    void connectToSKRDataSignals();
    void disconnectFromSKRDataSignals();
    static ProjectTreeModel *m_instance;
    SKRTreeHub *m_treeHub;
    SKRPropertyHub *m_propertyHub;
    ProjectTreeItem *m_rootItem;
    QList<QMetaObject::Connection>m_dataConnectionsList;
    QList<ProjectTreeItem *> m_itemList;
    QHash<QString, PageTypeIconInterface *> m_typeWithPlugin;


    ProjectTreeItem *getTreeItem(int projectId, int treeItemId) const;
    void removeProjectItem(int projectId, int treeItemId);
};


//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------


class AddItemAfterCommand : public QUndoCommand
{
public:
    AddItemAfterCommand(int projectId, int targetId, const QString &type, const QVariantMap &properties, ProjectTreeModel *model);
    void undo();
    void redo();
    int result();

private:
    int m_projectId, m_targetId, m_newId;
    QString m_type;
    QVariantMap m_properties;
    QList<int> m_propertyIds;
    ProjectTreeModel *m_model;
    QVariantMap m_savedItemValues;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------


class AddItemBeforeCommand : public QUndoCommand
{
public:
    AddItemBeforeCommand(int projectId, int targetId, const QString &type, const QVariantMap &properties, ProjectTreeModel *model);
    void undo();
    void redo();
    int result();
private:
    int m_projectId, m_targetId, m_newId;
    QString m_type;
    QVariantMap m_properties;
    QList<int> m_propertyIds;
    ProjectTreeModel *m_model;
    QVariantMap m_savedItemValues;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class AddSubItemCommand : public QUndoCommand
{
public:
    AddSubItemCommand(int projectId, int targetId, const QString &type, const QVariantMap &properties, ProjectTreeModel *model);
    void undo();
    void redo();
    int result();
private:
    int m_projectId, m_targetId, m_newId;
    QString m_type;
    QVariantMap m_properties;
    QList<int> m_propertyIds;
    ProjectTreeModel *m_model;
    QVariantMap m_savedItemValues;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class SetItemPropertyCommand : public QUndoCommand
{
public:
    SetItemPropertyCommand(int projectId, int targetId, const QString &property, const QVariant &value, bool isSystem);
    void undo();
    void redo();
private:
    int m_projectId, m_targetId, m_newId;
    QString m_property;
    QVariant m_newValue;
    QVariant m_oldValue;
    bool m_isSystem;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class MoveItemsCommand : public Command
{
    Q_GADGET
public:

    enum Move {
        Above,
        Below,
        AsChildOf
    };
    Q_ENUM(Move)



    MoveItemsCommand(int sourceProjectId, QList<int>  sourceIds, int targetProjectId, int targetId, Move move);
    void undo();
    void redo();
private:
    int m_sourceProjectId, m_targetProjectId, m_targetId;
    QList<int> m_sourceIds;
    QList<QVariantMap> m_newTree;
    QList<QVariantMap> m_oldTree;
    Move m_move;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class RenameItemCommand : public Command
{
public:


    RenameItemCommand(int projectId, int treeItemId, const QString &newName);
    void undo();
    void redo();
private:
    int m_projectId, m_treeItemId;
    QString m_oldName, m_newName;

};


//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class TrashItemCommand : public Command
{
public:
    TrashItemCommand(int projectId, int treeItemId, bool newTrashState, int forcedOriginalParentId = -1, int forcedOriginalRow = -1);
    void undo();
    void redo();
    bool result() const;
private:
    int m_projectId, m_treeItemId, m_originalParentId, m_originalRow, m_forcedOriginalParentId, m_forcedOriginalRow;
    bool m_oldTrashState, m_newTrashState;
    QList<QVariantMap> m_newTree, m_newPropertyTable;
    QList<QVariantMap> m_oldTree, m_oldPropertyTable;
    bool m_result;

};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

#endif // PROJECTTREEMODEL_H
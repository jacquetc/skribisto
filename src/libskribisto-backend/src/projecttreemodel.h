#ifndef PROJECTTREEMODEL_H
#define PROJECTTREEMODEL_H

#include "projecttreeitem.h"
#include "skribisto_backend_global.h"
#include "skrdata.h"

#include <QAbstractItemModel>
#include <QUndoCommand>
#define projectTreeModel ProjectTreeModel::instance()

class SKRBACKENDEXPORT ProjectTreeModel : public QAbstractItemModel
{
    Q_OBJECT
    friend class AddItemAfterCommand;
    friend class AddItemBeforeCommand;
    friend class AddSubItemCommand;

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


    QModelIndex getModelIndex(int projectId, int treeItemId) const;
    ProjectTreeItem *getProjectItem(int projectId, int treeItemId) const;
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
private:
    int m_projectId, m_targetId, m_newId;
    QString m_type;
    QVariantMap m_properties;
    QList<int> m_propertyIds;
    ProjectTreeModel *m_model;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------


class AddItemBeforeCommand : public QUndoCommand
{
public:
    AddItemBeforeCommand(int projectId, int targetId, const QString &type, const QVariantMap &properties, ProjectTreeModel *model);
    void undo();
    void redo();
private:
    int m_projectId, m_targetId, m_newId;
    QString m_type;
    QVariantMap m_properties;
    QList<int> m_propertyIds;
    ProjectTreeModel *m_model;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

class AddSubItemCommand : public QUndoCommand
{
public:
    AddSubItemCommand(int projectId, int targetId, const QString &type, const QVariantMap &properties, ProjectTreeModel *model);
    void undo();
    void redo();
private:
    int m_projectId, m_targetId, m_newId;
    QString m_type;
    QVariantMap m_properties;
    QList<int> m_propertyIds;
    ProjectTreeModel *m_model;
};

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

#endif // PROJECTTREEMODEL_H

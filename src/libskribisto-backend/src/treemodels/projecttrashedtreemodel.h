#ifndef PROJECTTRASHEDTREEMODEL_H
#define PROJECTTRASHEDTREEMODEL_H

#include "projecttreeitem.h"
#include "skribisto_backend_global.h"
#include "interfaces/pagetypeiconinterface.h"

#include <QAbstractItemModel>
#include <QUndoCommand>
#include <command.h>

class SKRBACKENDEXPORT ProjectTrashedTreeModel : public QAbstractItemModel
{
    Q_OBJECT
public:
    explicit ProjectTrashedTreeModel(QObject *parent = nullptr);


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

    QHash<int, QByteArray> roleNames() const override;
    QModelIndex getModelIndex(const TreeItemAddress &treeItemAddress) const;

public:


    int projectId() const;
    void setProjectId(int newProjectId);

signals:
    void projectIdChanged();

protected:


private slots:

    void populate();
    void clear();
    void exploitSignalFromSKRData(const TreeItemAddress &treeItemAddress,
                                  ProjectTreeItem::Roles role);

private:
    void connectToSKRDataSignals();
    void disconnectFromSKRDataSignals();
    int m_projectId;
    SKRTreeHub *m_treeHub;
    SKRPropertyHub *m_propertyHub;
    ProjectTreeItem *m_rootItem;
    QList<QMetaObject::Connection>m_dataConnectionsList;
    QList<ProjectTreeItem *> m_itemList;
    QHash<QString, PageTypeIconInterface *> m_typeWithPlugin;


    ProjectTreeItem *getTreeItem(const TreeItemAddress &treeItemAddress) const;
    void removeProjectItem(const TreeItemAddress &treeItemAddress);
    Q_PROPERTY(int projectId READ projectId WRITE setProjectId NOTIFY projectIdChanged)
};



#endif // PROJECTTRASHEDTREEMODEL_H

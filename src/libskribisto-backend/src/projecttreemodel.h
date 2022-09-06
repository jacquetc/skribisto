#ifndef PROJECTTREEMODEL_H
#define PROJECTTREEMODEL_H

#include "projecttreeitem.h"
#include "skribisto_backend_global.h"

#include <QAbstractItemModel>
#define projectTreeModel ProjectTreeModel::instance()

class ProjectTreeModel : public QAbstractItemModel
{
    Q_OBJECT

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
    QList<ProjectTreeItem *> m_itemList;
    QList<QMetaObject::Connection>m_dataConnectionsList;

    ProjectTreeItem *searchForExistingItem(int projectId, int treeItemId) const;

};

#endif // PROJECTTREEMODEL_H

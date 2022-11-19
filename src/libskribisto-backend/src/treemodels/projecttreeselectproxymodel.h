#ifndef PROJECTTREESELECTPROXYMODEL_H
#define PROJECTTREESELECTPROXYMODEL_H

#include <QSortFilterProxyModel>
#include "skribisto_backend_global.h"
#include "treeitemaddress.h"

class SKRBACKENDEXPORT ProjectTreeSelectProxyModel : public QSortFilterProxyModel
{
    Q_OBJECT

public:
    explicit ProjectTreeSelectProxyModel(QObject *parent, int projectId);

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    Qt::ItemFlags flags(const QModelIndex& index) const override;

    int projectId() const;
    void setProjectId(int newProjectId);

private:
    int m_projectId;
    QHash<TreeItemAddress, Qt::CheckState>m_checkedIdsHash;

    // QSortFilterProxyModel interface
    void checkStateOfAllChildren(const TreeItemAddress &treeItemAddress, Qt::CheckState checkState);
    void determineCheckStateOfAllAncestors(const TreeItemAddress &treeItemAddress, Qt::CheckState checkState);
protected:
    bool filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const override;

    // QAbstractItemModel interface
public:
    bool setData(const QModelIndex &index, const QVariant &value, int role) override;
    void setCheckedIdsList(const QList<TreeItemAddress> checkedAddresses);
    QList<TreeItemAddress> getCheckedIdsList();
    void checkNone();
};

#endif // PROJECTTREESELECTPROXYMODEL_H

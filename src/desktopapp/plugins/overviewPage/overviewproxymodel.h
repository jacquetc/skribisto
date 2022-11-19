#ifndef OVERVIEWPROXYMODEL_H
#define OVERVIEWPROXYMODEL_H

#include "treeitemaddress.h"
#include <QSortFilterProxyModel>

class OverviewProxyModel : public QSortFilterProxyModel
{
    Q_OBJECT

public:
    explicit OverviewProxyModel(QObject *parent = nullptr);

    // Basic functionality:


    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    int projectId() const;
    void setProjectId(int newProjectId);
    int columnCount(const QModelIndex &parent) const override;
    Qt::ItemFlags flags(const QModelIndex &index) const override;

    QVariant headerData(int section, Qt::Orientation orientation, int role) const override;

    QModelIndex getModelIndex(const TreeItemAddress &treeItemAddress) const;
private:

    // QSortFilterProxyModel interface
protected:
    bool filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const override;


private:
    int m_projectId;

};

#endif // OVERVIEWPROXYMODEL_H

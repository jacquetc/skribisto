#ifndef OVERVIEWPROXYMODEL_H
#define OVERVIEWPROXYMODEL_H

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

    QModelIndex getModelIndex(int projectId, int treeItemId) const;
private:

    // QSortFilterProxyModel interface
protected:
    bool filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const override;


private:
    int m_projectId;

    // QAbstractItemModel interface
public:
    int columnCount(const QModelIndex &parent) const override;
};

#endif // OVERVIEWPROXYMODEL_H

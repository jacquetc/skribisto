#ifndef PROJECTTREEPROXYMODEL_H
#define PROJECTTREEPROXYMODEL_H

#include <QIdentityProxyModel>
#include "interfaces/pagetypeiconinterface.h"
#include "skribisto_backend_global.h"

class SKRBACKENDEXPORT ProjectTreeProxyModel : public QIdentityProxyModel
{
    Q_OBJECT

public:
    explicit ProjectTreeProxyModel(QObject *parent = nullptr);

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    Qt::ItemFlags flags(const QModelIndex& index) const override;

    QModelIndex getModelIndex(const TreeItemAddress &treeItemAddress) const;
private:
        QHash<QString, PageTypeIconInterface *> m_typeWithPlugin;

        // QAbstractItemModel interface
public:
        QStringList mimeTypes() const override;
        QMimeData *mimeData(const QModelIndexList &indexes) const override;
        bool canDropMimeData(const QMimeData *data, Qt::DropAction action, int row, int column, const QModelIndex &parent) const override;
        bool dropMimeData(const QMimeData *data, Qt::DropAction action, int row, int column, const QModelIndex &parent) override;
        Qt::DropActions supportedDropActions() const override;
        Qt::DropActions supportedDragActions() const override;
};

#endif // PROJECTTREEPROXYMODEL_H

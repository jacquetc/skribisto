#ifndef PROJECTTREEPROXYMODEL_H
#define PROJECTTREEPROXYMODEL_H

#include <QIdentityProxyModel>
#include "interfaces/pageinterface.h"

class ProjectTreeProxyModel : public QIdentityProxyModel
{
    Q_OBJECT

public:
    explicit ProjectTreeProxyModel(QObject *parent = nullptr);

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    Qt::ItemFlags flags(const QModelIndex& index) const override;

private:
        QHash<QString, PageInterface *> m_typeWithPlugin;
};

#endif // PROJECTTREEPROXYMODEL_H

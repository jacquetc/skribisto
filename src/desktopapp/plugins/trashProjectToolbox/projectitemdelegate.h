#pragma once


#include <QStyledItemDelegate>

class ProjectItemDelegate : public QStyledItemDelegate
{
    Q_OBJECT
public:
    explicit ProjectItemDelegate(QObject *parent = nullptr);
    QSize sizeHint(const QStyleOptionViewItem &option, const QModelIndex &index) const;
};



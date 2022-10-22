#include "projectitemdelegate.h"

ProjectItemDelegate::ProjectItemDelegate(QObject *parent)
    : QStyledItemDelegate{parent}
{

}

QSize ProjectItemDelegate::sizeHint(const QStyleOptionViewItem &option, const QModelIndex &index) const
{
 return QSize(QStyledItemDelegate::sizeHint(option, index).width(), 23);
}

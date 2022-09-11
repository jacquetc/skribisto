#include "itemdelegate.h"

ItemDelegate::ItemDelegate()
{

}

QSize ItemDelegate::sizeHint(const QStyleOptionViewItem &option, const QModelIndex &index) const
{
 return QSize(QStyledItemDelegate::sizeHint(option, index).width(), 23);
}

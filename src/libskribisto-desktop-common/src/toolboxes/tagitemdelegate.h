#ifndef TAGITEMDELEGATE_H
#define TAGITEMDELEGATE_H

#include <QStyledItemDelegate>
#include <QObject>
#include "skribisto_desktop_common_global.h"

class SKRDESKTOPCOMMONEXPORT TagItemDelegate : public QStyledItemDelegate
{
    Q_OBJECT
public:
    explicit TagItemDelegate(QObject *parent = nullptr);

    // QAbstractItemDelegate interface
public:
    void paint(QPainter *painter, const QStyleOptionViewItem &option, const QModelIndex &index) const override;
    QSize sizeHint(const QStyleOptionViewItem &option, const QModelIndex &index) const override;
};

#endif // TAGITEMDELEGATE_H

#ifndef OUTLINEITEMDELEGATE_H
#define OUTLINEITEMDELEGATE_H

#include <QStyledItemDelegate>
#include <QObject>

class OutlineItemDelegate : public QStyledItemDelegate
{
    Q_OBJECT

public:
    explicit OutlineItemDelegate(QObject *parent = nullptr);

    // QAbstractItemDelegate interface
public:
    void paint(QPainter *painter, const QStyleOptionViewItem &option, const QModelIndex &index) const override;
    QSize sizeHint(const QStyleOptionViewItem &option, const QModelIndex &index) const override;
    QWidget *createEditor(QWidget *parent, const QStyleOptionViewItem &option, const QModelIndex &index) const override;
    void setEditorData(QWidget *editor, const QModelIndex &index) const override;
    void setModelData(QWidget *editor, QAbstractItemModel *model, const QModelIndex &index) const override;

    // QObject interface

    bool eventFilter(QObject *watched, QEvent *event) override;

    // QAbstractItemDelegate interface

    bool editorEvent(QEvent *event, QAbstractItemModel *model, const QStyleOptionViewItem &option, const QModelIndex &index) override;

    // QAbstractItemDelegate interface

    void updateEditorGeometry(QWidget *editor, const QStyleOptionViewItem &option, const QModelIndex &index) const override;


signals:
    void editFinished(const QModelIndex &index) const;
};

#endif // OUTLINEITEMDELEGATE_H

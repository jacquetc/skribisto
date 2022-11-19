#ifndef VIEWHOLDER_H
#define VIEWHOLDER_H

#include "view.h"

#include <QUuid>
#include <QWidget>
#include <QAction>
#include <QDateTime>
#include <QUuid>

namespace Ui {
class ViewHolder;
}

class HistoryItem;
class ViewHolder : public QWidget
{
    Q_OBJECT
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid NOTIFY uuidChanged)

public:
    explicit ViewHolder(QWidget *parent = nullptr);
    ~ViewHolder();

    QList<View *> viewList() const;
    void addView(View *view);
    void removeView(View *view);
    void removeViews(const TreeItemAddress &treeItemAddress = TreeItemAddress());
    void clear();
    View *currentView() const;
    void setCurrentView(View *view);

    QUuid uuid() const;
    void setUuid(const QUuid &newUuid);

    //history
    QAction *goBackAction() const;
    QAction *goForwardAction() const;

signals:
    void viewtoBeAdded(View *view);
    void viewAdded(View *view);
    void viewToBeRemoved(View *view);
    void viewRemoved();
    void aboutToBeDestroyed();
    void uuidChanged();

private slots:
    //history
    void addToHistory(View *view, const QVariantMap &parameters);
    void clearHistoryOfView(View *view);
    void goBackInHistory();
    void goForwardInHistory();

private:
    Ui::ViewHolder *ui;
    QUuid m_uuid;
    QList<View *> m_viewList;
    QAction *m_goBackAction, *m_goForwardAction;

    //history

    HistoryItem *m_currentHistoryItem;
    QList<HistoryItem *> m_historyItemList;



    // QWidget interface
protected:
    void dropEvent(QDropEvent *event) override;
    void dragEnterEvent(QDragEnterEvent *event) override;
    void dragLeaveEvent(QDragLeaveEvent *event) override;
    void dragMoveEvent(QDragMoveEvent *event) override;

};

//-------------------------------

class HistoryItem
{
public:
    explicit HistoryItem(View *view, const QVariantMap &parameters);

    QDateTime date() const;

    View *view() const;

    QVariantMap parameters() const;
    bool operator==(const HistoryItem& otherHistoryItem) const;

private:
    QUuid m_uuid;
    QDateTime m_date;
    View *m_view;
    QVariantMap m_parameters;

};

#endif // VIEWHOLDER_H

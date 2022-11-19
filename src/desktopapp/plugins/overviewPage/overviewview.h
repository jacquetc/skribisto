#ifndef OVERVIEWVIEW_H
#define OVERVIEWVIEW_H

#include <QWidget>
#include "view.h"
#include "overviewproxymodel.h"

namespace Ui {
class OverviewView;
}

typedef QPair<int, int> PathItem;
typedef QList<PathItem> Path;

class OverviewView : public View
{
    Q_OBJECT

public:
    explicit OverviewView(QWidget *parent = nullptr);
    ~OverviewView();
    QList<Toolbox *> toolboxes();

protected:
    void initialize();

private slots:
    void onCustomContextMenu(const QPoint &point);
    void saveExpandStates();
    void restoreExpandStates();
    void expandProjectItems();
    void setCurrentIndex(const QModelIndex &index);

private:
    Path pathFromIndex(const QModelIndex &index);
    QModelIndex pathToIndex(const Path &path);
    Ui::OverviewView *centralWidgetUi;
    OverviewProxyModel *m_overviewProxyModel;
    TreeItemAddress m_targetTreeItemAddress;
    QModelIndex m_currentModelIndex;
    QAction *m_addItemAfterAction, *m_addItemBeforeAction, *m_addSubItemAction,
    *m_openItemAction, *m_openItemInAnotherViewAction, *m_openItemInANewWindowAction, *m_renameAction, *m_sendToTrashAction, *m_copyItemsAction, *m_cutItemsAction, *m_pasteItemsAction;

    //QList< QPair<int, int>> copyCutList;

    void open(const QModelIndex &index);
    void openInAnotherView(const QModelIndex &index);
};

#endif // OVERVIEWVIEW_H

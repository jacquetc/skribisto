#ifndef NAVIGATION_H
#define NAVIGATION_H

#include "toolbox.h"
#include <QModelIndex>
#include <QWidget>

namespace Ui {
class Navigation;
}

typedef QPair<int, int> PathItem;
typedef QList<PathItem> Path;

class Navigation : public Toolbox
{
    Q_OBJECT

public:
    explicit Navigation(QWidget *parent = nullptr);
    ~Navigation();
    QString title() const {
        return tr("Navigation");
    }
    QIcon icon() const;

private slots:
    void onCustomContextMenu(const QPoint &point);
    void saveExpandStates();
    void restoreExpandStates();
    void addToExpandPathes(const QModelIndex &modelIndex);
    void expandProjectItems();
    void setCurrentIndex(const QModelIndex &index);

private:
    Path pathFromIndex(const QModelIndex &index);
    QModelIndex pathToIndex(const Path &path);


    Ui::Navigation *ui;
    QList<Path> m_expandedPathes;
    int m_targetTreeItemId, m_projectId;
    QModelIndex m_currentModelIndex;
    QAction *m_addItemAfterAction, *m_addItemBeforeAction, *m_addSubItemAction,
    *m_openItemAction, *m_openItemInAnotherViewAction, *m_openItemInANewWindowAction, *m_renameAction, *m_sendToTrashAction, *m_copyItemsAction, *m_cutItemsAction, *m_pasteItemsAction;

    QList< QPair<int, int>> copyCutList;

    void open(const QModelIndex &index);
    void openInAnotherView(const QModelIndex &index);
protected:
};

#endif // NAVIGATION_H

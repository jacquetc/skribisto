#ifndef NAVIGATION_H
#define NAVIGATION_H

#include "toolbox.h"
#include <QModelIndex>
#include <QSortFilterProxyModel>
#include <QWidget>

namespace Ui {
class Navigation;
}

typedef QPair<int, int> PathItem;
typedef QList<PathItem> Path;

class FilterModel;

class Navigation : public Toolbox
{
    Q_OBJECT

public:
    explicit Navigation(QWidget *parent = nullptr);
    ~Navigation();
    QString title() const override{
        return tr("Navigation");
    }
    QIcon icon() const override;

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
    TreeItemAddress m_targetTreeItemAddress;
    QModelIndex m_currentModelIndex;
    QAction *m_addItemAfterAction, *m_addItemBeforeAction, *m_addSubItemAction,
    *m_openItemAction, *m_openItemInAnotherViewAction, *m_openItemInANewWindowAction, *m_renameAction,
    *m_sendToTrashAction, *m_copyItemsAction, *m_cutItemsAction, *m_pasteItemsAction, *m_setActiveProjectAction,
    *m_exportAction;

    //QList< QPair<int, int>> copyCutList;

    void open(const QModelIndex &index);
    void openInAnotherView(const QModelIndex &index);
protected:

    // Toolbox interface
public:
    void initialize() override;
};

//----------------------------------------

class FilterModel : public QSortFilterProxyModel
{

public:
    FilterModel(QObject *parent = nullptr);

    QModelIndex getModelIndex(const TreeItemAddress &treeItemAddress) const;
protected:
    bool filterAcceptsRow(int source_row, const QModelIndex &source_parent) const override;


    // QAbstractItemModel interface
public:
    int columnCount(const QModelIndex &parent) const override;
};


#endif // NAVIGATION_H

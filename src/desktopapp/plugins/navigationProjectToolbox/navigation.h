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

private slots:
    void onCustomContextMenu(const QPoint &point);
    void saveExpandStates();
    void restoreExpandStates();
    void addToExpandPathes(const QModelIndex &modelIndex);
    void removeFromExpandPathes(const QModelIndex &modelIndex);
    void on_treeView_doubleClicked(const QModelIndex &index);

private:
    Path pathFromIndex(const QModelIndex &index);
    QModelIndex pathToIndex(const Path &path);


    Ui::Navigation *ui;
    QWidget * QWidget;
    QList<Path> m_expandedPathes;
    int m_targetTreeItemId, m_projectId;
    QModelIndex m_currentModelIndex;
    QAction *m_addItemAfterAction, *m_addItemBeforeAction, *m_addSubItemAction;
};

#endif // NAVIGATION_H

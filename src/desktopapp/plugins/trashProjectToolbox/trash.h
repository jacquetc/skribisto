#ifndef TRASH_H
#define TRASH_H

#include "toolbox.h"
#include <QModelIndex>
#include <QWidget>

namespace Ui {
class Trash;
}

typedef QPair<int, int> PathItem;
typedef QList<PathItem> Path;

class Trash : public Toolbox
{
    Q_OBJECT

public:
    explicit Trash(QWidget *parent = nullptr);
    ~Trash();
    QString title() const override
    {
        return tr("Trash");
    }
    QIcon icon() const override
    {
        return QIcon(":/icons/backup/edit-delete.svg");
    }

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


    Ui::Trash *ui;
    QList<Path> m_expandedPathes;
    TreeItemAddress m_targetTreeItemAddress;
    int m_projectId;
    QModelIndex m_currentModelIndex;
    QAction *m_openItemAction, *m_openItemInAnotherViewAction, *m_openItemInANewWindowAction, *m_renameAction, *m_restoreAction,
    *m_empyTrashAction;

    QList< QPair<int, int>> copyCutList;

    void open(const QModelIndex &index);
    void openInAnotherView(const QModelIndex &index);
protected:

    // Toolbox interface
public:
    void initialize() override;
};

#endif // TRASH_H

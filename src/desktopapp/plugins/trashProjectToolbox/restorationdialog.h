#ifndef RESTORATIONDIALOG_H
#define RESTORATIONDIALOG_H

#include <QDialog>
#include <QSortFilterProxyModel>
#include "interfaces/pagetypeiconinterface.h"


class FilterModel;

namespace Ui {
class RestorationDialog;
}

class RestorationDialog : public QDialog
{
    Q_OBJECT

public:
    explicit RestorationDialog(QWidget *parent = nullptr);
    ~RestorationDialog();

    const QList<TreeItemAddress> &notRestorableIds() const;
    void setNotRestorableIds(const QList<TreeItemAddress> &newNotRestorableIds);

    int projectId() const;
    void setProjectId(int newProjectId);

private:
    Ui::RestorationDialog *ui;
    QList<TreeItemAddress> m_notRestorableIds;
    int m_projectId;
    QList< QPair<TreeItemAddress, TreeItemAddress> > m_notRestorableIdForTargetFolderList;
    QHash<QString, PageTypeIconInterface *> m_typeWithPlugin;
    FilterModel *m_treeModel;

    void reset();

    // QDialog interface
public slots:
    void accept() override;
    void reject() override;


    // QDialog interface
public slots:
    void open() override;
};


//----------------------------------------

class FilterModel : public QSortFilterProxyModel
{

public:
    FilterModel(QObject *parent = nullptr);



    // QSortFilterProxyModel interface
    int projectId() const;
    void setProjectId(int newProjectId);

    QModelIndex getModelIndex(const TreeItemAddress &treeItemAddress) const;
protected:
    bool filterAcceptsRow(int source_row, const QModelIndex &source_parent) const override;

private:
    int m_projectId;

    // QAbstractItemModel interface
public:
    int columnCount(const QModelIndex &parent) const override;
};

#endif // RESTORATIONDIALOG_H

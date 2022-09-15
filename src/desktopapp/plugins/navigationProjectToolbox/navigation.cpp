#include "navigation.h"
#include "ui_navigation.h"
#include "projecttreeproxymodel.h"
#include "projecttreeitem.h"
#include "projecttreecommands.h"
#include "invoker.h"
#include "viewmanager.h"
#include "plmprojecthub.h"
#include "projectitemdelegate.h"

#include <QMenu>
#include <newtreeitemdialog.h>

Navigation::Navigation(class QWidget *parent) :
    ui(new Ui::Navigation)
{
    ui->setupUi(this);


    QWidget *viewPort = new QWidget();

    ui->treeView->setViewport(viewPort);


    ProjectTreeProxyModel *model = new ProjectTreeProxyModel(this);
    ui->treeView->setModel(model);


    ui->treeView->setContextMenuPolicy(Qt::CustomContextMenu);
    connect(ui->treeView, &QTreeView::customContextMenuRequested, this, &Navigation::onCustomContextMenu);

    //expand management
    connect(ui->treeView, &QTreeView::expanded, this, &Navigation::addToExpandPathes);
    connect(ui->treeView, &QTreeView::collapsed, this, &Navigation::removeFromExpandPathes);

    m_addItemAfterAction = new QAction("Add below", this);


    expandProjectItems();
    QObject::connect(model, &ProjectTreeProxyModel::modelReset, this, &Navigation::expandProjectItems);


    QObject::connect(ui->treeView, &QTreeView::clicked, this, [this](const QModelIndex &index){
        if (index.isValid()) {
            m_projectId = index.data(ProjectTreeItem::ProjectIdRole).toInt();
            m_targetTreeItemId = index.data(ProjectTreeItem::TreeItemIdRole).toInt();
            m_currentModelIndex = index;

            QString type = index.data(ProjectTreeItem::TypeRole).toString();

            ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
            viewManager->openViewAtCurrentView(type, m_projectId, m_targetTreeItemId);

        }
    });

    NewTreeItemDialog *dialog = new NewTreeItemDialog(this);

    QObject::connect(m_addItemAfterAction, &QAction::triggered, this, [this, dialog](){
        dialog->setActionType(NewTreeItemDialog::AddAfter);
        dialog->setIdentifiers(m_projectId, m_targetTreeItemId);
        dialog->open();
    } );


    m_addItemBeforeAction = new QAction("Add above", this);
    QObject::connect(m_addItemBeforeAction, &QAction::triggered, this, [this, dialog](){
        dialog->setActionType(NewTreeItemDialog::AddBefore);
        dialog->setIdentifiers(m_projectId, m_targetTreeItemId);
        dialog->open();
    } );

    m_addSubItemAction = new QAction("Add sub-item", this);
    QObject::connect(m_addSubItemAction, &QAction::triggered, this, [this, dialog](){
        dialog->setActionType(NewTreeItemDialog::AddSubItem);
        dialog->setIdentifiers(m_projectId, m_targetTreeItemId);
        dialog->open();

        QObject::connect(dialog, &QDialog::finished, this, [this](int result){
            if(result == QDialog::Accepted){
                ui->treeView->expand(m_currentModelIndex);
            }
        });

    } );


    ui->treeView->setItemDelegate(new ProjectItemDelegate);

}

Navigation::~Navigation()
{
    delete ui;
}

QIcon Navigation::icon() const
{
return QIcon(":/icons/backup/compass.svg");
}

void Navigation::onCustomContextMenu(const QPoint &point)
{

    QModelIndex index = ui->treeView->indexAt(point);
    if (index.isValid()) {
        m_projectId = index.data(ProjectTreeItem::ProjectIdRole).toInt();
        m_targetTreeItemId = index.data(ProjectTreeItem::TreeItemIdRole).toInt();
        m_currentModelIndex = index;


        QMenu *contextMenu = new QMenu("Context menu", ui->treeView);
        contextMenu->setMinimumSize(100, 50);


        contextMenu->addAction(m_addItemBeforeAction);
        contextMenu->addAction(m_addItemAfterAction);
        contextMenu->addAction(m_addSubItemAction);

        contextMenu->exec(ui->treeView->viewport()->mapToGlobal(point));
    }
}

void Navigation::saveExpandStates()
{

}

void Navigation::restoreExpandStates()
{
    ui->treeView->setAnimated(false);

    for(const auto &path : m_expandedPathes){

        QModelIndex index = this->pathToIndex(path);
        if(index.isValid()){
            ui->treeView->expand(index);
        }
    }

    ui->treeView->setAnimated(true);

}

void Navigation::removeFromExpandPathes(const QModelIndex &modelIndex)
{
    m_expandedPathes.removeAll(this->pathFromIndex(modelIndex));

}

void Navigation::expandProjectItems()
{
    for(int projectId : skrdata->projectHub()->getProjectIdList()){
        auto modelIndex = static_cast<ProjectTreeProxyModel *>(ui->treeView->model())->getModelIndex(projectId, 0);
        if(modelIndex.isValid()){
            ui->treeView->expand(modelIndex);
        }
    }
}

void Navigation::addToExpandPathes(const QModelIndex &modelIndex)
{
    m_expandedPathes.append(this->pathFromIndex(modelIndex));
}



Path Navigation::pathFromIndex(const QModelIndex &index){
  QModelIndex iter = index;
  Path path;
  while(iter.isValid()){
    path.prepend(PathItem(iter.row(), iter.column()));
    iter = iter.parent();
  }
  return path;
}

QModelIndex Navigation::pathToIndex(const Path &path){
  QModelIndex iter;
  for(int i=0;i<path.size();i++){
              iter = projectTreeModel->index(path[i].first,
                                  path[i].second, iter);
  }
  return iter;
}

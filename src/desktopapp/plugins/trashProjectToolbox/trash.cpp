#include "trash.h"
#include "skrusersettings.h"
#include "ui_trash.h"
#include "treemodels/projecttrashedtreemodel.h"
#include "projecttreecommands.h"
#include "invoker.h"
#include "viewmanager.h"
#include "plmprojecthub.h"
#include "projectitemdelegate.h"
#include "restorationdialog.h"

#include <QKeyEvent>
#include <QMenu>
#include <QInputDialog>
#include <QMessageBox>

Trash::Trash(class QWidget *parent) :
    ui(new Ui::Trash), m_projectId(skrdata->projectHub()->getActiveProject())
{
    ui->setupUi(this);

    ProjectTrashedTreeModel *model = new ProjectTrashedTreeModel(this);
    ui->treeView->setModel(model);


    ui->treeView->setContextMenuPolicy(Qt::CustomContextMenu);
    connect(ui->treeView, &QTreeView::customContextMenuRequested, this, &Trash::onCustomContextMenu);

    model->setProjectId(m_projectId);

    QObject::connect(skrdata->projectHub(), &PLMProjectHub::activeProjectChanged, this, [this, model](int projectId){
        m_projectId = projectId;
        model->setProjectId(projectId);
    });


    //expand management

    expandProjectItems();
    QObject::connect(model, &ProjectTrashedTreeModel::modelReset, this, &Trash::expandProjectItems);

    restoreExpandStates();
    QObject::connect(model, &ProjectTrashedTreeModel::modelReset, this, &Trash::restoreExpandStates);

    QObject::connect(skrdata->projectHub(), &PLMProjectHub::projectToBeClosed, this, &Trash::saveExpandStates);

    //QObject::connect(ui->treeView, &QTreeView::clicked, this, &Trash::setCurrentIndex);
    QObject::connect(ui->treeView, &QTreeView::activated, this, [this](const QModelIndex &index){
        this->setCurrentIndex(index);
        this->open(index);
    });

//    m_openItemAction = new QAction(tr("Open"), ui->treeView);
//    m_openItemAction->setShortcut(QKeySequence(Qt::Key_Return));
//    m_openItemAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
//    QObject::connect(m_openItemAction, &QAction::triggered, this, [this](){
//        QModelIndex index = ui->treeView->selectionModel()->currentIndex();
//            this->setCurrentIndex(index);
//    } );
    m_openItemInAnotherViewAction = new QAction(tr("Open in another view"), ui->treeView);
    ui->treeView->addAction(m_openItemInAnotherViewAction);
    m_openItemInAnotherViewAction->setShortcut(QKeySequence("Ctrl+Return"));
    m_openItemInAnotherViewAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_openItemInAnotherViewAction, &QAction::triggered, this, [this](){
        QModelIndex index = ui->treeView->selectionModel()->currentIndex();
        this->setCurrentIndex(index);
        this->openInAnotherView(index);
    } );


    m_openItemInANewWindowAction = new QAction(tr("Open in a new window"), ui->treeView);
    ui->treeView->addAction(m_openItemInANewWindowAction);
    m_openItemInANewWindowAction->setShortcut(QKeySequence("Ctrl+Shift+Return"));
    m_openItemInANewWindowAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_openItemInANewWindowAction, &QAction::triggered, this, [this](){
        QModelIndex index = ui->treeView->selectionModel()->currentIndex();
        this->setCurrentIndex(index);


        QMetaObject::invokeMethod(this->window(), "addWindowForItemId", Qt::QueuedConnection
                                  , Q_ARG(TreeItemAddress, m_targetTreeItemAddress)
                                  );


    } );



    m_renameAction = new QAction(tr("Rename"), ui->treeView);
    ui->treeView->addAction(m_renameAction);
    m_renameAction->setShortcut(QKeySequence("F2"));
    m_renameAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_renameAction, &QAction::triggered, this, [this](){
        this->setCurrentIndex(ui->treeView->selectionModel()->currentIndex());

        QString newName = QInputDialog::getText(ui->treeView, tr("Rename"), tr("Enter a new title for the item"), QLineEdit::Normal,
                                                skrdata->treeHub()->getTitle(m_targetTreeItemAddress));
        projectTreeCommands->renameItem(m_targetTreeItemAddress, newName);

    } );

    RestorationDialog *restorationDialog = new RestorationDialog(this);

    m_restoreAction = new QAction(tr("Restore"), ui->treeView);
    ui->treeView->addAction(m_restoreAction);
    m_restoreAction->setShortcut(QKeySequence("Ctrl+Del"));
    m_restoreAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_restoreAction, &QAction::triggered, this, [this, restorationDialog](){

        this->setCurrentIndex(ui->treeView->selectionModel()->currentIndex());
        QModelIndexList indexList = ui->treeView->selectionModel()->selectedIndexes();

        if(indexList.size() == 1){
               TreeItemAddress notRestorableId =  projectTreeCommands->restoreItemFromTrash(m_targetTreeItemAddress);

            if(notRestorableId.isValid()){
                restorationDialog->setProjectId(m_projectId);
                restorationDialog->setNotRestorableIds(QList<TreeItemAddress>() << notRestorableId);
                restorationDialog->open();
            }

        }
        else{

            QList<TreeItemAddress> targetIds;
            for(const QModelIndex &index : indexList){
                targetIds.append(index.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>());
            }

            QList<TreeItemAddress> notRestorableIdList =  projectTreeCommands->restoreSeveralItemsFromTrash(targetIds);


            if(!notRestorableIdList.empty()){
                restorationDialog->setProjectId(m_projectId);
                restorationDialog->setNotRestorableIds(notRestorableIdList);
                restorationDialog->open();
            }

        }



    } );


    m_empyTrashAction = new QAction(tr("Empty trash"), this);
    m_empyTrashAction->setIcon(QIcon(":/icons/backup/edit-delete.svg"));
    m_empyTrashAction->setShortcut(QKeySequence("Ctrl+Shift+Del"));
    m_empyTrashAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_empyTrashAction, &QAction::triggered, this, [this](){


        int ret = QMessageBox::warning(this, tr("Empty trash"), tr("All trashed items will be deleted deinitively.\nThis action isn't recoverable."),
                             QMessageBox::Ok | QMessageBox::Cancel, QMessageBox::Cancel);
        switch (ret) {
          case QMessageBox::Ok:
              // Ok was clicked
                projectTreeCommands->emptyTrash(m_projectId);

              break;
          case QMessageBox::Cancel:
              // Cancel was clicked
              break;
          default:
              // should never be reached
              break;
        }


    });


    ui->treeView->setItemDelegate(new ProjectItemDelegate);


    QMenu *trashMenu = new QMenu;
    trashMenu->addAction(m_empyTrashAction);

    ui->trashMenuToolButton->setMenu(trashMenu);
}

Trash::~Trash()
{
    delete ui;
}



void Trash::onCustomContextMenu(const QPoint &point)
{


    QModelIndex index = ui->treeView->indexAt(point);
    if (index.isValid()) {
        this->setCurrentIndex(index);

        QMenu *contextMenu = new QMenu("Context menu", ui->treeView);
        contextMenu->setMinimumSize(100, 50);
        contextMenu->addAction(m_openItemInAnotherViewAction);
        contextMenu->addAction(m_openItemInANewWindowAction);
        contextMenu->addSeparator();
        contextMenu->addAction(m_renameAction);
        contextMenu->addSeparator();
        contextMenu->addAction(m_restoreAction);

        contextMenu->exec(ui->treeView->viewport()->mapToGlobal(point));
    }
}

void Trash::saveExpandStates()
{
    ui->treeView->selectAll();
    m_expandedPathes.clear();
    for (const QModelIndex &index : ui->treeView->selectionModel()->selection().indexes()){
        if(ui->treeView->isExpanded(index)){
            addToExpandPathes(index);
        }
    }
    ui->treeView->clearSelection();

    QByteArray byteArray;
    QDataStream stream(&byteArray, QIODevice::WriteOnly);
    stream << m_expandedPathes;

    SKRUserSettings::setProjectSetting(skrdata->projectHub()->getActiveProject(), "trashTreeExpandState", byteArray);
}

void Trash::restoreExpandStates()
{

    if( skrdata->projectHub()->getProjectCount() == 0){
        return;
    }

    QByteArray byteArray = SKRUserSettings::getProjectSetting(skrdata->projectHub()->getActiveProject(), "trashTreeExpandState", 0).toByteArray();
    QDataStream stream(&byteArray, QIODevice::ReadOnly);
    stream >> m_expandedPathes;


    ui->treeView->setAnimated(false);

    for(const auto &path : m_expandedPathes){

        QModelIndex index = this->pathToIndex(path);
        if(index.isValid()){
            ui->treeView->expand(index);
        }
    }

    ui->treeView->setAnimated(true);

}


void Trash::expandProjectItems()
{
    for(int projectId : skrdata->projectHub()->getProjectIdList()){
        auto modelIndex = static_cast<ProjectTrashedTreeModel *>(ui->treeView->model())->getModelIndex(TreeItemAddress(projectId, 0));
        if(modelIndex.isValid()){
            ui->treeView->expand(modelIndex);
        }
    }
}

void Trash::addToExpandPathes(const QModelIndex &modelIndex)
{
    m_expandedPathes.append(this->pathFromIndex(modelIndex));
}



Path Trash::pathFromIndex(const QModelIndex &index){
  QModelIndex iter = index;
  Path path;
  while(iter.isValid()){
    path.prepend(PathItem(iter.row(), iter.column()));
    iter = iter.parent();
  }
  return path;
}

QModelIndex Trash::pathToIndex(const Path &path){
  QModelIndex iter;
  for(int i=0;i<path.size();i++){
              iter = ui->treeView->model()->index(path[i].first,
                                  path[i].second, iter);
  }
  return iter;
}


//--------------------------------------------------------------

void Trash::setCurrentIndex(const QModelIndex &index)
{
    if (index.isValid()) {

        m_projectId = index.data(ProjectTreeItem::ProjectIdRole).toInt();
        m_targetTreeItemAddress = index.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>();
        m_currentModelIndex = index;

        //project item
//        if(m_targetTreeItemId){

//        }

    }
}

//--------------------------------------------------------------

void Trash::open(const QModelIndex &index)
{
    if (index.isValid()) {
        QString type = index.data(ProjectTreeItem::TypeRole).toString();

        ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
        viewManager->openViewAtCurrentViewHolder(type, m_targetTreeItemAddress);

    }
}
//--------------------------------------------------------------

void Trash::openInAnotherView(const QModelIndex &index)
{
    if (index.isValid()) {
        QString type = index.data(ProjectTreeItem::TypeRole).toString();

        ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
        viewManager->openViewInAnotherViewHolder(type, m_targetTreeItemAddress);

    }
}


void Trash::initialize()
{
}

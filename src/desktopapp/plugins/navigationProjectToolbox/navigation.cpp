#include "navigation.h"
#include "projectcommands.h"
#include "skrusersettings.h"
#include "ui_navigation.h"
#include "treemodels/projecttreeproxymodel.h"
#include "projecttreecommands.h"
#include "invoker.h"
#include "viewmanager.h"
#include "plmprojecthub.h"
#include "projectitemdelegate.h"

#include <QKeyEvent>
#include <QMenu>
#include <QInputDialog>
#include <newtreeitemdialog.h>

Navigation::Navigation(class QWidget *parent) :
    ui(new Ui::Navigation)
{
    ui->setupUi(this);

    this->setFocusProxy(ui->treeView);

    ProjectTreeProxyModel *model = new ProjectTreeProxyModel(this);
    FilterModel *filterModel = new FilterModel(this);
    filterModel->setSourceModel(model);
    ui->treeView->setModel(model);
    ui->treeView->setColumnHidden(1, true);
    ui->treeView->setColumnHidden(2, true);
    ui->treeView->setColumnHidden(3, true);


    ui->treeView->setContextMenuPolicy(Qt::CustomContextMenu);
    connect(ui->treeView, &QTreeView::customContextMenuRequested, this, &Navigation::onCustomContextMenu);

    //expand management

    expandProjectItems();

    restoreExpandStates();
    QObject::connect(model, &ProjectTreeProxyModel::modelReset, this, &Navigation::restoreExpandStates);
    QObject::connect(model, &ProjectTreeProxyModel::modelReset, this, &Navigation::expandProjectItems);

    QObject::connect(skrdata->projectHub(), &PLMProjectHub::projectToBeClosed, this, &Navigation::saveExpandStates);

    //QObject::connect(ui->treeView, &QTreeView::clicked, this, &Navigation::setCurrentIndex);
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

        if(!ui->treeView->selectionModel()->currentIndex().isValid()){
            return;
        }

        QModelIndex index = ui->treeView->selectionModel()->currentIndex();
        this->setCurrentIndex(index);
        this->openInAnotherView(index);
    } );


    m_openItemInANewWindowAction = new QAction(tr("Open in a new window"), ui->treeView);
    ui->treeView->addAction(m_openItemInANewWindowAction);
    m_openItemInANewWindowAction->setShortcut(QKeySequence("Ctrl+Shift+Return"));
    m_openItemInANewWindowAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_openItemInANewWindowAction, &QAction::triggered, this, [this](){
        if(!ui->treeView->selectionModel()->currentIndex().isValid()){
            return;
        }

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
        if(!ui->treeView->selectionModel()->currentIndex().isValid()){
            return;
        }

        this->setCurrentIndex(ui->treeView->selectionModel()->currentIndex());

        bool ok;
        QString newName = QInputDialog::getText(ui->treeView, tr("Rename"), tr("Enter a new title for the item"), QLineEdit::Normal,
                                                skrdata->treeHub()->getTitle(m_targetTreeItemAddress), &ok);

        if(ok){
            projectTreeCommands->renameItem(m_targetTreeItemAddress, newName);
        }

    } );


    NewTreeItemDialog *addItemDialog = new NewTreeItemDialog(this);

    m_addItemAfterAction = new QAction(tr("Add below"), ui->treeView);
    ui->treeView->addAction(m_addItemAfterAction);
    m_addItemAfterAction->setShortcut(QKeySequence("Ctrl+N"));
    m_addItemAfterAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_addItemAfterAction, &QAction::triggered, this, [this, addItemDialog](){

        if(!ui->treeView->selectionModel()->currentIndex().isValid()){
            return;
        }

        this->setCurrentIndex(ui->treeView->selectionModel()->currentIndex());
        addItemDialog->setActionType(NewTreeItemDialog::AddAfter);
        addItemDialog->setIdentifiers(m_targetTreeItemAddress);
        addItemDialog->open();
    } );


    m_addItemBeforeAction = new QAction(tr("Add above"), ui->treeView);
    ui->treeView->addAction(m_addItemBeforeAction);
    m_addItemBeforeAction->setShortcut(QKeySequence("Ctrl+H"));
    m_addItemBeforeAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_addItemBeforeAction, &QAction::triggered, this, [this, addItemDialog](){
        if(!ui->treeView->selectionModel()->currentIndex().isValid()){
            return;
        }

        this->setCurrentIndex(ui->treeView->selectionModel()->currentIndex());
        addItemDialog->setActionType(NewTreeItemDialog::AddBefore);
        addItemDialog->setIdentifiers(m_targetTreeItemAddress);
        addItemDialog->open();
    } );

    m_addSubItemAction = new QAction(tr("Add sub-item"), this);
    ui->treeView->addAction(m_addSubItemAction);
    m_addSubItemAction->setShortcut(QKeySequence("Ctrl+J"));
    m_addSubItemAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_addSubItemAction, &QAction::triggered, this, [this, addItemDialog](){

        if(!ui->treeView->selectionModel()->currentIndex().isValid()){
            return;
        }

        this->setCurrentIndex(ui->treeView->selectionModel()->currentIndex());

        addItemDialog->setActionType(NewTreeItemDialog::AddSubItem);
        addItemDialog->setIdentifiers(m_targetTreeItemAddress);
        addItemDialog->open();

        QObject::connect(addItemDialog, &QDialog::finished, this, [this](int result){
            if(result == QDialog::Accepted){
            //auto modelIndex = static_cast<FilterModel *>(ui->treeView->model())->getModelIndex(m_projectId, m_targetTreeItemAddress);
            ui->treeView->expand(m_currentModelIndex);
            }
        });

    } );



    m_sendToTrashAction = new QAction(tr("Send to trash"), ui->treeView);
    ui->treeView->addAction(m_sendToTrashAction);
    m_sendToTrashAction->setShortcut(QKeySequence::Delete);
    m_sendToTrashAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_sendToTrashAction, &QAction::triggered, this, [this](){

        if(!ui->treeView->selectionModel()->currentIndex().isValid()){
            return;
        }

        this->setCurrentIndex(ui->treeView->selectionModel()->currentIndex());
        QModelIndexList indexList = ui->treeView->selectionModel()->selectedIndexes();

        if(indexList.size() == 1){
            projectTreeCommands->sendItemToTrash(m_targetTreeItemAddress);

        }
        else{
            //TODO: forbid selection accross multiple projects

            QList<TreeItemAddress> targetIds;
            for(const QModelIndex &index : indexList){
                targetIds.append(index.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>());
            }

            projectTreeCommands->sendSeveralItemsToTrash(targetIds);
        }


    } );


    m_exportAction = new QAction(tr("Export"), this);
    ui->treeView->addAction(m_exportAction);
    m_exportAction->setShortcut(QKeySequence("Ctrl+E"));
    m_exportAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_exportAction, &QAction::triggered, this, [this](){
        if(!ui->treeView->selectionModel()->currentIndex().isValid()){
            return;
        }
        this->setCurrentIndex(ui->treeView->selectionModel()->currentIndex());



    } );


    m_setActiveProjectAction = new QAction(tr("Set as the active project"), this);
    ui->treeView->addAction(m_setActiveProjectAction);
    m_setActiveProjectAction->setShortcut(QKeySequence("Ctrl+A"));
    m_setActiveProjectAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_setActiveProjectAction, &QAction::triggered, this, [this](){
        if(!ui->treeView->selectionModel()->currentIndex().isValid()){
            return;
        }

        this->setCurrentIndex(ui->treeView->selectionModel()->currentIndex());

        projectCommands->setActiveProject(m_targetTreeItemAddress.projectId);

    } );

//    m_copyItemsAction = new QAction(tr("Copy"), this);
//    ui->treeView->addAction(m_copyItemsAction);
//    m_copyItemsAction->setShortcut(QKeySequence("Ctrl+C"));
//    m_copyItemsAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
//    QObject::connect(m_copyItemsAction, &QAction::triggered, this, [this](){


//        for(auto index : ui->treeView->selectionModel()->selection().indexes()){
//            copyCutIndex
//        }

//    } );


    connect(ui->treeView->selectionModel(), &QItemSelectionModel::selectionChanged, this, [this](){

        if(ui->treeView->selectionModel()->selectedIndexes().count() > 1){
          QModelIndexList indexList = ui->treeView->selectionModel()->selectedIndexes();

          int firstIndexProjectId = indexList.first().data(ProjectTreeItem::ProjectIdRole).toInt();

          for(const QModelIndex &index : indexList){
              TreeItemAddress treeItemAddress = index.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>();
              // forbid selection accross multiple projects
              if(treeItemAddress.projectId != firstIndexProjectId){
                  ui->treeView->selectionModel()->select(index, QItemSelectionModel::Deselect);
              }
              // deselect project item
              if(treeItemAddress.itemId == 0){
                  ui->treeView->selectionModel()->select(index, QItemSelectionModel::Deselect);
              }
          }
        }


    });

    ui->treeView->setItemDelegate(new ProjectItemDelegate);


    QMenu *navigationMenu = new QMenu;
    //navigationMenu->addAction(splitHorizontalyAction);

    ui->navigationMenuToolButton->setMenu(navigationMenu);
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
        this->setCurrentIndex(index);

        QMenu *contextMenu = new QMenu("Context menu", ui->treeView);
        contextMenu->setMinimumSize(100, 50);
        contextMenu->addAction(m_openItemInAnotherViewAction);
        contextMenu->addAction(m_openItemInANewWindowAction);
        contextMenu->addSeparator();
        contextMenu->addAction(m_renameAction);
        contextMenu->addSeparator();
        contextMenu->addAction(m_addItemBeforeAction);
        contextMenu->addAction(m_addItemAfterAction);
        contextMenu->addAction(m_addSubItemAction);
        if(m_targetTreeItemAddress.itemId > 0){
            contextMenu->addSeparator();
            contextMenu->addAction(m_sendToTrashAction);
        }
        if(m_targetTreeItemAddress.itemId == 0){

            contextMenu->addSeparator();
            contextMenu->addAction(m_setActiveProjectAction);
        }


        contextMenu->exec(ui->treeView->viewport()->mapToGlobal(point));
    }
}

void Navigation::saveExpandStates()
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

    SKRUserSettings::setProjectSetting(skrdata->projectHub()->getActiveProject(), "navigationTreeExpandState", byteArray);
}

void Navigation::restoreExpandStates()
{

    if( skrdata->projectHub()->getProjectCount() == 0){
        return;
    }

    QByteArray byteArray = SKRUserSettings::getProjectSetting(skrdata->projectHub()->getActiveProject(), "navigationTreeExpandState", 0).toByteArray();
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


void Navigation::expandProjectItems()
{
    for(int projectId : skrdata->projectHub()->getProjectIdList()){
        auto modelIndex = static_cast<ProjectTreeProxyModel *>(ui->treeView->model())->getModelIndex(TreeItemAddress(projectId, 0));
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
              iter = ui->treeView->model()->index(path[i].first,
                                  path[i].second, iter);
  }
  return iter;
}


//--------------------------------------------------------------

void Navigation::setCurrentIndex(const QModelIndex &index)
{
    if (index.isValid()) {

        m_targetTreeItemAddress = index.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>();
        m_currentModelIndex = index;

        m_addItemAfterAction->setEnabled(index.data(ProjectTreeItem::CanAddSiblingTreeItemRole).toBool());
        m_addItemBeforeAction->setEnabled(index.data(ProjectTreeItem::CanAddSiblingTreeItemRole).toBool());
        m_addSubItemAction->setEnabled(index.data(ProjectTreeItem::CanAddChildTreeItemRole).toBool());
        m_sendToTrashAction->setEnabled(index.data(ProjectTreeItem::IsTrashableRole).toBool());

        m_setActiveProjectAction->setEnabled(m_targetTreeItemAddress.projectId != skrdata->projectHub()->getActiveProject());

    }
}

//--------------------------------------------------------------

void Navigation::open(const QModelIndex &index)
{
    if (index.isValid()) {
        QString type = index.data(ProjectTreeItem::TypeRole).toString();

        ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
        viewManager->openViewAtCurrentViewHolder(type, m_targetTreeItemAddress);

    }
}
//--------------------------------------------------------------

void Navigation::openInAnotherView(const QModelIndex &index)
{
    if (index.isValid()) {
        QString type = index.data(ProjectTreeItem::TypeRole).toString();

        ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
        viewManager->openViewInAnotherViewHolder(type, m_targetTreeItemAddress);

    }
}


void Navigation::initialize()
{
}

//--------------------------------------------------------------
//--------------------------------------------------------------
//--------------------------------------------------------------


FilterModel::FilterModel(QObject *parent) : QSortFilterProxyModel(parent)
{

}

//----------------------------------------------------------

bool FilterModel::filterAcceptsRow(int source_row, const QModelIndex &source_parent) const
{

   const QModelIndex index = this->sourceModel()->index(source_row, 0, source_parent);

   if(!index.isValid()){
       return false;
   }

//    bool trashed = index.data(ProjectTreeItem::TrashedRole).toBool();
//    QString internalTitle = index.data(ProjectTreeItem::InternalTitleRole).toString();
//    QString type = index.data(ProjectTreeItem::TypeRole).toString();
//    int projectId = index.data(ProjectTreeItem::ProjectIdRole).toInt();

//    if(trashed){
//       return false;
//    }


//    if(type != "PROJECT" && type != "FOLDER"){
//       return false;
//    }

//    if(internalTitle == "trash_folder"){
//       return false;
//    }

    return true;

}


QModelIndex FilterModel::getModelIndex(const TreeItemAddress &treeItemAddress) const {
    // search for index
    QModelIndex index;
    QModelIndexList list =  this->match(this->index(0, 0,
                                                    QModelIndex()),
                                        ProjectTreeItem::Roles::TreeItemAddressRole,
                                        QVariant::fromValue(treeItemAddress),
                                        -1,
                                        Qt::MatchFlag::MatchRecursive |
                                        Qt::MatchFlag::MatchExactly |
                                        Qt::MatchFlag::MatchWrap);

    for (const QModelIndex& modelIndex : qAsConst(list)) {
        index = modelIndex;
    }

    if (index.isValid())
        return index;

    return QModelIndex();
}



int FilterModel::columnCount(const QModelIndex &parent) const
{
    return 1;
}

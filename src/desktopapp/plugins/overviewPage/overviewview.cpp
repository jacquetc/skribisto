#include "overviewview.h"
#include "invoker.h"
#include "projecttreecommands.h"
#include "skrdata.h"
#include "treemodels/projecttreeproxymodel.h"
#include "outlineitemdelegate.h"
#include "ui_overviewview.h"
#include "viewmanager.h"
#include "thememanager.h"

#include <QKeyEvent>
#include <QMenu>
#include <QInputDialog>
#include <newtreeitemdialog.h>
#include <skrusersettings.h>

#include <treemodels/projecttreeitem.h>

OverviewView::OverviewView(QWidget *parent) :
    View("OVERVIEW", parent),
    centralWidgetUi(new Ui::OverviewView)
{

    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);

    setCentralWidget(centralWidget);
    this->setFocusProxy(centralWidgetUi->treeView);


    ProjectTreeProxyModel *model = new ProjectTreeProxyModel(this);


    m_overviewProxyModel = new OverviewProxyModel(this);
    m_overviewProxyModel->setSourceModel(model);

    centralWidgetUi->treeView->setModel(m_overviewProxyModel);

    centralWidgetUi->treeView->setStyleSheet(R"""(QTreeView::item {
                                             border: 1px solid #d9d9d9;
                                            border-left-color: transparent;
                                            border-right-color: transparent;
                                            border-top-color: transparent;
                                        })""");

    connect(themeManager, &ThemeManager::currentThemeTypeChanged, this, [this](){

        centralWidgetUi->treeView->setStyleSheet(R"""(QTreeView::item {
                                                 border: 1px solid #d9d9d9;
                                                border-left-color: transparent;
                                                border-right-color: transparent;
                                                border-top-color: transparent;
                                            })""");
    });

    OutlineItemDelegate *outlineItemDelegate = new OutlineItemDelegate(centralWidgetUi->treeView);
    centralWidgetUi->treeView->setItemDelegateForColumn(1, outlineItemDelegate);


    connect(outlineItemDelegate, &OutlineItemDelegate::editFinished,  this, [this](const QModelIndex &index){
        centralWidgetUi->treeView->dataChanged(index, index, QList<int>() << Qt::ItemDataRole::SizeHintRole);
    });


    centralWidgetUi->treeView->setContextMenuPolicy(Qt::CustomContextMenu);
    connect(centralWidgetUi->treeView, &QTreeView::customContextMenuRequested, this, &OverviewView::onCustomContextMenu);


    //expand management

    //QObject::connect(centralWidgetUi->treeView, &QTreeView::expanded, this, &OverviewView::saveExpandStates, Qt::QueuedConnection);
    //QObject::connect(centralWidgetUi->treeView, &QTreeView::collapsed, this, &OverviewView::saveExpandStates, Qt::QueuedConnection);
    QObject::connect(model, &ProjectTreeProxyModel::modelReset, this, &OverviewView::expandProjectItems);

    QObject::connect(model, &ProjectTreeProxyModel::modelReset, this, &OverviewView::restoreExpandStates);

    QObject::connect(skrdata->projectHub(), &PLMProjectHub::projectToBeClosed, this, &OverviewView::saveExpandStates);

    //QObject::connect(centralWidgetUi->treeView, &QTreeView::clicked, this, &Navigation::setCurrentIndex);
    QObject::connect(centralWidgetUi->treeView, &QTreeView::activated, this, [this](const QModelIndex &index){
        this->setCurrentIndex(index);


        centralWidgetUi->treeView->edit(index);

    });



    m_openItemInAnotherViewAction = new QAction(tr("Open in another view"), centralWidgetUi->treeView);
    centralWidgetUi->treeView->addAction(m_openItemInAnotherViewAction);
    m_openItemInAnotherViewAction->setShortcut(QKeySequence("Ctrl+Return"));
    m_openItemInAnotherViewAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_openItemInAnotherViewAction, &QAction::triggered, this, [this](){
        QModelIndex index = centralWidgetUi->treeView->selectionModel()->currentIndex();
        this->setCurrentIndex(index);
        this->openInAnotherView(index);
    } );


    m_openItemInANewWindowAction = new QAction(tr("Open in a new window"), centralWidgetUi->treeView);
    centralWidgetUi->treeView->addAction(m_openItemInANewWindowAction);
    m_openItemInANewWindowAction->setShortcut(QKeySequence("Ctrl+Shift+Return"));
    m_openItemInANewWindowAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_openItemInANewWindowAction, &QAction::triggered, this, [this](){
        QModelIndex index = centralWidgetUi->treeView->selectionModel()->currentIndex();
        this->setCurrentIndex(index);


        QMetaObject::invokeMethod(this->window(), "addWindowForItemId", Qt::QueuedConnection
                                  , Q_ARG(TreeItemAddress, m_targetTreeItemAddress)
                                  );


    } );



    m_renameAction = new QAction(tr("Rename"), centralWidgetUi->treeView);
    centralWidgetUi->treeView->addAction(m_renameAction);
    m_renameAction->setShortcut(QKeySequence("F2"));
    m_renameAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_renameAction, &QAction::triggered, this, [this](){
        this->setCurrentIndex(centralWidgetUi->treeView->selectionModel()->currentIndex());
        bool ok;

        QString newName = QInputDialog::getText(centralWidgetUi->treeView, tr("Rename"), tr("Enter a new title for the item"), QLineEdit::Normal,
                                                skrdata->treeHub()->getTitle(m_targetTreeItemAddress), &ok);
        if(ok){
            projectTreeCommands->renameItem(m_targetTreeItemAddress, newName);
        }

    } );


    NewTreeItemDialog *addItemDialog = new NewTreeItemDialog(this);

    m_addItemAfterAction = new QAction(tr("Add below"), centralWidgetUi->treeView);
    centralWidgetUi->treeView->addAction(m_addItemAfterAction);
    m_addItemAfterAction->setShortcut(QKeySequence("Ctrl+N"));
    m_addItemAfterAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_addItemAfterAction, &QAction::triggered, this, [this, addItemDialog](){
        this->setCurrentIndex(centralWidgetUi->treeView->selectionModel()->currentIndex());
        addItemDialog->setActionType(NewTreeItemDialog::AddAfter);
        addItemDialog->setIdentifiers(m_targetTreeItemAddress);
        addItemDialog->open();
    } );


    m_addItemBeforeAction = new QAction(tr("Add above"), centralWidgetUi->treeView);
    centralWidgetUi->treeView->addAction(m_addItemBeforeAction);
    m_addItemBeforeAction->setShortcut(QKeySequence("Ctrl+H"));
    m_addItemBeforeAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_addItemBeforeAction, &QAction::triggered, this, [this, addItemDialog](){
        this->setCurrentIndex(centralWidgetUi->treeView->selectionModel()->currentIndex());
        addItemDialog->setActionType(NewTreeItemDialog::AddBefore);
        addItemDialog->setIdentifiers(m_targetTreeItemAddress);
        addItemDialog->open();
    } );

    m_addSubItemAction = new QAction(tr("Add sub-item"), this);
    centralWidgetUi->treeView->addAction(m_addSubItemAction);
    m_addSubItemAction->setShortcut(QKeySequence("Ctrl+J"));
    m_addSubItemAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_addSubItemAction, &QAction::triggered, this, [this, addItemDialog](){
        this->setCurrentIndex(centralWidgetUi->treeView->selectionModel()->currentIndex());

        addItemDialog->setActionType(NewTreeItemDialog::AddSubItem);
        addItemDialog->setIdentifiers(m_targetTreeItemAddress);
        addItemDialog->open();

        QObject::connect(addItemDialog, &QDialog::finished, this, [this](int result){
            if(result == QDialog::Accepted){
                centralWidgetUi->treeView->expand(m_currentModelIndex);
            }
        });

    } );



    m_sendToTrashAction = new QAction(tr("Send to trash"), centralWidgetUi->treeView);
    centralWidgetUi->treeView->addAction(m_sendToTrashAction);
    m_sendToTrashAction->setShortcut(QKeySequence("Del"));
    m_sendToTrashAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    QObject::connect(m_sendToTrashAction, &QAction::triggered, this, [this](){
        this->setCurrentIndex(centralWidgetUi->treeView->selectionModel()->currentIndex());
        QModelIndexList indexList = centralWidgetUi->treeView->selectionModel()->selectedIndexes();

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



    //    m_copyItemsAction = new QAction(tr("Copy"), this);
    //    centralWidgetUi->treeView->addAction(m_copyItemsAction);
    //    m_copyItemsAction->setShortcut(QKeySequence("Ctrl+C"));
    //    m_copyItemsAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    //    QObject::connect(m_copyItemsAction, &QAction::triggered, this, [this, addItemDialog](){


    //        for(auto index : centralWidgetUi->treeView->selectionModel()->selection().indexes()){
    //            copyCutIndex
    //        }

    //    } );


    connect(centralWidgetUi->treeView->selectionModel(), &QItemSelectionModel::selectionChanged, this, [this](){

        if(centralWidgetUi->treeView->selectionModel()->selectedIndexes().count() > 1){
            QModelIndexList indexList = centralWidgetUi->treeView->selectionModel()->selectedIndexes();

            int firstIndexProjectId = indexList.first().data(ProjectTreeItem::ProjectIdRole).toInt();

            for(const QModelIndex &index : indexList){
                // forbid selection accross multiple projects
                if(index.data(ProjectTreeItem::ProjectIdRole).toInt() != firstIndexProjectId){
                    centralWidgetUi->treeView->selectionModel()->select(index, QItemSelectionModel::Deselect);
                }
                // deselect project item
                if(index.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>().itemId == 0){
                    centralWidgetUi->treeView->selectionModel()->select(index, QItemSelectionModel::Deselect);
                }
            }
        }


    });


    connect(this, &View::aboutToBeDestroyed, this,  [this](){
        saveExpandStates();

    });


}

OverviewView::~OverviewView()
{    
    delete centralWidgetUi;
}

QList<Toolbox *> OverviewView::toolboxes()
{

    QList<Toolbox *> toolboxes;


    return toolboxes;
}

void OverviewView::initialize()
{
    if(this->projectId() > 0){
        m_overviewProxyModel->setProjectId(this->projectId());
        expandProjectItems();
        restoreExpandStates();

    }

    // history
    emit this->addToHistoryCalled(this, QVariantMap());

}

void OverviewView::onCustomContextMenu(const QPoint &point)
{


    QModelIndex index = centralWidgetUi->treeView->indexAt(point);
    if (index.isValid()) {
        this->setCurrentIndex(index);

        QMenu *contextMenu = new QMenu("Context menu", centralWidgetUi->treeView);
        contextMenu->setMinimumSize(100, 50);
        contextMenu->addAction(m_openItemInAnotherViewAction);
        contextMenu->addAction(m_openItemInANewWindowAction);
        contextMenu->addSeparator();
        contextMenu->addAction(m_renameAction);
        contextMenu->addSeparator();
        contextMenu->addAction(m_addItemBeforeAction);
        contextMenu->addAction(m_addItemAfterAction);
        contextMenu->addAction(m_addSubItemAction);
        contextMenu->addSeparator();
        contextMenu->addAction(m_sendToTrashAction);

        contextMenu->exec(centralWidgetUi->treeView->viewport()->mapToGlobal(point));
    }
}

void OverviewView::saveExpandStates()
{
    centralWidgetUi->treeView->selectAll();
    QList<Path>  expandedPathes;
    for (const QModelIndex &index : centralWidgetUi->treeView->selectionModel()->selectedRows(0)){
        if(centralWidgetUi->treeView->isExpanded(index)){
            expandedPathes.append(this->pathFromIndex(index));
        }
    }
    centralWidgetUi->treeView->clearSelection();

    QByteArray byteArray;
    QDataStream stream(&byteArray, QIODevice::WriteOnly);
    stream << expandedPathes;

    SKRUserSettings::setProjectSetting(skrdata->projectHub()->getActiveProject(), "overviewTreeExpandState", byteArray);
}

void OverviewView::restoreExpandStates()
{
    QList<Path> expandedPathes;
    QByteArray byteArray = SKRUserSettings::getProjectSetting(skrdata->projectHub()->getActiveProject(), "overviewTreeExpandState", 0).toByteArray();
    QDataStream stream(&byteArray, QIODevice::ReadOnly);
    stream >> expandedPathes;


    centralWidgetUi->treeView->setAnimated(false);

    for(const auto &path : expandedPathes){

        QModelIndex index = this->pathToIndex(path);
        if(index.isValid()){
            centralWidgetUi->treeView->expand(index);
        }
    }

    centralWidgetUi->treeView->setAnimated(true);

}


void OverviewView::expandProjectItems()
{
    centralWidgetUi->treeView->setAnimated(false);

    auto modelIndex = centralWidgetUi->treeView->indexAt(QPoint(1,1));
    //auto modelIndex = m_overviewProxyModel->getModelIndex(this->projectId(), 0);
    if(modelIndex.isValid()){
        centralWidgetUi->treeView->expand(modelIndex);
    }

    centralWidgetUi->treeView->setAnimated(true);
}

//TODO: redo the expand save/restore system

Path OverviewView::pathFromIndex(const QModelIndex &index){
    QModelIndex iter = index;
    Path path;
    while(iter.isValid()){
        path.prepend(PathItem(iter.row(), iter.column()));
        iter = iter.parent();
    }
    return path;
}

QModelIndex OverviewView::pathToIndex(const Path &path){
    QModelIndex iter;
    for(int i=0;i<path.size();i++){
        iter = centralWidgetUi->treeView->model()->index(path[i].first,
                                                         path[i].second, iter);
    }
    return iter;
}

//--------------------------------------------------------------

void OverviewView::setCurrentIndex(const QModelIndex &index)
{
    if (index.isValid()) {

        m_targetTreeItemAddress = index.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>();
        m_currentModelIndex = index;

        m_addItemAfterAction->setEnabled(index.data(ProjectTreeItem::CanAddSiblingTreeItemRole).toBool());
        m_addItemBeforeAction->setEnabled(index.data(ProjectTreeItem::CanAddSiblingTreeItemRole).toBool());
        m_addSubItemAction->setEnabled(index.data(ProjectTreeItem::CanAddChildTreeItemRole).toBool());
        m_sendToTrashAction->setEnabled(index.data(ProjectTreeItem::IsTrashableRole).toBool());

//        //project item
//        if(m_targetTreeItemAddress == 0){

//        }

    }
}


//--------------------------------------------------------------

void OverviewView::open(const QModelIndex &index)
{
    if (index.isValid()) {
        QString type = index.data(ProjectTreeItem::TypeRole).toString();

        ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
        viewManager->openViewAtCurrentViewHolder(type, m_targetTreeItemAddress);

    }
}
//--------------------------------------------------------------

void OverviewView::openInAnotherView(const QModelIndex &index)
{
    if (index.isValid()) {
        QString type = index.data(ProjectTreeItem::TypeRole).toString();

        ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
        viewManager->openViewInAnotherViewHolder(type, m_targetTreeItemAddress);

    }
}

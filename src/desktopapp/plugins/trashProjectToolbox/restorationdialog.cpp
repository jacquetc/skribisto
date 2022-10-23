#include "restorationdialog.h"
#include "ui_restorationdialog.h"
#include "projecttreecommands.h"

RestorationDialog::RestorationDialog(QWidget *parent) :
    QDialog(parent),
    ui(new Ui::RestorationDialog), m_treeModel(new FilterModel(this))
{
    ui->setupUi(this);


    QList<PageTypeIconInterface *> pluginList =
            skrpluginhub->pluginsByType<PageTypeIconInterface>();
    for(auto *plugin : pluginList){
        m_typeWithPlugin.insert(plugin->pageType(), plugin);
    }

    ui->treeView->setModel(m_treeModel);


    connect(ui->nextItemButton, &QPushButton::clicked, this, [this](){
        if(ui->treeView->selectionModel()->hasSelection()){
            ui->itemListWidget->currentItem()->setCheckState(Qt::Checked);
        }


        // select next row
        if(ui->itemListWidget->currentRow() < ui->itemListWidget->count()){
            ui->itemListWidget->setCurrentRow(ui->itemListWidget->currentRow() + 1);
        }

        bool ok;
        ui->itemListWidget->currentItem()->data(Qt::UserRole + 1).toInt(&ok);
        if(!ok){
            ui->itemListWidget->currentItem()->setCheckState(Qt::Unchecked);
            ui->treeView->clearSelection();
        }
    });


    connect(ui->applyToAllButton, &QPushButton::clicked, this, [this](){
        if(!ui->treeView->selectionModel()->hasSelection()){
            return;
        }

        int selectedTargetFolderId = ui->treeView->selectionModel()->selection().indexes().first().data(ProjectTreeItem::TreeItemIdRole).toInt();

        for(int row = 0 ; row < ui->itemListWidget->count(); row++){
            QListWidgetItem *item = ui->itemListWidget->item(row);
            item->setData(Qt::UserRole + 1, selectedTargetFolderId);
            item->setCheckState(Qt::Checked);
        }

    });


    connect(ui->treeView->selectionModel(), &QItemSelectionModel::selectionChanged, this, [this](const QItemSelection &selected, const QItemSelection &deselected){
        if(selected.empty()){
            ui->applyToAllButton->setEnabled(false);
            return;
        }

        int selectedTargetFolderId = selected.indexes().first().data(ProjectTreeItem::TreeItemIdRole).toInt();

        ui->itemListWidget->currentItem()->setData(Qt::UserRole + 1, selectedTargetFolderId);
        ui->itemListWidget->currentItem()->setCheckState(Qt::Checked);
        ui->applyToAllButton->setEnabled(true);
    });

    connect(ui->itemListWidget, &QListWidget::currentItemChanged, this, [this](QListWidgetItem *current, QListWidgetItem *previous){
        bool ok;
        int targetFolderId = current->data(Qt::UserRole + 1).toInt(&ok);
        if(ok){

            QModelIndex index = m_treeModel->getModelIndex(m_projectId, targetFolderId);
            if(index.isValid()){
                ui->treeView->selectionModel()->select(index, QItemSelectionModel::Select);
            }

        }
        else {
            ui->itemListWidget->currentItem()->setCheckState(Qt::Unchecked);
            ui->treeView->clearSelection();
        }

    });
}

RestorationDialog::~RestorationDialog()
{
    delete ui;
}

//----------------------------------------------------------

const QList<int> &RestorationDialog::notRestorableIds() const
{
    return m_notRestorableIds;
}

//----------------------------------------------------------

void RestorationDialog::setNotRestorableIds(const QList<int> &newNotRestorableIds)
{
    m_notRestorableIds = newNotRestorableIds;
}

//----------------------------------------------------------

int RestorationDialog::projectId() const
{
    return m_projectId;
}

//----------------------------------------------------------

void RestorationDialog::setProjectId(int newProjectId)
{
    m_projectId = newProjectId;
}

//----------------------------------------------------------

void RestorationDialog::reset()
{
    ui->applyToAllButton->setEnabled(false);

    // item list:

    for(int notRestorableId : m_notRestorableIds){

        QString text = skrdata->treeHub()->getTitle(m_projectId, notRestorableId);
        QIcon icon;
        auto *plugin = m_typeWithPlugin.value(skrdata->treeHub()->getType(m_projectId, notRestorableId), nullptr);
            if(plugin){
                icon = QIcon(plugin->pageTypeIconUrl(m_projectId, notRestorableId));
            }



        QListWidgetItem *item = new QListWidgetItem(icon, text,ui->itemListWidget);
        item->setFlags(Qt::ItemFlag::ItemIsEnabled | Qt::ItemFlag::ItemIsUserCheckable | Qt::ItemFlag::ItemIsSelectable);
        item->setData(Qt::UserRole, notRestorableId);
        item->setCheckState(Qt::Unchecked);
    }

    ui->itemListWidget->setCurrentRow(0);


    // tree view :
    m_treeModel->setProjectId(m_projectId);


    ui->treeView->expandAll();

}

//----------------------------------------------------------

void RestorationDialog::reject()
{

    QDialog::reject();
}

//----------------------------------------------------------

void RestorationDialog::accept()
{
    m_notRestorableIdForTargetFolderList.clear();

    for(int row = 0 ; row < ui->itemListWidget->count(); row++){
        QListWidgetItem *item = ui->itemListWidget->item(row);
        bool ok;
        item->data(Qt::UserRole + 1).toInt(&ok);
        if(item->checkState() == Qt::Checked && ok){
            QPair<int, int> pair(item->data(Qt::UserRole).toInt(), item->data(Qt::UserRole + 1).toInt());

            m_notRestorableIdForTargetFolderList.append(pair);
        }
    }




    for(QPair<int, int> notRestorableIdForTargetFolder : m_notRestorableIdForTargetFolderList){

        projectTreeCommands->restoreItemFromTrash(m_projectId, notRestorableIdForTargetFolder.first, notRestorableIdForTargetFolder.second);

    }
    QDialog::accept();
}

//----------------------------------------------------------


void RestorationDialog::open()
{
    reset();

    QDialog::open();
}

//----------------------------------------------------------
//----------------------------------------------------------
//----------------------------------------------------------


FilterModel::FilterModel(QObject *parent) : QSortFilterProxyModel(parent), m_projectId(-1)
{
    this->setSourceModel(projectTreeModel);

}

//----------------------------------------------------------

bool FilterModel::filterAcceptsRow(int source_row, const QModelIndex &source_parent) const
{
    if(m_projectId == -1){
        return false;
    }

    const QModelIndex index = this->sourceModel()->index(source_row, 0, source_parent);

    bool trashed = index.data(ProjectTreeItem::TrashedRole).toBool();
    QString internalTitle = index.data(ProjectTreeItem::InternalTitleRole).toString();
    QString type = index.data(ProjectTreeItem::TypeRole).toString();
    int projectId = index.data(ProjectTreeItem::ProjectIdRole).toInt();

    if(projectId != m_projectId){
       return false;
    }

    if(trashed){
       return false;
    }


    if(type != "PROJECT" && type != "FOLDER"){
       return false;
    }

    if(internalTitle == "trash_folder"){
       return false;
    }

    return true;

}

int FilterModel::projectId() const
{
    return m_projectId;
}

void FilterModel::setProjectId(int newProjectId)
{
    m_projectId = newProjectId;
    this->invalidateFilter();
}



QModelIndex FilterModel::getModelIndex(int projectId, int treeItemId) const {
    // search for index
    QModelIndex index;
    QModelIndexList list =  this->match(this->index(0, 0,
                                                    QModelIndex()),
                                        ProjectTreeItem::Roles::TreeItemIdRole,
                                        treeItemId,
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

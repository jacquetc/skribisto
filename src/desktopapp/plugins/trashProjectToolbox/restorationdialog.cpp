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
        if(ui->itemListWidget->currentRow() < ui->itemListWidget->count() - 1){
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

        TreeItemAddress selectedTargetFolderId = ui->treeView->selectionModel()->selection().indexes().first().data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>();

        for(int row = 0 ; row < ui->itemListWidget->count(); row++){
            QListWidgetItem *item = ui->itemListWidget->item(row);
            item->setData(Qt::UserRole + 1, QVariant::fromValue(selectedTargetFolderId));
            item->setCheckState(Qt::Checked);
        }

    });


    connect(ui->treeView->selectionModel(), &QItemSelectionModel::selectionChanged, this, [this](const QItemSelection &selected, const QItemSelection &deselected){
        if(selected.empty()){
            ui->applyToAllButton->setEnabled(false);
            return;
        }

        TreeItemAddress selectedTargetFolderId = selected.indexes().first().data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>();

        ui->itemListWidget->currentItem()->setData(Qt::UserRole + 1, QVariant::fromValue(selectedTargetFolderId));
        ui->itemListWidget->currentItem()->setCheckState(Qt::Checked);
        ui->applyToAllButton->setEnabled(true);
    });

    connect(ui->itemListWidget, &QListWidget::currentItemChanged, this, [this](QListWidgetItem *current, QListWidgetItem *previous){
        if(!current){
            return;
        }

        bool ok;
        TreeItemAddress targetFolderId = current->data(Qt::UserRole + 1).value<TreeItemAddress>();
        if(targetFolderId.isValid()){

            QModelIndex index = m_treeModel->getModelIndex(targetFolderId);
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

const QList<TreeItemAddress> &RestorationDialog::notRestorableIds() const
{
    return m_notRestorableIds;
}

//----------------------------------------------------------

void RestorationDialog::setNotRestorableIds(const QList<TreeItemAddress> &newNotRestorableIds)
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
    ui->itemListWidget->clear();

    // item list:

    for(const TreeItemAddress &notRestorableId : m_notRestorableIds){

        QString text = skrdata->treeHub()->getTitle(notRestorableId);
        QIcon icon;
        auto *plugin = m_typeWithPlugin.value(skrdata->treeHub()->getType(notRestorableId), nullptr);
            if(plugin){
                icon = QIcon(plugin->pageTypeIconUrl(notRestorableId));
            }



        QListWidgetItem *item = new QListWidgetItem(icon, text,ui->itemListWidget);
        item->setFlags(Qt::ItemFlag::ItemIsEnabled | Qt::ItemFlag::ItemIsUserCheckable | Qt::ItemFlag::ItemIsSelectable);
        item->setData(Qt::UserRole, QVariant::fromValue(notRestorableId));
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

        TreeItemAddress destinationFolderAddress = item->data(Qt::UserRole + 1).value<TreeItemAddress>();
        if(item->checkState() == Qt::Checked && destinationFolderAddress.isValid()){
            QPair<TreeItemAddress, TreeItemAddress> pair(item->data(Qt::UserRole).value<TreeItemAddress>(), destinationFolderAddress);

            m_notRestorableIdForTargetFolderList.append(pair);
        }
    }




    for(QPair<TreeItemAddress, TreeItemAddress> notRestorableIdForTargetFolder : m_notRestorableIdForTargetFolderList){

        projectTreeCommands->restoreItemFromTrash(notRestorableIdForTargetFolder.first, notRestorableIdForTargetFolder.second);

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

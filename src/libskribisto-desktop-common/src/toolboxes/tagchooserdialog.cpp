#include "tagchooserdialog.h"
#include "skrdata.h"
#include "tagcommands.h"
#include "tagdialog.h"
#include "thememanager.h"
#include "ui_tagchooserdialog.h"
#include "tagitemdelegate.h"

TagChooserDialog::TagChooserDialog(QWidget *parent, const TreeItemAddress &treeItemAddress) :
    QDialog(parent),
    ui(new Ui::TagChooserDialog), m_treeItemAddress(treeItemAddress), m_projectId(treeItemAddress.projectId)
{
    ui->setupUi(this);

    ui->itemTagsGroupBox->setTitle(skrdata->treeHub()->getTitle(m_treeItemAddress));

    ui->itemTagsListWidget->setItemDelegate(new TagItemDelegate(this));
    ui->availableTagsListWidget->setItemDelegate(new TagItemDelegate(this));


    connect(ui->removeTagToolButton, &QAbstractButton::clicked, this, [this](){
        QList<int> tagIdsToBeRemoved;
            for(auto *item : ui->itemTagsListWidget->selectedItems()){
                tagIdsToBeRemoved.append(item->data(Qt::UserRole).toInt());

            }

            if(!tagIdsToBeRemoved.isEmpty()){
                tagCommands->removeSeveralTags(m_projectId, tagIdsToBeRemoved);
            }

    });

    //--------------------------

    connect(ui->addTagToolButton, &QAbstractButton::clicked, this, [this]() {


      TagDialog dialog(this, m_projectId, -1, true);
      dialog.exec();
      reloadAvailableTags();

      if(dialog.newTagId() != -1){
          // add new tag to item
          tagCommands->setTagRelationship(m_treeItemAddress, dialog.newTagId());
          reloadItemTags();
      }
    });

    //--------------------------

    connect(ui->availableTagsListWidget, &QListWidget::itemActivated, this, [this](QListWidgetItem *item){
        TagDialog dialog(this, m_projectId, item->data(Qt::UserRole).toInt(), false);
        dialog.exec();
        reloadAvailableTags();
        reloadItemTags();

    });

    //--------------------------

    connect(ui->itemTagsListWidget, &QListWidget::itemActivated, this, [this](QListWidgetItem *item){
        TagDialog dialog(this, m_projectId, item->data(Qt::UserRole).toInt(), false);
        dialog.exec();
        reloadAvailableTags();
        reloadItemTags();

    });

    //--------------------------

    connect(ui->addToItemPushButton, &QAbstractButton::clicked, this, [this]() {

        QList<int> tagIdsToBeAdded;
        for(auto *item : ui->availableTagsListWidget->selectedItems()){
            tagIdsToBeAdded.append(item->data(Qt::UserRole).toInt());

        }

        if(!tagIdsToBeAdded.isEmpty()){
            tagCommands->setSeveralTagsRelationship(m_treeItemAddress, tagIdsToBeAdded);
        }

        reloadAvailableTags();
        reloadItemTags();
    });

    //--------------------------

    connect(ui->removeFromItemPushButton, &QAbstractButton::clicked, this, [this]() {

        QList<int> tagIdsToBeRemoved;
        for(auto *item : ui->itemTagsListWidget->selectedItems()){
            tagIdsToBeRemoved.append(item->data(Qt::UserRole).toInt());

        }

        if(!tagIdsToBeRemoved.isEmpty()){
            tagCommands->removeSeveralTagsRelationship(m_treeItemAddress, tagIdsToBeRemoved);
        }

        reloadAvailableTags();
        reloadItemTags();
    });

    //--------------------------

    connect(ui->removeAllPushButton, &QAbstractButton::clicked, this, [this]() {
        ui->itemTagsListWidget->selectAll();

        QList<int> tagIdsToBeRemoved;
        for(auto *item : ui->itemTagsListWidget->selectedItems()){
            tagIdsToBeRemoved.append(item->data(Qt::UserRole).toInt());

        }

        if(!tagIdsToBeRemoved.isEmpty()){
            tagCommands->removeSeveralTagsRelationship(m_treeItemAddress, tagIdsToBeRemoved);
        }

        reloadAvailableTags();
        reloadItemTags();
    });

    reloadAvailableTags();
    reloadItemTags();

    themeManager->scanChildrenAndAddWidgetsHoldingIcons(this);

}

//--------------------------------------------------


TagChooserDialog::~TagChooserDialog()
{
    delete ui;
}

//--------------------------------------------------

void TagChooserDialog::reloadAvailableTags()
{
    ui->availableTagsListWidget->clear();

    QList<int> itemTagIdsList = skrdata->tagHub()->getTagsFromItemId(m_treeItemAddress);

    for(int tagId : skrdata->tagHub()->getAllTagIds(m_projectId)){
        if(itemTagIdsList.contains(tagId)){
            continue;
        }

        auto item = new QListWidgetItem(skrdata->tagHub()->getTagName(m_projectId, tagId), ui->availableTagsListWidget);
        item->setData(Qt::UserRole, tagId);
        item->setData(Qt::UserRole + 1 , skrdata->tagHub()->getTagTextColor(m_projectId, tagId));
        item->setData(Qt::UserRole + 2, skrdata->tagHub()->getTagColor(m_projectId, tagId));
        item->setForeground(QBrush(QColor(skrdata->tagHub()->getTagTextColor(m_projectId, tagId))));
        item->setBackground(QBrush(QColor(skrdata->tagHub()->getTagColor(m_projectId, tagId))));
    }

}

//--------------------------------------------------


void TagChooserDialog::reloadItemTags()
{
    ui->itemTagsListWidget->clear();

    for (int tagId : skrdata->tagHub()->getTagsFromItemId(m_treeItemAddress)) {

      auto item = new QListWidgetItem(
          skrdata->tagHub()->getTagName(m_projectId, tagId), ui->itemTagsListWidget);
      item->setData(Qt::UserRole, tagId);

      item->setData(Qt::UserRole + 1 , skrdata->tagHub()->getTagTextColor(m_projectId, tagId));
      item->setData(Qt::UserRole + 2, skrdata->tagHub()->getTagColor(m_projectId, tagId));

      item->setForeground(
          QBrush(QColor(skrdata->tagHub()->getTagTextColor(m_projectId, tagId))));
      item->setBackground(
          QBrush(QColor(skrdata->tagHub()->getTagColor(m_projectId, tagId))));
    }
}

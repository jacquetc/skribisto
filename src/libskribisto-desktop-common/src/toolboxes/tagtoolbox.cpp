#include "skrdata.h"
#include "tagtoolbox.h"
#include "toolboxes/tagitemdelegate.h"
#include "ui_tagtoolbox.h"

#include "tagchooserdialog.h"
#include "tagcommands.h"

#include <tagdialog.h>

TagToolbox::TagToolbox(QWidget *parent, int projectId, int treeItemId)
    : Toolbox(parent), m_projectId(projectId), m_treeItemId(treeItemId), ui(new Ui::TagToolbox) {
  ui->setupUi(this);

  ui->listWidget->setItemDelegate(new TagItemDelegate(this));


  connect(skrdata->tagHub(), &SKRTagHub::tagAdded, this,
          &TagToolbox::reloadTags);
  connect(skrdata->tagHub(), &SKRTagHub::tagRemoved, this,
          &TagToolbox::reloadTags);
  connect(skrdata->tagHub(), &SKRTagHub::tagChanged, this,
          &TagToolbox::reloadTags);
  connect(skrdata->tagHub(), &SKRTagHub::tagRelationshipChanged, this,
          &TagToolbox::reloadTags);
  connect(skrdata->tagHub(), &SKRTagHub::tagRelationshipRemoved, this,
          &TagToolbox::reloadTags);

  //--------------------

  connect(ui->removeTagButton, &QAbstractButton::clicked, this, [this]() {
    QList<int> tagIdsToBeRemoved;
    for (auto *item : ui->listWidget->selectedItems()) {
      tagIdsToBeRemoved.append(item->data(Qt::UserRole).toInt());
    }

    if (!tagIdsToBeRemoved.isEmpty()) {
      tagCommands->removeSeveralTagsRelationship(m_projectId, m_treeItemId, tagIdsToBeRemoved);
    }
  });

  //--------------------

  connect(ui->addTagButton, &QAbstractButton::clicked, this, [this]() {

      TagChooserDialog dialog(this, m_projectId, m_treeItemId);
    dialog.exec();
    reloadTags();
  });

  //--------------------

  connect(ui->listWidget, &QListWidget::itemActivated, this,
          [this](QListWidgetItem *item) {
            TagDialog dialog(this, m_projectId,
                             item->data(Qt::UserRole).toInt(), false);
            dialog.exec();
            reloadTags();
          });

  //--------------------

  reloadTags();
}

//----------------------------------------------------------

TagToolbox::~TagToolbox() { delete ui; }

//----------------------------------------------------------


void TagToolbox::reloadTags() {
  ui->listWidget->clear();

  for (int tagId : skrdata->tagHub()->getTagsFromItemId(m_projectId, m_treeItemId)) {

    auto item = new QListWidgetItem(
        skrdata->tagHub()->getTagName(m_projectId, tagId), ui->listWidget);
    item->setData(Qt::UserRole, tagId);

    item->setData(Qt::UserRole + 1 , skrdata->tagHub()->getTagTextColor(m_projectId, tagId));
    item->setData(Qt::UserRole + 2, skrdata->tagHub()->getTagColor(m_projectId, tagId));

  }
}

//----------------------------------------------------------


void TagToolbox::initialize() {}

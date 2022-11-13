#include "tagmanagertoolbox.h"
#include "skrdata.h"
#include "toolboxes/tagitemdelegate.h"
#include "ui_tagmanagertoolbox.h"

#include "tagcommands.h"

#include <tagdialog.h>

TagManagerToolbox::TagManagerToolbox(QWidget *parent, int projectId) :
    Toolbox(parent),
    m_projectId(projectId),
    ui(new Ui::TagManagerToolbox)
{
    ui->setupUi(this);

    ui->listWidget->setItemDelegate(new TagItemDelegate(this));


    connect(skrdata->tagHub(), &SKRTagHub::tagAdded, this, &TagManagerToolbox::reloadTags);
    connect(skrdata->tagHub(), &SKRTagHub::tagRemoved, this, &TagManagerToolbox::reloadTags);
    connect(skrdata->tagHub(), &SKRTagHub::tagChanged, this, &TagManagerToolbox::reloadTags);

    connect(ui->removeTagButton, &QAbstractButton::clicked, this, [this](){
        QList<int> tagIdsToBeRemoved;
        for(auto *item : ui->listWidget->selectedItems()){
            tagIdsToBeRemoved.append(item->data(Qt::UserRole).toInt());
        }

        if(!tagIdsToBeRemoved.isEmpty()){
            tagCommands->removeSeveralTags(m_projectId, tagIdsToBeRemoved);
        }

    });

    connect(ui->addTagButton, &QAbstractButton::clicked, this, [this](){
        TagDialog dialog(this, m_projectId, -1, true);
        dialog.exec();
        reloadTags();

    });


    connect(ui->listWidget, &QListWidget::itemActivated, this, [this](QListWidgetItem *item){
        TagDialog dialog(this, m_projectId, item->data(Qt::UserRole).toInt(), false);
        dialog.exec();
        reloadTags();

    });



    reloadTags();
}

TagManagerToolbox::~TagManagerToolbox()
{
    delete ui;
}

void TagManagerToolbox::reloadTags()
{
    ui->listWidget->clear();

    for(int tagId : skrdata->tagHub()->getAllTagIds(m_projectId)){
            auto item = new QListWidgetItem(skrdata->tagHub()->getTagName(m_projectId, tagId), ui->listWidget);
            item->setData(Qt::UserRole, tagId);
            item->setData(Qt::UserRole + 1 , skrdata->tagHub()->getTagTextColor(m_projectId, tagId));
            item->setData(Qt::UserRole + 2, skrdata->tagHub()->getTagColor(m_projectId, tagId));

            item->setForeground(QBrush(QColor(skrdata->tagHub()->getTagTextColor(m_projectId, tagId))));
            item->setBackground(QBrush(QColor(skrdata->tagHub()->getTagColor(m_projectId, tagId))));
    }

}







void TagManagerToolbox::initialize()
{
}

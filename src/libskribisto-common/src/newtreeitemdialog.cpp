#include "newtreeitemdialog.h"
#include "ui_newtreeitemdialog.h"
#include "projecttreecommands.h"

#include "interfaces/pageinterface.h"

NewTreeItemDialog::NewTreeItemDialog(QWidget *parent) :
    QDialog(parent),
    ui(new Ui::NewTreeItemDialog), m_customPropertiesWidget(nullptr)
{
    ui->setupUi(this);

    QObject::connect(this, &NewTreeItemDialog::finished, this, [this](int result){

        if(result == QDialog::Accepted){
            int numberToCreate = ui->numberToCreateSpinBox->value();

            QList<PageInterface *> pluginList =
                    skrpluginhub->pluginsByType<PageInterface>();

            QVariantMap creationProperties;
            for(auto *plugin : pluginList){
                if(plugin->pageType() == m_type){
                    QVariantMap customProperties;
                    auto *customPropertiesWidget = this->getCustomPropertiesWidget();
                    if(customPropertiesWidget){

                        customProperties = customPropertiesWidget->getItemCreationProperties();
                    }

                    creationProperties = plugin->propertiesForCreationOfTreeItem(customProperties);
                    break;
                }
            }

            if(result == 1){
                if(this->actionType() == NewTreeItemDialog::AddBefore){
                if(numberToCreate == 1){
                    projectTreeCommands->addItemBefore(m_projectId, m_treeItemId, m_type);
                }
                else {
                    projectTreeCommands->addSeveralItemsBefore(m_projectId, m_treeItemId, m_type, numberToCreate, creationProperties);
                }
                }
                if(this->actionType() == NewTreeItemDialog::AddAfter){
                    if(numberToCreate == 1){
                        projectTreeCommands->addItemAfter(m_projectId, m_treeItemId, m_type);
                    }
                    else {
                        projectTreeCommands->addSeveralItemsAfter(m_projectId, m_treeItemId, m_type, numberToCreate, creationProperties);
                    }
                }
                if(this->actionType() == NewTreeItemDialog::AddSubItem){

                    if(numberToCreate == 1){
                        projectTreeCommands->addSubItem(m_projectId, m_treeItemId, m_type);
                    }
                    else {
                        projectTreeCommands->addSeveralSubItems(m_projectId, m_treeItemId, m_type, numberToCreate, creationProperties);
                    }
                }
            }
        }


        this->reset();

    });




    QList<PageInterface *> pluginList =
            skrpluginhub->pluginsByType<PageInterface>();

    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](PageInterface *plugin1, PageInterface
              *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
    );

    for(auto *plugin : pluginList){

        QListWidgetItem *item = new QListWidgetItem(QIcon(plugin->pageTypeIconUrl(-1, -1)), plugin->visualText(), ui->listWidget);
        item->setData(Qt::UserRole, plugin->pageDetailText());
        item->setData(Qt::UserRole + 1, plugin->pageType());

        m_typeWithparameterWidgetHash.insert(plugin->pageType(), plugin->pageCreationParametersWidget());
    }

    ui->listWidget->setCurrentRow(0);


}

NewTreeItemDialog::~NewTreeItemDialog()
{
    delete ui;
}

void NewTreeItemDialog::setIdentifiers(int projectId, int treeItemId)
{
    m_projectId = projectId;
    m_treeItemId = treeItemId;
}

NewTreeItemDialog::ActionType NewTreeItemDialog::actionType() const
{
    return m_actionType;
}

void NewTreeItemDialog::setActionType(ActionType newActionType)
{
    m_actionType = newActionType;
}

//----------------------------------------------------------------------

TreeItemCreationParametersWidget *NewTreeItemDialog::getCustomPropertiesWidget() const
{
 return m_customPropertiesWidget;
}

void NewTreeItemDialog::setCustomPropertiesWidget(TreeItemCreationParametersWidget *widget)
{
    if(m_customPropertiesWidget){
        ui->contentLayout->removeWidget(m_customPropertiesWidget);
        m_customPropertiesWidget->hide();
    }
    ui->contentLayout->insertWidget(2, widget);
    m_customPropertiesWidget = widget;
    m_customPropertiesWidget->show();
}

void NewTreeItemDialog::removeCustomPropetiesWidget()
{
    ui->contentLayout->removeWidget(m_customPropertiesWidget);
    m_customPropertiesWidget = nullptr;
}

void NewTreeItemDialog::reset()
{
    m_type = "";
    m_projectId = -1;
    m_treeItemId = -1;
    ui->numberToCreateSpinBox->setValue(1);

}

//----------------------------------------------------------------------


void NewTreeItemDialog::on_listWidget_currentItemChanged(QListWidgetItem *current, QListWidgetItem *previous)
{
    ui->titleLabel->setText(current->text());
    ui->detailsLabel->setText(current->data(Qt::UserRole).toString());
    m_type = current->data(Qt::UserRole + 1).toString();

    this->setCustomPropertiesWidget(m_typeWithparameterWidgetHash.value(m_type));


}


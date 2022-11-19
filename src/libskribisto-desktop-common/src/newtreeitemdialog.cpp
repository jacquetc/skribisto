#include "newtreeitemdialog.h"
#include "thememanager.h"
#include "ui_newtreeitemdialog.h"
#include "projecttreecommands.h"

#include "interfaces/pagedesktopinterface.h"

#include "interfaces/pagetypeiconinterface.h"

NewTreeItemDialog::NewTreeItemDialog(QWidget *parent) :
    QDialog(parent),
    ui(new Ui::NewTreeItemDialog), m_customPropertiesWidget(nullptr)
{
    ui->setupUi(this);

    QObject::connect(this, &NewTreeItemDialog::finished, this, [this](int result){

        if(result == QDialog::Accepted){
            int numberToCreate = ui->numberToCreateSpinBox->value();

            QVariantMap creationProperties;
            auto *customPropertiesWidget = this->getCustomPropertiesWidget();
            if(customPropertiesWidget){

                creationProperties = customPropertiesWidget->getItemCreationProperties();
            }


            //titles

            QString titleTemplate = ui->nameLineEdit->text();
            QStringList titles;
            QString title = titleTemplate;
            titles << title.replace("#", "").trimmed();

            for(int i = 2; i <= numberToCreate; i++){
                title = titleTemplate;
                titles << title.replace("#", QString::number(i)).trimmed();
            }


            if(result == 1){
                if(this->actionType() == NewTreeItemDialog::AddBefore){
                if(numberToCreate == 1){
                    projectTreeCommands->addItemBefore(m_treeItemAddress, m_type, titles.first(), creationProperties);
                }
                else {
                    projectTreeCommands->addSeveralItemsBefore(m_treeItemAddress, m_type, numberToCreate, titles, creationProperties);
                }
                }
                if(this->actionType() == NewTreeItemDialog::AddAfter){
                    if(numberToCreate == 1){
                        projectTreeCommands->addItemAfter(m_treeItemAddress, m_type, titles.first(), creationProperties);
                    }
                    else {
                        projectTreeCommands->addSeveralItemsAfter(m_treeItemAddress, m_type, numberToCreate, titles, creationProperties);
                    }
                }
                if(this->actionType() == NewTreeItemDialog::AddSubItem){

                    if(numberToCreate == 1){
                        projectTreeCommands->addSubItem(m_treeItemAddress, m_type, titles.first(), creationProperties);
                    }
                    else {
                        projectTreeCommands->addSeveralSubItems(m_treeItemAddress, m_type, numberToCreate, titles, creationProperties);
                    }
                }
            }
        }


        this->reset();

    });




    QList<PageTypeIconInterface *> pageTypeIconPluginList =
            skrpluginhub->pluginsByType<PageTypeIconInterface>();

    QHash<QString, QString> pageTypeByIconUrl;

    for(auto *plugin : pageTypeIconPluginList){
        pageTypeByIconUrl.insert(plugin->pageType(), plugin->pageTypeIconUrl(TreeItemAddress()));
    }



    QList<PageInterface *> pluginList =
            skrpluginhub->pluginsByType<PageInterface>();


    QList<PageDesktopInterface *> pageDesktopPluginList =
            skrpluginhub->pluginsByType<PageDesktopInterface>();


    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](PageInterface *plugin1, PageInterface
              *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
    );

    for(auto *plugin : pluginList){
        if(plugin->isConstructible()){

            QIcon icon = QIcon(pageTypeByIconUrl.value(plugin->pageType()));

            QListWidgetItem *item = new QListWidgetItem(icon, plugin->visualText(), ui->listWidget);
            item->setData(Qt::UserRole, plugin->pageDetailText());
            item->setData(Qt::UserRole + 1, plugin->pageType());

            for(auto *pageDesktopPlugin : pageDesktopPluginList){
                if(pageDesktopPlugin->pageType() == plugin->pageType()){
                    TreeItemCreationParametersWidget *pageCreationParametersWidget = pageDesktopPlugin->pageCreationParametersWidget();
                    if(nullptr != pageCreationParametersWidget){
                        m_typeWithparameterWidgetHash.insert(pageDesktopPlugin->pageType(), pageCreationParametersWidget);
                    }
                }
            }
        }

        else {
            m_typeWithparameterWidgetHash.insert(plugin->pageType(), new EmptyCreationParametersWidget());
        }
    }


    ui->listWidget->setCurrentRow(0);

    themeManager->scanChildrenAndAddWidgetsHoldingIcons(this);


}

NewTreeItemDialog::~NewTreeItemDialog()
{
    delete ui;
}

void NewTreeItemDialog::setIdentifiers(const TreeItemAddress &treeItemAddress)
{
    m_treeItemAddress = treeItemAddress;
    ui->listWidget->setCurrentRow(0);
}

NewTreeItemDialog::ActionType NewTreeItemDialog::actionType() const
{
    return m_actionType;
    ui->listWidget->setCurrentRow(0);
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

    if(widget){
        ui->contentLayout->insertWidget(2, widget);
        m_customPropertiesWidget = widget;
        m_customPropertiesWidget->show();
        m_customPropertiesWidget->reset();
    }
}

void NewTreeItemDialog::removeCustomPropertiesWidget()
{
    ui->contentLayout->removeWidget(m_customPropertiesWidget);
    m_customPropertiesWidget = nullptr;
}

void NewTreeItemDialog::reset()
{
    m_type = "";
    m_treeItemAddress = TreeItemAddress();
    ui->numberToCreateSpinBox->setValue(1);

}

//----------------------------------------------------------------------


void NewTreeItemDialog::on_listWidget_currentItemChanged(QListWidgetItem *current, QListWidgetItem *previous)
{
    ui->titleLabel->setText(current->text());
    ui->detailsLabel->setText(current->data(Qt::UserRole).toString());
    ui->nameLineEdit->setText(current->text() + " #");

    m_type = current->data(Qt::UserRole + 1).toString();
    this->setCustomPropertiesWidget(m_typeWithparameterWidgetHash.value(m_type));


}


#include "creationparameterswidget.h"
#include "ui_creationparameterswidget.h"

CreationParametersWidget::CreationParametersWidget(QWidget *parent) :
    TreeItemCreationParametersWidget(parent),
    ui(new Ui::CreationParametersWidget)
{
    ui->setupUi(this);

    ui->comboBox->addItem(QIcon(":/icons/skribisto/skribisto-book-beginning.svg"), tr("Beginning of a book"), "book-beginning");
    ui->comboBox->addItem(QIcon(":/icons/backup/bookmark-new.svg"), tr("Chapter"), "chapter");
    ui->comboBox->addItem(QIcon(":/icons/backup/menu_new_sep.svg"), tr("Separator"), "separator");
    ui->comboBox->addItem(QIcon(":/icons/skribisto/skribisto-book-end.svg"), tr("End of a book"), "book-end");
}

CreationParametersWidget::~CreationParametersWidget()
{
    delete ui;
}

QVariantMap CreationParametersWidget::getItemCreationProperties() const
{
    QVariantMap properties;

properties.insert("section_type", ui->comboBox->currentData().toString());

return properties;

}

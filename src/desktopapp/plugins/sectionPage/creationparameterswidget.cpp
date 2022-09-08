#include "creationparameterswidget.h"
#include "ui_creationparameterswidget.h"

CreationParametersWidget::CreationParametersWidget(QWidget *parent) :
    TreeItemCreationParametersWidget(parent),
    ui(new Ui::CreationParametersWidget)
{
    ui->setupUi(this);

    ui->comboBox->addItem(QIcon(":/icons/backup/skribisto-book-beginning.svg"), "book-beginning", tr("Beginning of a book"));
    ui->comboBox->addItem(QIcon(":/icons/backup/bookmark-new.svg"), "chapter", tr("Chapter"));
    ui->comboBox->addItem(QIcon(":/icons/backup/menu_new_sep.svg"), "separator", tr("Separator"));
    ui->comboBox->addItem(QIcon(":/icons/backup/skribisto-book-end.svg"), "book-end", tr("End of a book"));
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

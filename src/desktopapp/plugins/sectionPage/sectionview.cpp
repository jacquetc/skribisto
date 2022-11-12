#include "sectionview.h"
#include "ui_sectionview.h"
#include "toolboxes/outlinetoolbox.h"

#include "skrdata.h"

#include "projecttreecommands.h"

SectionView::SectionView(QWidget *parent) :
    View("SECTION",parent),
    centralWidgetUi(new Ui::SectionView)
{
    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);
    setCentralWidget(centralWidget);



}

SectionView::~SectionView()
{
    delete centralWidgetUi;
}

QList<Toolbox *> SectionView::toolboxes()
{

    QList<Toolbox *> toolboxes;

    OutlineToolbox *outlineToolbox = new OutlineToolbox;    toolboxes.append(outlineToolbox);
    connect(this, &View::initialized, outlineToolbox, &Toolbox::setIdentifiersAndInitialize);
    outlineToolbox->setIdentifiersAndInitialize(this->projectId(), this->treeItemId());

    return toolboxes;


}

void SectionView::initialize()
{

    centralWidgetUi->comboBox->addItem(QIcon(":/icons/skribisto/skribisto-book-beginning.svg"), tr("Beginning of a book"), "book-beginning");
    centralWidgetUi->comboBox->addItem(QIcon(":/icons/backup/bookmark-new.svg"), tr("Chapter"), "chapter");
    centralWidgetUi->comboBox->addItem(QIcon(":/icons/backup/menu_new_sep.svg"), tr("Separator"), "separator");
    centralWidgetUi->comboBox->addItem(QIcon(":/icons/skribisto/skribisto-book-end.svg"), tr("End of a book"), "book-end");

    QString property = skrdata->treePropertyHub()->getProperty(this->projectId(), this->treeItemId(), "section_type", "separator");
    centralWidgetUi->comboBox->setCurrentIndex(centralWidgetUi->comboBox->findData(property));


    QObject::connect(centralWidgetUi->comboBox, &QComboBox::activated, this, [this](){

        QVariantMap properties;
        properties.insert("section_type", centralWidgetUi->comboBox->currentData().toString());

        projectTreeCommands->setItemProperties(this->projectId(), this->treeItemId(), properties);
    });


    QObject::connect(skrdata->treePropertyHub(), &SKRPropertyHub::propertyChanged, this, [this](int            projectId,
                     int            propertyId,
                     int            treeItemCode,
                     const QString& name,
                     const QString& value){

        if(projectId == this->projectId() && treeItemCode == this->treeItemId() && name == "section_type"){
            centralWidgetUi->comboBox->setCurrentIndex(centralWidgetUi->comboBox->findData(value));
        }

    });

    // history
    emit this->addToHistoryCalled(this, QVariantMap());
}

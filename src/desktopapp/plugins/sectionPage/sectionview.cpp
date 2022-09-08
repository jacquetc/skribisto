#include "sectionview.h"
#include "ui_sectionview.h"

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


    return toolboxes;

}

void SectionView::initialize()
{

    centralWidgetUi->comboBox->addItem(QIcon(":/icons/backup/skribisto-book-beginning.svg"), "book-beginning", tr("Beginning of a book"));
    centralWidgetUi->comboBox->addItem(QIcon(":/icons/backup/bookmark-new.svg"), "chapter", tr("Chapter"));
    centralWidgetUi->comboBox->addItem(QIcon(":/icons/backup/menu_new_sep.svg"), "separator", tr("Separator"));
    centralWidgetUi->comboBox->addItem(QIcon(":/icons/backup/skribisto-book-end.svg"), "book-end", tr("End of a book"));

}

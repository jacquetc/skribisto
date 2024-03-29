#include "folderview.h"
#include "toolboxes/outlinetoolbox.h"
#include "toolboxes/tagtoolbox.h"
#include "ui_folderview.h"

FolderView::FolderView(QWidget *parent) :
    View("FOLDER", parent),
    centralWidgetUi(new Ui::FolderView)
{
    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);

    setCentralWidget(centralWidget);

}

FolderView::~FolderView()
{
    delete centralWidgetUi;
}

QList<Toolbox *> FolderView::toolboxes()
{

    QList<Toolbox *> toolboxes;

    OutlineToolbox *outlineToolbox = new OutlineToolbox;    toolboxes.append(outlineToolbox);
    connect(this, &View::initialized, outlineToolbox, &Toolbox::setIdentifiersAndInitialize);
    outlineToolbox->setIdentifiersAndInitialize(this->treeItemAddress());

    TagToolbox *tagToolbox = new TagToolbox(nullptr, this->treeItemAddress());
    toolboxes.append(tagToolbox);

    return toolboxes;


}

void FolderView::initialize()
{

    // history
    emit this->addToHistoryCalled(this, QVariantMap());
}

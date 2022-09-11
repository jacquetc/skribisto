#include "folderview.h"
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


    return toolboxes;

}

void FolderView::initialize()
{

}

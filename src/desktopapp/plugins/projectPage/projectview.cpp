#include "projectview.h"
#include "ui_projectview.h"

ProjectView::ProjectView(QWidget *parent) :
    View("PROJECT", parent),
    centralWidgetUi(new Ui::ProjectView)
{
    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);

    setCentralWidget(centralWidget);



    this->setFocusProxy(centralWidgetUi->lineEdit);
}

ProjectView::~ProjectView()
{
    delete centralWidgetUi;
}

QList<Toolbox *> ProjectView::toolboxes()
{
    QList<Toolbox *> toolboxes;


    return toolboxes;

}

void ProjectView::initialize()
{

}

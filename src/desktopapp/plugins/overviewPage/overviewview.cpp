#include "overviewview.h"
#include "ui_overviewview.h"

OverviewView::OverviewView(QWidget *parent) :
    View("OVERVIEW", parent),
    centralWidgetUi(new Ui::OverviewView)
{

    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);

    setCentralWidget(centralWidget);
    this->setFocusProxy(centralWidgetUi->treeView);

}

OverviewView::~OverviewView()
{
    delete centralWidgetUi;
}

QList<Toolbox *> OverviewView::toolboxes()
{

    QList<Toolbox *> toolboxes;


    return toolboxes;
}

void OverviewView::initialize()
{

}

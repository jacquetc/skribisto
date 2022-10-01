#include "emptyview.h"
#include "thememanager.h"
#include "ui_emptyview.h"

EmptyView::EmptyView(QWidget *parent) :
    View("EMPTY", parent),
    centralWidgetUi(new Ui::EmptyView)
{
    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);

    setCentralWidget(centralWidget);

    connect(themeManager, &ThemeManager::currentThemeTypeChanged, this, [&](){

    }, Qt::QueuedConnection);
}

EmptyView::~EmptyView()
{
    delete centralWidgetUi;
}

QList<Toolbox *> EmptyView::toolboxes()
{
    QList<Toolbox *> toolboxes;
    return toolboxes;
}

void EmptyView::initialize()
{

}

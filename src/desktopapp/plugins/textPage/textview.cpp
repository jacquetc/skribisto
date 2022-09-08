#include "textview.h"
#include "ui_textview.h"

#include <toolboxes/outlinetoolbox.h>

TextView::TextView(QWidget *parent) :
    View("TEXT",parent),
    centralWidgetUi(new Ui::TextView)
{
    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);
    setCentralWidget(centralWidget);
}

TextView::~TextView()
{
    delete centralWidgetUi;
}

QList<Toolbox *> TextView::toolboxes()
{
    QList<Toolbox *> toolboxes;


    auto *outline = new OutlineToolbox;
toolboxes.append(outline);
    return toolboxes;

}

void TextView::initialize()
{

}

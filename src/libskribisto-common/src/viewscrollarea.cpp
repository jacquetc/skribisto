#include "viewbasecoloredwidget.h"
#include "viewscrollarea.h"

ViewScrollArea::ViewScrollArea(QWidget *parent) : QScrollArea(parent)
{

    this->setViewport(new ViewBaseColoredWidget);

}

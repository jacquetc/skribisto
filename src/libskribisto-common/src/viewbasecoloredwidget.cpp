#include "viewbasecoloredwidget.h"

#include <QPainter>

ViewBaseColoredWidget::ViewBaseColoredWidget(QWidget *parent)
    : QWidget{parent}
{

}

void ViewBaseColoredWidget::paintEvent(QPaintEvent *event)
{

    QPainter painter(this);

    painter.fillRect(0, 0, this->width(), this->height(),palette().color(QPalette::Base));
}

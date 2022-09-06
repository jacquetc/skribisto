#include "view.h"
#include "ui_view.h"

View::View(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::View)
{
    ui->setupUi(this);
}

View::~View()
{
    delete ui;
}

void View::setCentralWidget(QWidget *widget)
{
    ui->centralLayout->insertWidget(1, widget);
}

void View::setToolBar(QToolBar *toolBar)
{
    ui->toolBarHolder->addWidget(toolBar);
}

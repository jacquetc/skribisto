#include "toptoolbar.h"
#include "ui_toptoolbar.h"

TopToolBar::TopToolBar(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::TopToolBar)
{
    ui->setupUi(this);
}

TopToolBar::~TopToolBar()
{
    delete ui;
}

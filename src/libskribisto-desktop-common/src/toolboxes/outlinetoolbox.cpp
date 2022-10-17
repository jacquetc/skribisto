#include "outlinetoolbox.h"
#include "ui_outlinetoolbox.h"


OutlineToolbox::OutlineToolbox(QWidget *parent) :
    Toolbox(parent),
    ui(new Ui::OutlineToolbox)
{
    ui->setupUi(this);
}

OutlineToolbox::~OutlineToolbox()
{
    delete ui;
}

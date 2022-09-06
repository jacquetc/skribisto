#include "navigation.h"
#include "ui_navigation.h"
#include "projecttreemodel.h"

Navigation::Navigation(class QWidget *parent) :
    ui(new Ui::Navigation)
{
    ui->setupUi(this);

    ui->treeView->setModel(ProjectTreeModel::instance());
}

Navigation::~Navigation()
{
    delete ui;
}

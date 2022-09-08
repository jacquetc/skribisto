#include "toptoolbar.h"
#include "ui_toptoolbar.h"

#include "projecttreecommands.h"

TopToolBar::TopToolBar(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::TopToolBar)
{
    ui->setupUi(this);


    // undo stack
    ui->undoButton->setDefaultAction(projectTreeCommands->undoStack()->createUndoAction(this));
    ui->redoButton->setDefaultAction(projectTreeCommands->undoStack()->createRedoAction(this));

}

TopToolBar::~TopToolBar()
{
    delete ui;
}

void TopToolBar::on_toolButton_triggered(QAction *arg1)
{
}


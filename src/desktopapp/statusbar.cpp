#include "statusbar.h"
#include "ui_statusbar.h"
#include "invoker.h"
#include <QTimer>

StatusBar::StatusBar(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::StatusBar)
{
    ui->setupUi(this);    

    QTimer::singleShot(0, this, &StatusBar::init);

}

StatusBar::~StatusBar()
{
    delete ui;
}

void StatusBar::init()
{
    auto actionShow_View_Dock = invoke<QAction>(this, "actionShow_View_Dock");
    ui->showViewDockButton->setDefaultAction(actionShow_View_Dock);

    auto actionShow_Project_Dock = invoke<QAction>(this, "actionShow_Project_Dock");
    ui->showProjectDockButton->setDefaultAction(actionShow_Project_Dock);


}

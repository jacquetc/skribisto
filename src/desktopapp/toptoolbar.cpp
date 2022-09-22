#include "toptoolbar.h"
#include "invoker.h"
#include "ui_toptoolbar.h"

#include "projecttreecommands.h"

#include <QToolBar>

TopToolBar::TopToolBar(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::TopToolBar)
{
    ui->setupUi(this);

    m_middleToolBar = new QToolBar;
    ui->horizontalLayout->addWidget(m_middleToolBar);
    ui->horizontalLayout->setStretchFactor(m_middleToolBar, 1);
    ui->horizontalLayout->setAlignment(m_middleToolBar, Qt::AlignHCenter);

    m_rightToolBar = new QToolBar;
    ui->horizontalLayout->addWidget(m_rightToolBar);


    QTimer::singleShot(0, this, &TopToolBar::init);

}

    //---------------------------------------

void TopToolBar::init(){

    auto actionSwitch_theme = invoke<QAction>(this, "actionSwitch_theme");
    m_rightToolBar->addAction(actionSwitch_theme);
}

TopToolBar::~TopToolBar()
{
    delete ui;
}



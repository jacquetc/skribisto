#include "dock.h"

#include <QHBoxLayout>
#include <QMainWindow>
#include <QPushButton>
#include <QTimer>

Dock::Dock(QWidget *parent)
    : QDockWidget{parent}
{
    m_dockTitle = new DockTitle(this);
    this->setTitleBarWidget(m_dockTitle);



    QTimer::singleShot(0, this, &Dock::init);
    }

    //---------------------------------------

    void Dock::init(){

        m_stack = new QStackedWidget(this);
        this->setWidget(m_stack);
        m_stack->show();
    }

void Dock::setToolboxes(QList<Toolbox *> toolboxes)
{

    for(auto toolbox : toolboxes){

        m_dockTitle->tabBar()->addTab(toolbox->title());

    }
}

QStackedWidget *Dock::stack() const
{
    return m_stack;
}

//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------


DockTitle::DockTitle(QWidget *parent): QWidget(parent)

{
    QDockWidget *dockWidget = qobject_cast<QDockWidget*>(parentWidget());

    m_layout = new QHBoxLayout(this);


    m_layout->setContentsMargins(0,0,0,0);
    m_hideButton = new QToolButton;
    m_hideButton->setText(tr("Hide dock"));
    //hideButton->setIcon();
    QObject::connect(m_hideButton, &QToolButton::clicked, dockWidget, &QDockWidget::hide);

    m_tabBar = new QTabBar();
    m_tabBar->setChangeCurrentOnDrag(true);
    m_tabBar->setExpanding(false);
    m_layout->addWidget(m_tabBar);
    m_layout->setStretchFactor(m_tabBar, 1);

    m_layout->addWidget(m_hideButton);

     QTimer::singleShot(0, this, &DockTitle::init);
}

//------------------------------------------------------------------------------------

void DockTitle::init()
{

    QDockWidget *dockWidget = qobject_cast<QDockWidget*>(parentWidget());
    QMainWindow *mainWindow = qobject_cast<QMainWindow*>(this->window());
    if(mainWindow->dockWidgetArea(dockWidget) == Qt::RightDockWidgetArea){
        m_layout->setDirection(QBoxLayout::RightToLeft);
    }


    m_tabBar->addTab("afzef");
   m_tabBar->addTab("afzef");
   m_tabBar->addTab("afzef");
  m_tabBar->addTab("afzef");
}

//------------------------------------------------------------------------------------

QSize DockTitle::sizeHint() const
{
 return QSize(1000, 40);
}

//------------------------------------------------------------------------------------

QSize DockTitle::minimumSizeHint() const
{
    return QSize(150, 40);

}


QTabBar *DockTitle::tabBar() const
{
    return m_tabBar;
}

//------------------------------------------------------------------------------------

#include "dock.h"
#include "thememanager.h"

#include <QHBoxLayout>
#include <QMainWindow>
#include <QPaintEvent>
#include <QPainter>
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
    // clear

    m_dockTitle->toolbarSelector()->clear();

    m_stack->deleteLater();
    m_stack = new QStackedWidget(this);
    this->setWidget(m_stack);

    QObject::connect(m_dockTitle->toolbarSelector(), &ToolbarSelector::currentIndexChanged, m_stack, &QStackedWidget::setCurrentIndex);


    // add

    int index = 0;
    for(auto toolbox : toolboxes){
        m_stack->addWidget(toolbox);
        m_dockTitle->toolbarSelector()->add(toolbox->icon(), toolbox->title());
        index++;

    }

    this->setProperty("themeZone", "sideZone");
    themeManager->reapplyCurrentTheme();
    themeManager->scanChildrenAndAddWidgetsHoldingIcons(this);

}

void Dock::paintEvent(QPaintEvent *event)
{
//    QPainter painter(this);
//    painter.setRenderHint(QPainter::Antialiasing);

//    painter.save();
//    painter.fillRect(event->rect(), this->palette().window());

//    painter.restore();

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
    this->setContentsMargins(0,0,0,0);


    QDockWidget *dockWidget = qobject_cast<QDockWidget*>(parentWidget());


    QVBoxLayout *vLayout = new QVBoxLayout(this);
    vLayout->setContentsMargins(0,0,0,0);
    vLayout->setSpacing(0);


    m_layout = new QHBoxLayout;
    m_layout->setContentsMargins(0,0,0,0);
    m_layout->addStretch(2);

    m_toolbarSelector = new ToolbarSelector;

    m_layout->addWidget(m_toolbarSelector);
    m_layout->setStretchFactor(m_toolbarSelector, 1);

    m_layout->addStretch(2);

    m_hideButton = new QToolButton;
    m_hideButton->setText(tr("Hide dock"));
    m_hideButton->setAutoRaise(true);
    m_hideButton->setIcon(QIcon(":/icons/skribisto/dock-left-close.svg"));

    QObject::connect(m_hideButton, &QToolButton::clicked, dockWidget, &QDockWidget::hide);


    m_layout->addWidget(m_hideButton);

    vLayout->addLayout(m_layout);

    auto *line = new QFrame();
    line->setFrameShape(QFrame::HLine);
    line->setFrameShadow(QFrame::Plain);
    line->setEnabled(false);
    line->setMaximumHeight(2);
    line->setLineWidth(0);

    vLayout->addWidget(line);

     QTimer::singleShot(0, this, &DockTitle::init);
}

//------------------------------------------------------------------------------------

void DockTitle::init()
{

    QDockWidget *dockWidget = qobject_cast<QDockWidget*>(parentWidget());
    QMainWindow *mainWindow = qobject_cast<QMainWindow*>(this->window());
    if(mainWindow->dockWidgetArea(dockWidget) == Qt::RightDockWidgetArea){
        m_layout->setDirection(QBoxLayout::RightToLeft);
        m_hideButton->setIcon(QIcon(":/icons/skribisto/dock-right-close.svg"));
    }

}

//------------------------------------------------------------------------------------

ToolbarSelector *DockTitle::toolbarSelector() const
{
    return m_toolbarSelector;
}


//------------------------------------------------------------------------------------

QSize DockTitle::sizeHint() const
{
 return QSize(1000, 26);
}

//------------------------------------------------------------------------------------

QSize DockTitle::minimumSizeHint() const
{
    return QSize(150, 26);

}


//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------

ToolbarSelector::ToolbarSelector(QWidget *parent)
{
    this->setContentsMargins(0,0,0,0);
    this->setSizePolicy(QSizePolicy::Maximum, QSizePolicy::Fixed);

 m_layout = new QHBoxLayout(this);
 m_layout->setContentsMargins(0,0,0,0);
 m_layout->setSpacing(3);

 m_actionGroup = new QActionGroup(this);

}

//------------------------------------------------------------------------------------

void ToolbarSelector::add(const QIcon &icon, const QString &title)
{
    auto *action = new QAction(icon, title, m_actionGroup);
    action->setCheckable(true);

    this->connect(action, &QAction::toggled,this, [this](bool toggled){

        if(toggled){
            auto *senderAction = static_cast<QAction *>(this->sender());

            int index = 0;
            for(auto *action : m_actionGroup->actions()){
                if(action == senderAction){
                    emit currentIndexChanged(index);
                }
                index++;
            }
        }
    });


    QToolButton *button = new QToolButton();
    button->setAutoRaise(true);
    button->setDefaultAction(action);
    button->setMinimumSize(20, 20);

    m_layout->addWidget(button);

    if(m_layout->indexOf(button) == 0){
        button->setChecked(true);
    }
}

//------------------------------------------------------------------------------------

void ToolbarSelector::clear()
{
    while(m_layout->count() > 0){

    QWidget *widget = static_cast<QWidget *>(m_layout->takeAt(0)->widget());
        widget->hide();
        widget->deleteLater();
    }


    for(auto *action : m_actionGroup->actions()){
        m_actionGroup->removeAction(action);
        action->deleteLater();
    }

}

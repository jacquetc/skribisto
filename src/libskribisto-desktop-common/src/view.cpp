#include "view.h"
#include "QtCore/qobjectdefs.h"
#include "desktopapplication.h"
#include "ui_view.h"

#include "invoker.h"
#include "viewmanager.h"

#include <QPainter>
#include <QTimer>


#include <QMenu>
#include "thememanager.h"
#include <QMetaObject>

View::View(const QString &type, QWidget *parent) :
    QWidget(parent),
    ui(new Ui::View), m_type(type), m_projectId(-1), m_treeItemId(-1), m_uuid(QUuid::createUuid())
{
    ui->setupUi(this);

    QToolBar *historyToolbar = new QToolBar;
    historyToolbar->setIconSize(QSize(16, 16));
    historyToolbar->setContentsMargins(0, 0, 0, 0);

    historyToolbar->addWidget(ui->previousHistory);
    historyToolbar->addWidget(ui->nextHistory);
    ui->toolBarLayout->insertWidget(0, historyToolbar);


    //TODO: implement it
    historyToolbar->hide();

    QToolBar *viewControlsToolbar = new QToolBar;
    viewControlsToolbar->setIconSize(QSize(16, 16));
    viewControlsToolbar->setContentsMargins(0, 0, 0, 0);

    viewControlsToolbar->addWidget(ui->splitToolButton);
    viewControlsToolbar->addWidget(ui->closeToolButton);
    ui->toolBarLayout->addWidget(viewControlsToolbar);

    ui->viewControlsFrame->setParent(nullptr);
    ui->viewControlsFrame->deleteLater();
    ui->navigationFrame->setParent(nullptr);
    ui->navigationFrame->deleteLater();
    ui->toolBarLayout->setStretch(1, 1);
    ui->toolBarLayout->setContentsMargins(0,0,0,0);

    //------------------------------

    QAction *splitHorizontalyAction = new QAction(QIcon(":/icons/backup/view-split-left-right.svg"), "Split", this);
    QObject::connect(splitHorizontalyAction, &QAction::triggered, this, [this](){

        ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
        viewManager->addViewParametersBeforeCreation(this->addOtherViewParametersBeforeSplit());
        viewManager->splitForSamePage(this, Qt::Horizontal);

    });


    //------------------------------

    QAction *splitVerticalyAction = new QAction(QIcon(":/icons/backup/view-split-top-bottom.svg"), "Split verticaly", this);
    QObject::connect(splitVerticalyAction, &QAction::triggered, this, [this](){

        ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
        viewManager->addViewParametersBeforeCreation(this->addOtherViewParametersBeforeSplit());
        viewManager->splitForSamePage(this, Qt::Vertical);

    });

    //------------------------------

    QAction *openInNewWindowAction = new QAction(QIcon(":/icons/backup/window-new.svg"), "Open in a new window", this);
    QObject::connect(openInNewWindowAction, &QAction::triggered, this, [this](){

        if(m_treeItemId == -1 && m_projectId == -1){
            QMetaObject::invokeMethod(this->window(), "addWindowForProjectIndependantPageType", Qt::QueuedConnection, Q_ARG(QString, m_type));
        }
        else if(m_treeItemId == -1){
            QMetaObject::invokeMethod(this->window(), "addWindowForProjectDependantPageType", Qt::QueuedConnection
                                      , Q_ARG(int, m_projectId)
                                      , Q_ARG(QString, m_type)
                                      );
        }
        else {
            QMetaObject::invokeMethod(this->window(), "addWindowForItemId", Qt::QueuedConnection
                                      , Q_ARG(int, m_projectId)
                                      , Q_ARG(int, m_treeItemId)
                                      );
        }




    });

    //------------------------------

    QMenu *splitMenu = new QMenu;
    splitMenu->addAction(splitHorizontalyAction);
    splitMenu->addAction(splitVerticalyAction);
    splitMenu->addAction(openInNewWindowAction);

    ui->splitToolButton->setMenu(splitMenu);

    // settings:

    connect(static_cast<DesktopApplication *>(qApp), &DesktopApplication::settingsChanged, this, &View::settingsChanged);


    QTimer::singleShot(0, this, &View::init);
}

//---------------------------------------

void View::init(){

    this->setProperty("themeZone", "middleZone");
    themeManager->reapplyCurrentTheme();

}

QUuid View::uuid() const
{
    return m_uuid;
}

void View::setUuid(const QUuid &newUuid)
{
    if (m_uuid == newUuid)
        return;
    m_uuid = newUuid;
    emit uuidChanged();
}

//---------------------------------------

//---------------------------------------

View::~View()
{
    delete ui;
}

//---------------------------------------

void View::setIdentifiersAndInitialize(int projectId, int treeItemId)
{
    m_projectId = projectId;
    m_treeItemId = treeItemId;

    this->initialize();

    this->applyParameters();

    emit initialized(projectId, treeItemId);
}

void View::setCentralWidget(QWidget *widget)
{
    ui->centralLayout->addWidget(widget);

    themeManager->scanChildrenAndAddWidgetsHoldingIcons(this);
}

void View::setToolBar(QToolBar *toolBar)
{
    toolBar->setIconSize(QSize(16,16));
    toolBar->setContentsMargins(0,0,0,0);
    ui->toolBarLayout->insertWidget(1, toolBar);
    ui->toolBarLayout->setStretchFactor(toolBar, 1);
    ui->toolBarPlaceHolder->deleteLater();

    themeManager->scanChildrenAndAddWidgetsHoldingIcons(this);
}

void View::setSecondToolBar(QToolBar *toolBar)
{
    toolBar->hide();
    toolBar->setIconSize(QSize(16,16));
    toolBar->setContentsMargins(0,0,0,0);
    if(ui->toolBarSystemLayout->count() == 2){
        ui->toolBarSystemLayout->removeItem(ui->toolBarSystemLayout->itemAt(1));
    }

    ui->toolBarSystemLayout->insertWidget(1, toolBar);

    themeManager->scanChildrenAndAddWidgetsHoldingIcons(this);


}

void View::setSecondToolBarVisible(bool visible)
{
    ui->toolBarSystemLayout->itemAt(1)->widget()->setVisible(visible);
}

//------------------------------------------------------------

int View::treeItemId() const
{
    return m_treeItemId;
}

//------------------------------------------------------------

int View::projectId() const
{
    return m_projectId;
}

//------------------------------------------------------------

const QString &View::type() const
{
    return m_type;
}

//------------------------------------------------------------

void View::on_closeToolButton_clicked()
{

    ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
    viewManager->removeSplitWithView(this);
}

//------------------------------------------------------------

const QVariantMap &View::parameters() const
{
    return m_parameters;
}

//------------------------------------------------------------

void View::setParameters(const QVariantMap &newParameters)
{
    m_parameters = newParameters;
}

//------------------------------------------------------------


//void View::paintEvent(QPaintEvent *event)
// {
//     QPainter painter(this);

//     painter.fillRect(0, 0, this->width(), this->height(),palette().color(QPalette::Base));

// }

#include "view.h"
#include "ui_view.h"

#include "invoker.h"
#include "viewmanager.h"

#include <QMenu>

View::View(const QString &type, QWidget *parent) :
    QWidget(parent),
    ui(new Ui::View), m_type(type), m_projectId(-1), m_treeItemId(-1)
{
    ui->setupUi(this);

    QAction *splitHorizontalyAction = new QAction(QIcon(":/icons/backup/view-split-left-right.svg"), "Split", this);
    QObject::connect(splitHorizontalyAction, &QAction::triggered, this, [this](){

        ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
        viewManager->addViewParametersBeforeCreation(this->addOtherViewParametersBeforeSplit());
        viewManager->splitForSamePage(this, Qt::Horizontal);

    });


    QAction *splitVerticalyAction = new QAction(QIcon(":/icons/backup/view-split-top-bottom.svg"), "Split verticaly", this);
    QObject::connect(splitVerticalyAction, &QAction::triggered, this, [this](){

        ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
        viewManager->addViewParametersBeforeCreation(this->addOtherViewParametersBeforeSplit());
        viewManager->splitForSamePage(this, Qt::Vertical);

    });

    QAction *openInNewWindowAction = new QAction(QIcon(":/icons/backup/window-new.svg"), "Open in a new window", this);

    QMenu *splitMenu = new QMenu;
    splitMenu->addAction(splitHorizontalyAction);
    splitMenu->addAction(splitVerticalyAction);
    splitMenu->addAction(openInNewWindowAction);

    ui->splitToolButton->setMenu(splitMenu);

}

View::~View()
{
    delete ui;
}

void View::setIdentifiersAndInitialize(int projectId, int treeItemId)
{
    m_projectId = projectId;
    m_treeItemId = treeItemId;

    this->initialize();
}

void View::setCentralWidget(QWidget *widget)
{
    ui->centralLayout->addWidget(widget);
}

void View::setToolBar(QToolBar *toolBar)
{
    ui->toolBarHolder->addWidget(toolBar);
}

int View::treeItemId() const
{
    return m_treeItemId;
}

int View::projectId() const
{
    return m_projectId;
}

const QString &View::type() const
{
    return m_type;
}

void View::on_closeToolButton_clicked()
{

    ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
    viewManager->removeSplit(this);
}

const QVariantMap &View::parameters() const
{
    return m_parameters;
}

void View::setParameters(const QVariantMap &newParameters)
{
    m_parameters = newParameters;
}


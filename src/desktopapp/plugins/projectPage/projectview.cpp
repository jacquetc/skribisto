#include "projectview.h"
#include "projectcommands.h"
#include "ui_projectview.h"
#include "skrdata.h"

ProjectView::ProjectView(QWidget *parent) :
    View("PROJECT", parent),
    centralWidgetUi(new Ui::ProjectView)
{
    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);

    setCentralWidget(centralWidget);
    this->setFocusProxy(centralWidgetUi->projectNameLineEdit);


    connect(centralWidgetUi->projectNameLineEdit, &QLineEdit::editingFinished, this, [this](){
        projectCommands->setProjectName(this->projectId(), centralWidgetUi->projectNameLineEdit->text());
    });
    connect(centralWidgetUi->authorLineEdit, &QLineEdit::editingFinished, this, [this](){
        projectCommands->setAuthor(this->projectId(), centralWidgetUi->authorLineEdit->text());
    });
}

ProjectView::~ProjectView()
{
    delete centralWidgetUi;
}

QList<Toolbox *> ProjectView::toolboxes()
{
    QList<Toolbox *> toolboxes;


    return toolboxes;

}

void ProjectView::initialize()
{

    centralWidgetUi->projectNameLineEdit->setText(skrdata->projectHub()->getProjectName(this->projectId()));
    centralWidgetUi->authorLineEdit->setText(skrdata->projectHub()->getAuthor(this->projectId()));

}



#include "projectview.h"
#include "projectcommands.h"
#include "ui_projectview.h"
#include "skrdata.h"
#include "tagmanagertoolbox.h"

ProjectView::ProjectView(QWidget *parent) :
    View("PROJECT", parent),
    centralWidgetUi(new Ui::ProjectView)
{
    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);

    setCentralWidget(centralWidget);
    this->setFocusProxy(centralWidgetUi->projectNameLineEdit);


    connect(centralWidgetUi->projectNameLineEdit, &QLineEdit::editingFinished, this, [this](){
        if(skrdata->projectHub()->getProjectName(this->projectId()) != centralWidgetUi->projectNameLineEdit->text()){
            projectCommands->setProjectName(this->projectId(), centralWidgetUi->projectNameLineEdit->text());
        }
    });
    connect(centralWidgetUi->authorLineEdit, &QLineEdit::editingFinished, this, [this](){
        if(skrdata->projectHub()->getAuthor(this->projectId()) != centralWidgetUi->authorLineEdit->text()){
            projectCommands->setAuthor(this->projectId(), centralWidgetUi->authorLineEdit->text());
        }
    });
}

ProjectView::~ProjectView()
{
    delete centralWidgetUi;
}

QList<Toolbox *> ProjectView::toolboxes()
{
    QList<Toolbox *> toolboxes;

    toolboxes.append(new TagManagerToolbox(nullptr, this->projectId()));

    return toolboxes;

}

void ProjectView::initialize()
{

    centralWidgetUi->projectNameLineEdit->setText(skrdata->projectHub()->getProjectName(this->projectId()));
    centralWidgetUi->authorLineEdit->setText(skrdata->projectHub()->getAuthor(this->projectId()));

}



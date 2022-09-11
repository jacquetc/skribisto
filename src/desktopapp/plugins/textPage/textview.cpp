#include "textview.h"
#include "ui_textview.h"

#include <skrdata.h>

TextView::TextView(QWidget *parent) :
    View("TEXT",parent),
    centralWidgetUi(new Ui::TextView), m_isSecondaryContent(false)
{

    //central ui
    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);
    setCentralWidget(centralWidget);

    //toolbar

    QToolBar *toolBar = new QToolBar;
    //toolBar->addWidget(toolBar);

    setToolBar(toolBar);

    // link scrollbar to textedit
    centralWidgetUi->textEdit->setVerticalScrollBar(centralWidgetUi->verticalScrollBar);
    //centralWidgetUi->horizontalLayout->addWidget(centralWidgetUi->verticalScrollBar);
    //centralWidgetUi->horizontalLayout->set  (centralWidgetUi->verticalScrollBar);
    //connect(centralWidgetUi->textEdit.)
    centralWidgetUi->horizontalLayout->setStretchFactor(centralWidgetUi->horizontalLayout_2, 1);

    this->setFocusProxy(centralWidgetUi->textEdit);

}

TextView::~TextView()
{
    saveContent();
    delete centralWidgetUi;
}

QList<Toolbox *> TextView::toolboxes()
{
    QList<Toolbox *> toolboxes;


    m_outlineToolbox = new OutlineToolbox;
    toolboxes.append(m_outlineToolbox);
    return toolboxes;

}

void TextView::initialize()
{
    QTextDocument *document = new QTextDocument(this);

    m_isSecondaryContent = parameters().value("is_secondary_content", false).toBool();
    if(m_isSecondaryContent){
        document->setHtml(skrdata->treeHub()->getSecondaryContent(this->projectId(), this->treeItemId()));

    }
    else {
        document->setHtml(skrdata->treeHub()->getPrimaryContent(this->projectId(), this->treeItemId()));

    }

    centralWidgetUi->textEdit->setDocument(document);

    QTimer *saveTimer = new QTimer(this);
    saveTimer->setSingleShot(true);
    saveTimer->setInterval(2000);


    connect(centralWidgetUi->textEdit, &QTextEdit::textChanged, this, [saveTimer](){
        if(saveTimer->isActive()){
            saveTimer->stop();
        }
        saveTimer->start();
    });
    connect(saveTimer, &QTimer::timeout, this, &TextView::saveContent);

}

void TextView::saveContent()
{
    if(m_isSecondaryContent){
        skrdata->treeHub()->setSecondaryContent(this->projectId(), this->treeItemId(), centralWidgetUi->textEdit->toHtml());
    }
    else {
        skrdata->treeHub()->setPrimaryContent(this->projectId(), this->treeItemId(), centralWidgetUi->textEdit->toHtml());
    }
}

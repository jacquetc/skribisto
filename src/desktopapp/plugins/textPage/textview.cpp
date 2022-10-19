#include "textview.h"
#include "text/textbridge.h"
#include "ui_textview.h"

#include <QWheelEvent>
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
    centralWidgetUi->horizontalLayout->addWidget(centralWidgetUi->verticalScrollBar);
    centralWidgetUi->verticalScrollBar->hide();
    connect(centralWidgetUi->verticalScrollBar, &QScrollBar::rangeChanged, this, [&](int min, int max){
        centralWidgetUi->verticalScrollBar->setVisible(min < max);
    });
    //centralWidgetUi->horizontalLayout->set  (centralWidgetUi->verticalScrollBar);
    //connect(centralWidgetUi->textEdit.)


    centralWidgetUi->horizontalLayout->setStretchFactor(centralWidgetUi->horizontalLayout_2, 1);

    this->setFocusProxy(centralWidgetUi->textEdit);
    centralWidget->setFocusProxy(centralWidgetUi->textEdit);
    centralWidgetUi->widget->setFocusProxy(centralWidgetUi->textEdit);


    connect(centralWidgetUi->sizeHandle, &SizeHandle::moved, this, [this](int deltaX){
        centralWidgetUi->textEditHolder->setMaximumWidth(centralWidgetUi->textEditHolder->maximumWidth() + deltaX);

        QSettings settings;
        settings.setValue("textPage/textWidth", centralWidgetUi->textEditHolder->maximumWidth());
    });

    QSettings settings;
    centralWidgetUi->textEditHolder->setMaximumWidth(settings.value("textPage/textWidth", 450).toInt());
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
        document->setMarkdown(skrdata->treeHub()->getSecondaryContent(this->projectId(), this->treeItemId()));

    }
    else {
        document->setMarkdown(skrdata->treeHub()->getPrimaryContent(this->projectId(), this->treeItemId()));

    }

    centralWidgetUi->textEdit->setDocument(document);

    QString uniqueDocumentReference = QString("%1_%2_%3").arg(this->projectId()).arg(this->treeItemId()).arg(m_isSecondaryContent ? "secondary" : "primary");
    textBridge->subscribeTextDocument(
                uniqueDocumentReference,
                centralWidgetUi->textEdit->uuid(),
                document);


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
        skrdata->treeHub()->setSecondaryContent(this->projectId(), this->treeItemId(), centralWidgetUi->textEdit->toMarkdown());
    }
    else {
        skrdata->treeHub()->setPrimaryContent(this->projectId(), this->treeItemId(), centralWidgetUi->textEdit->toMarkdown());
    }
}


void TextView::mousePressEvent(QMouseEvent *event)
{

    centralWidgetUi->textEdit->setFocus();
    centralWidgetUi->textEdit->ensureCursorVisible();
}

void TextView::wheelEvent(QWheelEvent *event)
{
    if(event->modifiers() == Qt::ShiftModifier) {

        QPoint numPixels = event->pixelDelta();
        QPoint numDegrees = event->angleDelta() / 8;


        if (!numPixels.isNull()) {
            if(numPixels.x() > 0) {
                centralWidgetUi->textEdit->zoomOut();
            }
            else{
                centralWidgetUi->textEdit->zoomIn();
            }
        } else if (!numDegrees.isNull()) {
            QPoint numSteps = numDegrees / 15;
            if(numSteps.x() > 0) {
                centralWidgetUi->textEdit->zoomOut();
            }
            else{
                centralWidgetUi->textEdit->zoomIn();
            }
        }

    }
    else {
            centralWidgetUi->textEdit->wheelEvent(event);
    }
}

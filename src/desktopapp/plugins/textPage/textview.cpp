#include "textview.h"
#include "skrusersettings.h"
#include "text/textbridge.h"
#include "ui_textview.h"
#include "toolboxes/outlinetoolbox.h"

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
    centralWidgetUi->textEdit->setWordWrapMode(QTextOption::WrapAtWordBoundaryOrAnywhere);

    centralWidgetUi->horizontalLayout->setStretchFactor(centralWidgetUi->horizontalLayout_2, 1);

    this->setFocusProxy(centralWidgetUi->textEdit);
    centralWidget->setFocusProxy(centralWidgetUi->textEdit);
    centralWidgetUi->widget->setFocusProxy(centralWidgetUi->textEdit);


    connect(centralWidgetUi->sizeHandle, &SizeHandle::moved, this, [this](int deltaX){
        centralWidgetUi->textEditHolder->setMaximumWidth(centralWidgetUi->textEditHolder->maximumWidth() + deltaX);

        QSettings settings;
        settings.setValue("textPage/textWidth", centralWidgetUi->textEditHolder->maximumWidth());
    });

}

TextView::~TextView()
{
    saveContent();
    saveTextState();
    delete centralWidgetUi;
}

QList<Toolbox *> TextView::toolboxes()
{
    QList<Toolbox *> toolboxes;


    OutlineToolbox *outlineToolbox = new OutlineToolbox;
    toolboxes.append(outlineToolbox);

    connect(this, &TextView::initialized, outlineToolbox, &OutlineToolbox::setIdentifiersAndInitialize);
    outlineToolbox->setIdentifiersAndInitialize(this->projectId(), this->treeItemId());


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



    QSettings settings;

    // restore font size:

    float textFontPointSize = settings.value("textPage/textFontPointSize", -1).toFloat();

    if(textFontPointSize != -1){
        QFont font = centralWidgetUi->textEdit->font();
        float newSize = textFontPointSize;
        if (newSize <= 8)
            newSize = 8;
        font.setPointSizeF(newSize);
        centralWidgetUi->textEdit->setFont(font);
    }

    // restore textWidth:

    int textWidth = settings.value("textPage/textWidth", 600).toInt();
    centralWidgetUi->textEditHolder->setMaximumWidth(textWidth);

    // restore cursor position

    int cursorPosition = SKRUserSettings::getFromProjectSettingHash(this->projectId(), "textCursorPosition", QString::number(this->treeItemId()), 0).toInt();

    QTextCursor cursor(centralWidgetUi->textEdit->document());
    cursor.setPosition(cursorPosition);
    centralWidgetUi->textEdit->setTextCursor(cursor);
    centralWidgetUi->textEdit->ensureCursorVisible();

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

void TextView::saveTextState()
{
    SKRUserSettings::insertInProjectSettingHash(this->projectId(), "textCursorPosition", QString::number(this->treeItemId()),
                                                                     centralWidgetUi->textEdit->textCursor().position());

}

void TextView::mousePressEvent(QMouseEvent *event)
{

    centralWidgetUi->textEdit->setFocus();
    centralWidgetUi->textEdit->ensureCursorVisible();
}

void TextView::wheelEvent(QWheelEvent *event)
{
    if(event->modifiers() == Qt::ControlModifier) {

        QPoint numPixels = event->pixelDelta();
        QPoint numDegrees = event->angleDelta() / 8;

        if(centralWidgetUi->textEdit->font().pointSize() < 8){
            event->accept();
            return;
        }
        if (!numPixels.isNull()) {
            if(numPixels.y() > 0) {
                centralWidgetUi->textEdit->zoomOut();
            }
            else{
                centralWidgetUi->textEdit->zoomIn();
            }
        } else if (!numDegrees.isNull()) {
            QPoint numSteps = numDegrees / 15;
            if(numSteps.y() > 0) {
                centralWidgetUi->textEdit->zoomOut();
            }
            else{
                centralWidgetUi->textEdit->zoomIn();
            }
        }


        QSettings settings;
        settings.setValue("textPage/textFontPointSize", centralWidgetUi->textEdit->font().pointSizeF());

        event->accept();
    }
    else {
            centralWidgetUi->textEdit->wheelEvent(event);
    }
}

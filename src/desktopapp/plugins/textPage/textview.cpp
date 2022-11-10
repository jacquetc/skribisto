#include "textview.h"
#include "desktopapplication.h"
#include "text/markdowntextdocument.h"
#include "projecttreecommands.h"
#include "skrusersettings.h"
#include "text/textbridge.h"
#include "ui_textview.h"
#include "toolboxes/outlinetoolbox.h"

#include <QTimer>
#include <QWheelEvent>
#include <skrdata.h>

TextView::TextView(QWidget *parent) :
    View("TEXT",parent),
    centralWidgetUi(new Ui::TextView), m_isSecondaryContent(false), m_wasModified(false)
{

    //central ui
    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);
    setCentralWidget(centralWidget);

    //toolbar

    QToolBar *toolBar = new QToolBar;
    QAction *showFontToolBar = new QAction(QIcon(":/icons/backup/format-text-italic.svg"), "Show font toolbar",this);
    showFontToolBar->setCheckable(true);
    toolBar->addAction(showFontToolBar);
    toolBar->addAction(centralWidgetUi->textEdit->centerCursorAction());

    QToolBar *fontToolBar = new QToolBar(this);
    fontToolBar->hide();

    fontToolBar->addAction(centralWidgetUi->textEdit->italicAction());
    fontToolBar->addAction(centralWidgetUi->textEdit->boldAction());
    fontToolBar->addAction(centralWidgetUi->textEdit->underlineAction());
    fontToolBar->addAction(centralWidgetUi->textEdit->strikeAction());

    fontToolBar->addSeparator();
    fontToolBar->addAction(centralWidgetUi->textEdit->bulletListAction());

    QObject::connect(showFontToolBar, &QAction::triggered, this, [this, fontToolBar](bool checked){
        if(checked){
            this->setSecondToolBar(fontToolBar);
        }
        this->setSecondToolBarVisible(checked);
    });


    setToolBar(toolBar);




    // link scrollbar to textedit

    centralWidgetUi->horizontalLayout->addWidget(centralWidgetUi->textEdit->verticalScrollBar());
    delete centralWidgetUi->verticalScrollBar;
    centralWidgetUi->verticalScrollBar = centralWidgetUi->textEdit->verticalScrollBar();
    centralWidgetUi->verticalScrollBar->hide();

    connect(centralWidgetUi->verticalScrollBar, &QScrollBar::rangeChanged, this, [&](int min, int max){
        centralWidgetUi->verticalScrollBar->setVisible(min < max + centralWidgetUi->textEdit->viewport()->height() / 2);
    });

    //

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

    connect(this, &TextView::aboutToBeDestroyed, this, [this](){
        saveTextState();
    });

}

TextView::~TextView()
{

    if(m_wasModified){
        saveContent(true);
    }

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
    MarkdownTextDocument *document = new MarkdownTextDocument(this);

    m_isSecondaryContent = parameters().value("is_secondary_content", false).toBool();
    if(m_isSecondaryContent){
        document->setSkribistoMarkdown(skrdata->treeHub()->getSecondaryContent(this->projectId(), this->treeItemId()));

    }
    else {
        document->setSkribistoMarkdown(skrdata->treeHub()->getPrimaryContent(this->projectId(), this->treeItemId()));

    }

    centralWidgetUi->textEdit->setDocument(document);

    QString uniqueDocumentReference = QString("%1_%2_%3").arg(this->projectId()).arg(this->treeItemId()).arg(m_isSecondaryContent ? "secondary" : "primary");
    textBridge->subscribeTextDocument(
                uniqueDocumentReference,
                centralWidgetUi->textEdit->uuid(),
                document);


    //centralWidgetUi->textEdit->adaptScollBarRange(centralWidgetUi->verticalScrollBar->minimum(), centralWidgetUi->verticalScrollBar->maximum());

    // create save timer

    m_saveTimer = new QTimer(this);
    m_saveTimer->setSingleShot(true);
    m_saveTimer->setInterval(200);
    connect(m_saveTimer, &QTimer::timeout, this, [this](){ saveContent();});
    QTimer::singleShot(0, this, [this](){ connectSaveConnection();});

    QSettings settings;

    m_highlighter = new Highlighter(document);
    m_highlighter->setProjectId(this->projectId());
    m_highlighter->getSpellChecker()->setLangCode(skrdata->projectHub()->getLangCode(this->projectId()));
    m_highlighter->setSpellCheckHighlightColor("#FF0000");
    m_highlighter->getSpellChecker()->activate(settings.value("common/spellChecker", true).toBool());


    // restore font size and family:

    int textFontPointSize = settings.value("textPage/textFontPointSize", qApp->font().pointSize()).toInt();
    QString textFontFamily = settings.value("textPage/textFontFamily", qApp->font().family()).toString();

    QFont font = centralWidgetUi->textEdit->font();
    font.setPointSize(textFontPointSize);
    font.setFamily(textFontFamily);
    centralWidgetUi->textEdit->setFont(font);

    // restore font block format:

    QTextBlockFormat blockFormat;
    blockFormat.setTopMargin(settings.value("textPage/paragraphTopMargin", 12).toInt());
    blockFormat.setTextIndent(settings.value("textPage/paragraphFirstLineIndent", 12).toInt());

    QTextCursor textCursor = centralWidgetUi->textEdit->textCursor();
    textCursor.movePosition(QTextCursor::Start, QTextCursor::MoveAnchor);
    textCursor.movePosition(QTextCursor::End, QTextCursor::KeepAnchor);
    textCursor.mergeBlockFormat(blockFormat);

    // restore textWidth:

    int textWidth = settings.value("textPage/textWidth", 600).toInt();
    centralWidgetUi->textEditHolder->setMaximumWidth(textWidth);

    // restore y position


    QTimer::singleShot(0, this, [this](){

        QSettings settings;
        // restore cursor always centered:
        bool textCursorCentered = SKRUserSettings::getFromProjectSettingHash(this->projectId(), "textCursorAlwaysCentered", QString::number(this->treeItemId()), 0).toBool();
        bool textCursorAlwaysCentered = settings.value("textPage/alwaysCenterTheCursor", false).toBool();
        if(textCursorCentered || textCursorAlwaysCentered){
            centralWidgetUi->textEdit->centerCursorAction()->setChecked(true);
        }

        this->connect(centralWidgetUi->textEdit->centerCursorAction(), &QAction::triggered, this, [this](bool checked){
            QSettings settings;
            settings.setValue("textPage/textCursorAlwaysCentered", checked);
        });


        int scrollBarValue = SKRUserSettings::getFromProjectSettingHash(this->projectId(), "textScrollBarValue", QString::number(this->treeItemId()), 0).toInt();
        centralWidgetUi->verticalScrollBar->setValue(scrollBarValue);
       // centralWidgetUi->textEdit->ensureCursorVisible();

        // restore cursor position

        int cursorPosition = SKRUserSettings::getFromProjectSettingHash(this->projectId(), "textCursorPosition", QString::number(this->treeItemId()), 0).toInt();

        QTextCursor cursor(centralWidgetUi->textEdit->document());
        cursor.setPosition(cursorPosition);
        centralWidgetUi->textEdit->setTextCursor(cursor);

    });



}

void TextView::saveContent(bool sameThread)
{

    QString markdown = static_cast<MarkdownTextDocument *>(centralWidgetUi->textEdit->document())->toSkribistoMarkdown();

    projectTreeCommands->setContent(this->projectId(), this->treeItemId(), markdown, m_isSecondaryContent);

    if(!m_isSecondaryContent){
        projectTreeCommands->updateCharAndWordCount(this->projectId(), this->treeItemId(), "TEXT", sameThread);
    }

    m_wasModified = false;

}

void TextView::saveTextState()
{
    SKRUserSettings::insertInProjectSettingHash(this->projectId(), "textCursorPosition", QString::number(this->treeItemId()),
                                                                     centralWidgetUi->textEdit->textCursor().position());
    SKRUserSettings::insertInProjectSettingHash(this->projectId(), "textScrollBarValue", QString::number(this->treeItemId()),
                                                                     centralWidgetUi->verticalScrollBar->value());
    //qDebug() << "saveTextState" << centralWidgetUi->verticalScrollBar->value();

}

void TextView::connectSaveConnection()
{


    m_saveConnection = connect(centralWidgetUi->textEdit, &QTextEdit::textChanged, this, [this](){
        m_wasModified = true;

        if(m_saveTimer->isActive()){
            m_saveTimer->stop();
        }
        m_saveTimer->start();
    });

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


        QObject::disconnect(m_saveConnection);

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

        if(centralWidgetUi->textEdit->font().pointSize() < 8){

            QFont font = centralWidgetUi->textEdit->font();
            font.setPointSize(8);
            centralWidgetUi->textEdit->setFont(font);

            event->accept();
        }

        centralWidgetUi->textEdit->ensureCursorVisible();

        connectSaveConnection();


        QSettings settings;
        settings.setValue("textPage/textFontPointSize", centralWidgetUi->textEdit->font().pointSize());

        QHash<QString, QVariant> newSettings;
        newSettings.insert("textPage/textFontPointSize", centralWidgetUi->textEdit->font().pointSize());
        emit static_cast<DesktopApplication *>(qApp)->settingsChanged(newSettings);

        event->accept();
    }
    else {
            centralWidgetUi->textEdit->wheelEvent(event);
    }
}


void TextView::settingsChanged(const QHash<QString, QVariant> &newSettings)
{
    if(newSettings.contains("textPage/textFontFamily")){
        QString textFontFamily = newSettings.value("textPage/textFontFamily").toString();
        QFont font = centralWidgetUi->textEdit->font();
        font.setFamily(textFontFamily);
        centralWidgetUi->textEdit->setFont(font);
    }
    if(newSettings.contains("textPage/textFontPointSize")){
        int textFontPointSize = newSettings.value("textPage/textFontPointSize").toInt();
        QFont font = centralWidgetUi->textEdit->font();
        font.setPointSize(textFontPointSize);
        centralWidgetUi->textEdit->setFont(font);

    }
    if(newSettings.contains("textPage/paragraphTopMargin")){
        QTextBlockFormat blockFormat;
        blockFormat.setTopMargin(newSettings.value("textPage/paragraphTopMargin").toInt());

        QTextCursor textCursor = centralWidgetUi->textEdit->textCursor();
        textCursor.movePosition(QTextCursor::Start, QTextCursor::MoveAnchor);
        textCursor.movePosition(QTextCursor::End, QTextCursor::KeepAnchor);
        textCursor.mergeBlockFormat(blockFormat);
    }
    if(newSettings.contains("textPage/paragraphFirstLineIndent")){

        QTextBlockFormat blockFormat;
        blockFormat.setTextIndent(newSettings.value("textPage/paragraphFirstLineIndent").toInt());

        QTextCursor textCursor = centralWidgetUi->textEdit->textCursor();
        textCursor.movePosition(QTextCursor::Start, QTextCursor::MoveAnchor);
        textCursor.movePosition(QTextCursor::End, QTextCursor::KeepAnchor);
        textCursor.mergeBlockFormat(blockFormat);
    }

    if(newSettings.contains("textPage/alwaysCenterTheCursor")){
        centralWidgetUi->textEdit->centerCursorAction()->setChecked(newSettings.value("textPage/alwaysCenterTheCursor").toBool());
    }

    if(newSettings.contains("common/spellChecker")){
        m_highlighter->getSpellChecker()->activate(newSettings.value("common/spellChecker").toBool());
    }
}

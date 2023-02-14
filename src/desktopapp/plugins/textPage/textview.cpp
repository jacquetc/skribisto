#include "textview.h"
#include "desktopapplication.h"
#include "projecttreecommands.h"
#include "skrusersettings.h"
#include "text/highlighter.h"
#include "text/markdowntextdocument.h"
#include "text/textbridge.h"
#include "text/textedit.h"
#include "toolboxes/outlinetoolbox.h"
#include "toolboxes/tagtoolbox.h"
#include "ui_textview.h"

#include <QLabel>
#include <QTimer>
#include <QWheelEvent>
#include <skrdata.h>

TextView::TextView(QWidget *parent)
    : View("TEXT", parent), centralWidgetUi(new Ui::TextView), m_isSecondaryContent(false), m_wasModified(false),
      m_oldCursorPosition(-1), m_localWordMeter(new SKRWordMeter(this))
{

    // central ui
    QWidget *centralWidget = new QWidget;
    centralWidgetUi->setupUi(centralWidget);
    setCentralWidget(centralWidget);

    // toolbar

    QToolBar *toolBar = new QToolBar;
    QAction *showFontToolBar = new QAction(QIcon(":/icons/backup/format-text-italic.svg"), "Show font toolbar", this);
    showFontToolBar->setCheckable(true);
    toolBar->addAction(showFontToolBar);
    toolBar->addAction(centralWidgetUi->textEdit->centerCursorAction());
    QLabel *wordCountLabel = new QLabel(this);
    toolBar->addWidget(wordCountLabel);

    QToolBar *fontToolBar = new QToolBar(this);
    fontToolBar->hide();

    fontToolBar->addAction(centralWidgetUi->textEdit->italicAction());
    fontToolBar->addAction(centralWidgetUi->textEdit->boldAction());
    fontToolBar->addAction(centralWidgetUi->textEdit->underlineAction());
    fontToolBar->addAction(centralWidgetUi->textEdit->strikeAction());

    fontToolBar->addSeparator();
    fontToolBar->addAction(centralWidgetUi->textEdit->bulletListAction());

    QObject::connect(showFontToolBar, &QAction::triggered, this, [this, fontToolBar](bool checked) {
        if (checked)
        {
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

    connect(centralWidgetUi->verticalScrollBar, &QScrollBar::rangeChanged, this, [&](int min, int max) {
        centralWidgetUi->verticalScrollBar->setVisible(min < max + centralWidgetUi->textEdit->viewport()->height() / 2);
    });

    //

    centralWidgetUi->textEdit->setWordWrapMode(QTextOption::WrapAtWordBoundaryOrAnywhere);

    centralWidgetUi->horizontalLayout->setStretchFactor(centralWidgetUi->horizontalLayout_2, 1);

    this->setFocusProxy(centralWidgetUi->textEdit);
    centralWidget->setFocusProxy(centralWidgetUi->textEdit);
    centralWidgetUi->widget->setFocusProxy(centralWidgetUi->textEdit);

    connect(centralWidgetUi->sizeHandle, &SizeHandle::moved, this, [this](int deltaX) {
        centralWidgetUi->textEditHolder->setMaximumWidth(centralWidgetUi->textEditHolder->maximumWidth() + deltaX);

        QSettings settings;
        settings.setValue("textPage/textWidth", centralWidgetUi->textEditHolder->maximumWidth());
    });

    connect(this, &TextView::aboutToBeDestroyed, this, [this]() {
        centralWidgetUi->textEdit->blockSignals(true);
        if (m_saveTimer->isActive())
        {
            m_saveTimer->stop();
        }
        saveTextState();
        if (m_wasModified)
        {
            saveContent(true);
        }

        QString uniqueDocumentReference =
            QString("%1_%2_%3")
                .arg(QString::number(this->projectId()), QString::number(this->treeItemAddress().itemId),
                     m_isSecondaryContent ? "secondary" : "primary");
        textBridge->unsubscribeTextDocument(uniqueDocumentReference, centralWidgetUi->textEdit->uuid(),
                                            static_cast<MarkdownTextDocument *>(centralWidgetUi->textEdit->document()));
    });

    //            connect(m_localWordMeter, &SKRWordMeter::characterCountCalculated, skrdata->statHub(),
    //                    &SKRStatHub::updateCharacterStats);
    connect(m_localWordMeter, &SKRWordMeter::wordCountCalculated, this,
            [wordCountLabel](const TreeItemAddress &treeItemAddress, int wordCount, bool triggerProjectModifiedSignal) {

            });

    connect(
        skrdata->treePropertyHub(), &SKRPropertyHub::propertyChanged, this,
        [this, wordCountLabel](int projectId, int propertyId, int treeItemCode, const QString &name,
                               const QString &value) {
            if (name == "word_count" && projectId == this->treeItemAddress().projectId &&
                treeItemCode == this->treeItemAddress().itemId)
            {
                wordCountLabel->setText(tr("%1 words").arg(QLocale().toString(value.toInt())));
            }
        },
        Qt::QueuedConnection);
}

TextView::~TextView()
{

    delete centralWidgetUi;
}

QList<Toolbox *> TextView::toolboxes()
{
    QList<Toolbox *> toolboxes;

    OutlineToolbox *outlineToolbox = new OutlineToolbox;
    toolboxes.append(outlineToolbox);

    connect(this, &TextView::initialized, outlineToolbox, &OutlineToolbox::setIdentifiersAndInitialize);
    outlineToolbox->setIdentifiersAndInitialize(this->treeItemAddress());

    TagToolbox *tagToolbox = new TagToolbox(nullptr, this->treeItemAddress());
    toolboxes.append(tagToolbox);

    return toolboxes;
}

void TextView::initialize()
{
    centralWidgetUi->textEdit->setProjectId(this->projectId());

    MarkdownTextDocument *document = new MarkdownTextDocument(this);

    m_isSecondaryContent = parameters().value("is_secondary_content", false).toBool();
    if (m_isSecondaryContent)
    {
        document->setSkribistoMarkdown(skrdata->treeHub()->getSecondaryContent(this->treeItemAddress()));
    }
    else
    {
        document->setSkribistoMarkdown(skrdata->treeHub()->getPrimaryContent(this->treeItemAddress()));
    }

    centralWidgetUi->textEdit->setDocument(document);

    // centralWidgetUi->textEdit->adaptScollBarRange(centralWidgetUi->verticalScrollBar->minimum(),
    // centralWidgetUi->verticalScrollBar->maximum());

    // create save timer

    m_saveTimer = new QTimer(this);
    m_saveTimer->setSingleShot(true);
    // a bit blocking because of word count counting, so 10s is sufficient
    m_saveTimer->setInterval(100);
    connect(m_saveTimer, &QTimer::timeout, this, [this]() { saveContent(); });
    QTimer::singleShot(0, this, [this]() { connectSaveConnection(); });

    connect(centralWidgetUi->textEdit, &TextEdit::activeFocusChanged, this, [this](int focus) {
        if (!focus)
        {
            // qDebug() << "saving";
            m_saveTimer->stop();
            saveContent();
        }
    });

    // history
    m_historyTimer = new QTimer(this);
    m_historyTimer->setSingleShot(true);
    m_historyTimer->setInterval(2000);
    connect(m_saveTimer, &QTimer::timeout, this, [this]() { addPositionToHistory(); });
    connect(centralWidgetUi->textEdit, &TextEdit::cursorPositionChanged, this, [this]() {
        int newPosition = centralWidgetUi->textEdit->textCursor().position();

        if (m_oldCursorPosition != -1)
        {
            m_historyTimer->stop();
            m_historyTimer->start();

            // handle the case where the new position is far from the old position
            if (qAbs(m_oldCursorPosition - newPosition) > 30)
            {
                addPositionToHistory();
            }
        }

        m_oldCursorPosition = centralWidgetUi->textEdit->textCursor().position();
    });

    //---------------------------------

    QSettings settings;

    // spellchecker :
    centralWidgetUi->textEdit->setupHighlighter();
    centralWidgetUi->textEdit->setSpellcheckerEnabled(settings.value("common/spellChecker", true).toBool());

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
    centralWidgetUi->textEdit->setBlockFormat(blockFormat);

    // restore textWidth:

    int textWidth = settings.value("textPage/textWidth", 600).toInt();
    centralWidgetUi->textEditHolder->setMaximumWidth(textWidth);

    // restore y position

    QTimer::singleShot(0, this, [this, document]() {
        QString uniqueDocumentReference =
            QString("%1_%2_%3")
                .arg(QString::number(this->projectId()), QString::number(this->treeItemAddress().itemId),
                     m_isSecondaryContent ? "secondary" : "primary");
        textBridge->subscribeTextDocument(uniqueDocumentReference, centralWidgetUi->textEdit->uuid(), document);

        QSettings settings;
        // restore cursor always centered:
        bool textCursorCentered =
            SKRUserSettings::getFromProjectSettingHash(this->projectId(), "textCursorAlwaysCentered",
                                                       QString::number(this->treeItemAddress().itemId), 0)
                .toBool();
        bool textCursorAlwaysCentered = settings.value("textPage/alwaysCenterTheCursor", false).toBool();
        if (textCursorCentered || textCursorAlwaysCentered)
        {
            centralWidgetUi->textEdit->centerCursorAction()->setChecked(true);
        }

        this->connect(centralWidgetUi->textEdit->centerCursorAction(), &QAction::triggered, this, [this](bool checked) {
            QSettings settings;
            settings.setValue("textPage/textCursorAlwaysCentered", checked);
        });

        int scrollBarValue =
            SKRUserSettings::getFromProjectSettingHash(this->projectId(), "textScrollBarValue",
                                                       QString::number(this->treeItemAddress().itemId), 0)
                .toInt();
        centralWidgetUi->verticalScrollBar->setValue(scrollBarValue);
        // centralWidgetUi->textEdit->ensureCursorVisible();

        // restore cursor position

        int cursorPosition =
            SKRUserSettings::getFromProjectSettingHash(this->projectId(), "textCursorPosition",
                                                       QString::number(this->treeItemAddress().itemId), 0)
                .toInt();

        QTextCursor cursor(centralWidgetUi->textEdit->document());
        cursor.setPosition(cursorPosition);
        centralWidgetUi->textEdit->setTextCursor(cursor);
    });

    // centralWidgetUi->textEdit->setCursorWidth(2);
}

void TextView::saveContent(bool sameThread)
{
    // qDebug() << "s";
    const QString &markdown =
        static_cast<MarkdownTextDocument *>(centralWidgetUi->textEdit->document())->toSkribistoMarkdown();
    projectTreeCommands->setContent(this->treeItemAddress(), std::move(markdown), m_isSecondaryContent);

    if (!m_isSecondaryContent)
    {
        projectTreeCommands->updateCharAndWordCount(this->treeItemAddress(), "TEXT", sameThread);
    }

    m_wasModified = false;
}

void TextView::saveTextState()
{
    SKRUserSettings::insertInProjectSettingHash(this->projectId(), "textCursorPosition",
                                                QString::number(this->treeItemAddress().itemId),
                                                centralWidgetUi->textEdit->textCursor().position());
    SKRUserSettings::insertInProjectSettingHash(this->projectId(), "textScrollBarValue",
                                                QString::number(this->treeItemAddress().itemId),
                                                centralWidgetUi->verticalScrollBar->value());
    // qDebug() << "saveTextState" << centralWidgetUi->verticalScrollBar->value();
}

void TextView::connectSaveConnection()
{

    m_saveConnection = connect(centralWidgetUi->textEdit, &QTextEdit::textChanged, this, [this]() {
        m_wasModified = true;

        // m_localWordMeter->countText(this->treeItemAddress(), std::move(centralWidgetUi->textEdit->toPlainText()),
        // true,false);

        if (m_saveTimer->isActive())
        {
            m_saveTimer->stop();
        }
        if (this->hasFocus())
        {
            m_saveTimer->start();
        }
    });
}
//---------------------------------------------

void TextView::addPositionToHistory()
{
    QVariantMap parameters;
    parameters.insert("comparedParameterKey", "textCursorPosition");
    parameters.insert("textScrollBarValue", centralWidgetUi->verticalScrollBar->value());
    parameters.insert("textCursorPosition", centralWidgetUi->textEdit->textCursor().position());

    emit this->addToHistoryCalled(this, parameters);
}

//---------------------------------------------

void TextView::mousePressEvent(QMouseEvent *event)
{

    centralWidgetUi->textEdit->setFocus();
    centralWidgetUi->textEdit->ensureCursorVisible();
}

void TextView::wheelEvent(QWheelEvent *event)
{
    if (event->modifiers() == Qt::ControlModifier)
    {

        QPoint numPixels = event->pixelDelta();
        QPoint numDegrees = event->angleDelta() / 8;

        QObject::disconnect(m_saveConnection);

        if (!numPixels.isNull())
        {
            if (numPixels.y() > 0)
            {
                centralWidgetUi->textEdit->zoomOut();
            }
            else
            {
                centralWidgetUi->textEdit->zoomIn();
            }
        }
        else if (!numDegrees.isNull())
        {
            QPoint numSteps = numDegrees / 15;
            if (numSteps.y() > 0)
            {
                centralWidgetUi->textEdit->zoomOut();
            }
            else
            {
                centralWidgetUi->textEdit->zoomIn();
            }
        }

        if (centralWidgetUi->textEdit->font().pointSize() < 8)
        {

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
    else
    {
        centralWidgetUi->textEdit->wheelEvent(event);
    }
}

void TextView::applySettingsChanges(const QHash<QString, QVariant> &newSettings)
{
    if (newSettings.contains("textPage/textFontFamily"))
    {
        QString textFontFamily = newSettings.value("textPage/textFontFamily").toString();
        QFont font = centralWidgetUi->textEdit->font();
        font.setFamily(textFontFamily);
        centralWidgetUi->textEdit->setFont(font);
    }
    if (newSettings.contains("textPage/textFontPointSize"))
    {
        int textFontPointSize = newSettings.value("textPage/textFontPointSize").toInt();
        QFont font = centralWidgetUi->textEdit->font();
        font.setPointSize(textFontPointSize);
        centralWidgetUi->textEdit->setFont(font);
    }
    if (newSettings.contains("textPage/paragraphTopMargin"))
    {
        QTextBlockFormat blockFormat;
        blockFormat.setTopMargin(newSettings.value("textPage/paragraphTopMargin").toInt());

        QTextCursor textCursor = centralWidgetUi->textEdit->textCursor();
        textCursor.movePosition(QTextCursor::Start, QTextCursor::MoveAnchor);
        textCursor.movePosition(QTextCursor::End, QTextCursor::KeepAnchor);
        textCursor.mergeBlockFormat(blockFormat);

        centralWidgetUi->textEdit->document()->clearUndoRedoStacks();
    }
    if (newSettings.contains("textPage/paragraphFirstLineIndent"))
    {

        QTextBlockFormat blockFormat;
        blockFormat.setTextIndent(newSettings.value("textPage/paragraphFirstLineIndent").toInt());

        QTextCursor textCursor = centralWidgetUi->textEdit->textCursor();
        textCursor.movePosition(QTextCursor::Start, QTextCursor::MoveAnchor);
        textCursor.movePosition(QTextCursor::End, QTextCursor::KeepAnchor);
        textCursor.mergeBlockFormat(blockFormat);

        centralWidgetUi->textEdit->document()->clearUndoRedoStacks();
    }

    if (newSettings.contains("textPage/alwaysCenterTheCursor"))
    {
        centralWidgetUi->textEdit->centerCursorAction()->setChecked(
            newSettings.value("textPage/alwaysCenterTheCursor").toBool());
    }

    if (newSettings.contains("common/spellChecker"))
    {
        centralWidgetUi->textEdit->setSpellcheckerEnabled(newSettings.value("common/spellChecker").toBool());
    }
}

void TextView::applyParameters()
{
    QVariantMap parameters = this->parameters();

    QTimer::singleShot(0, this, [=]() {
        int scrollBarValue = parameters.value("textScrollBarValue").toInt();
        centralWidgetUi->verticalScrollBar->setValue(scrollBarValue);
        // centralWidgetUi->textEdit->ensureCursorVisible();

        // restore cursor position

        int cursorPosition = parameters.value("textCursorPosition").toInt();

        QTextCursor cursor(centralWidgetUi->textEdit->document());
        cursor.setPosition(cursorPosition);
        centralWidgetUi->textEdit->setTextCursor(cursor);

        addPositionToHistory();
    });
}

//------------------------------------------------------

QVariantMap TextView::addOtherViewParametersBeforeSplit()
{
    QVariantMap parameters;

    parameters.insert("textScrollBarValue", centralWidgetUi->verticalScrollBar->value());
    parameters.insert("textCursorPosition", centralWidgetUi->textEdit->textCursor().position());

    return parameters;
}

void TextView::applyHistoryParameters(const QVariantMap &parameters)
{

    int scrollBarValue = parameters.value("textScrollBarValue").toInt();
    centralWidgetUi->verticalScrollBar->setValue(scrollBarValue);

    // restore cursor position

    int cursorPosition = parameters.value("textCursorPosition").toInt();

    QTextCursor cursor(centralWidgetUi->textEdit->document());
    cursor.setPosition(cursorPosition);
    centralWidgetUi->textEdit->setTextCursor(cursor);

    centralWidgetUi->textEdit->ensureCursorVisible();
}

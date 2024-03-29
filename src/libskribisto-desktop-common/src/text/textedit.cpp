#include "textedit.h"
#include "dictcommands.h"
#include "invoker.h"
#include "projecttreecommands.h"
#include "skrdata.h"
#include "viewmanager.h"

#include <QClipboard>
#include <QInputDialog>
#include <QMimeData>
#include <QScrollBar>
#include <QTextBlock>
#include <QTextCursor>
#include <QTextDocumentFragment>
#include <QTextList>
#include <QUuid>

TextEdit::TextEdit(QWidget *parent, int projectId)
    : m_mouse_button_down(false), m_always_center_cursor(false), m_forceDisableCenterCursor(false),
      m_projectId(projectId), m_highlighter(nullptr)
{
    this->setMouseTracking(true);
    this->setAutoFormatting(QTextEdit::AutoNone);

    m_uuid = QUuid::createUuid().toString();

    setupContextMenu();

    connect(this, &QTextEdit::cursorPositionChanged, this, &TextEdit::updateFontActions);

    m_italicAction = new QAction(QIcon(":/icons/backup/format-text-italic.svg"), tr("Italic"), this);
    m_italicAction->setShortcut(QKeySequence(tr("Ctrl+I")));
    m_italicAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    m_italicAction->setCheckable(true);
    this->addAction(m_italicAction);

    m_boldAction = new QAction(QIcon(":/icons/backup/format-text-bold.svg"), tr("Bold"), this);
    m_boldAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    m_boldAction->setShortcut(QKeySequence(tr("Ctrl+B")));
    m_boldAction->setCheckable(true);
    this->addAction(m_boldAction);

    m_strikeAction = new QAction(QIcon(":/icons/backup/format-text-strikethrough.svg"), tr("Strikethrough"), this);
    m_strikeAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    m_strikeAction->setShortcut(QKeySequence(tr("Ctrl+T")));
    m_strikeAction->setCheckable(true);
    this->addAction(m_strikeAction);

    m_underlineAction = new QAction(QIcon(":/icons/backup/format-text-underline.svg"), tr("Underline"), this);
    m_underlineAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    m_underlineAction->setShortcut(QKeySequence(tr("Ctrl+U")));
    m_underlineAction->setCheckable(true);
    this->addAction(m_underlineAction);

    m_bulletListAction = new QAction(QIcon(":/icons/backup/format-list-unordered.svg"), tr("List"), this);
    m_bulletListAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    m_bulletListAction->setShortcut(QKeySequence(tr("")));
    m_bulletListAction->setCheckable(true);
    this->addAction(m_bulletListAction);

    m_centerCursorAction =
        new QAction(QIcon(":/icons/backup/format-align-vertical-center.svg"), tr("Center the cursor"), this);
    m_centerCursorAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    m_centerCursorAction->setShortcut(QKeySequence(tr("Ctrl+Shift+C")));
    m_centerCursorAction->setCheckable(true);
    this->addAction(m_centerCursorAction);

    connectActions();

    // center cursor:

    connect(this, &TextEdit::cursorPositionChanged, this, [this]() {
        if (!m_mouse_button_down)
        {
            this->centerCursor();
        }
    });

    connect(this->verticalScrollBar(), &QScrollBar::rangeChanged, this, &TextEdit::adaptScrollBarRange);

    QTimer::singleShot(0, this, [this]() { this->setCursorWidth(2); });
}

QString TextEdit::uuid() const
{
    return m_uuid;
}

// QSize TextEdit::sizeHint() const
//{
// return QSize(800, 500);
// }

void TextEdit::wheelEvent(QWheelEvent *event)
{

    // disable Ctrl+Wheel from QTextEdit which scrolls
    if (event->modifiers() == Qt::ControlModifier)
    {
        event->ignore();
        return;
    }

    QTextEdit::wheelEvent(event);
}

void TextEdit::connectActions()
{
    m_actionConnectionsList.clear();

    m_actionConnectionsList << QObject::connect(m_italicAction, &QAction::toggled, this, [this](bool checked) {
        QTextCharFormat format;
        format.setFontItalic(checked);
        this->mergeCurrentCharFormat(format);
    });

    m_actionConnectionsList << QObject::connect(m_boldAction, &QAction::toggled, this, [this](bool checked) {
        QTextCharFormat format;
        format.setFontWeight(checked ? 700 : 400);
        this->mergeCurrentCharFormat(format);
    });

    m_actionConnectionsList << QObject::connect(m_underlineAction, &QAction::toggled, this, [this](bool checked) {
        QTextCharFormat format;
        format.setFontUnderline(checked);
        this->mergeCurrentCharFormat(format);
    });

    m_actionConnectionsList << QObject::connect(m_strikeAction, &QAction::toggled, this, [this](bool checked) {
        QTextCharFormat format;
        format.setFontStrikeOut(checked);
        this->mergeCurrentCharFormat(format);
    });

    m_actionConnectionsList << QObject::connect(m_bulletListAction, &QAction::toggled, this, [this](bool checked) {
        QTextCursor cursor(this->document());
        cursor.setPosition(this->textCursor().position());
        cursor.setPosition(this->textCursor().anchor(), QTextCursor::KeepAnchor);

        QTextBlock block = cursor.block();
        QTextList *currentList = cursor.currentList();
        const QTextBlock previousBlock = block.previous();

        QTextCursor previousBlockCursor(this->document());
        previousBlockCursor.setPosition(previousBlock.position());
        QTextList *previousList = previousBlockCursor.currentList();

        // add to previous list if there is one
        if (previousBlock.isValid() && previousList)
        {

            previousList->add(block);
        }
        // remove list if there is one
        else if (currentList)
        {
            QList<QTextBlock> blockList;

            int start = cursor.position();
            int end = cursor.anchor();

            for (int i = start; i <= end; i++)
            {
                QTextBlock thisBlock = this->document()->findBlock(i);
                if (!blockList.contains(thisBlock))
                {
                    blockList.append(thisBlock);
                }
            }

            for (const QTextBlock &blockInSet : blockList)
            {
                currentList->remove(blockInSet);
                QTextBlockFormat format;
                format.setIndent(0);
                cursor.mergeBlockFormat(format);
            }
        }
        else
        { // create a new list
            QTextListFormat listFormat;
            listFormat.setStyle(QTextListFormat::ListDisc);
            cursor.createList(listFormat);
        }
    });

    m_actionConnectionsList << QObject::connect(m_centerCursorAction, &QAction::toggled, this, [this](bool checked) {
        m_always_center_cursor = checked;
        centerCursor();
        adaptScrollBarRange(this->verticalScrollBar()->minimum(), this->verticalScrollBar()->maximum());
    });
}

void TextEdit::disconnectActions()
{

    for (const QMetaObject::Connection &connection : qAsConst(m_actionConnectionsList))
    {
        QObject::disconnect(connection);
    }

    m_actionConnectionsList.clear();
}

QAction *TextEdit::boldAction() const
{
    return m_boldAction;
}

void TextEdit::updateFontActions()
{
    this->disconnectActions();

    const QFont font = this->currentFont();
    QTextCursor cursor(this->document());
    cursor.setPosition(this->textCursor().position());

    m_italicAction->setChecked(font.italic());
    m_boldAction->setChecked(font.bold());
    m_underlineAction->setChecked(font.underline());
    m_strikeAction->setChecked(font.strikeOut());
    m_bulletListAction->setChecked(cursor.currentList() ? true : false);

    this->connectActions();
}

QAction *TextEdit::bulletListAction() const
{
    return m_bulletListAction;
}

QAction *TextEdit::underlineAction() const
{
    return m_underlineAction;
}

QAction *TextEdit::strikeAction() const
{
    return m_strikeAction;
}

QAction *TextEdit::italicAction() const
{
    return m_italicAction;
}

QAction *TextEdit::centerCursorAction() const
{
    return m_centerCursorAction;
}

//---------------------------------------------------------

void TextEdit::centerCursor(bool force)
{
    if (m_forceDisableCenterCursor)
    {
        return;
    }
    const QRect cursor = this->cursorRect();
    const QRect viewport = this->viewport()->rect();
    if (force || m_always_center_cursor || (cursor.bottom() >= viewport.bottom()) || (cursor.top() <= viewport.top()))
    {
        const QPoint offset = viewport.center() - cursor.center();
        QScrollBar *scrollbar = this->verticalScrollBar();
        this->verticalScrollBar()->setValue(scrollbar->value() - offset.y());
    }
}

void TextEdit::adaptScrollBarRange(int min, int max)
{

    this->verticalScrollBar()->blockSignals(true);
    this->verticalScrollBar()->setMaximum(m_always_center_cursor ? max + this->viewport()->height() : max);
    this->verticalScrollBar()->blockSignals(false);
}

//---------------------------------------------------------

void TextEdit::setupHighlighter()
{
    if (m_projectId == -1)
    {
        return;
    }
    if (nullptr == m_highlighter)
    {
        m_highlighter = new Highlighter(this->document(), m_projectId);
    }

    m_highlighter->getSpellChecker()->setLangCode(skrdata->projectHub()->getLangCode(m_projectId));
    m_highlighter->setSpellCheckHighlightColor("#FF0000");
}
//---------------------------------------------------------

void TextEdit::setSpellcheckerEnabled(bool value)
{
    m_highlighter->getSpellChecker()->activate(value);
}
//---------------------------------------------------------

bool TextEdit::isWordMisspelled(int cursorPosition)
{
    QString wordValue = getWordUnderCursor(cursorPosition);
    return !m_highlighter->getSpellChecker()->spell(wordValue);
}

//---------------------------------------------------------

QStringList TextEdit::listSpellSuggestionsAt(int cursorPosition) const
{
    QString wordValue = getWordUnderCursor(cursorPosition);
    return m_highlighter->getSpellChecker()->suggest(wordValue);
}

//---------------------------------------------------------

QString TextEdit::getWordUnderCursor(int position) const
{
    QTextCursor textCursor(this->document());
    textCursor.setPosition(position);
    int positionInBlock = textCursor.positionInBlock();
    textCursor.movePosition(QTextCursor::StartOfBlock);
    textCursor.movePosition(QTextCursor::EndOfBlock, QTextCursor::KeepAnchor);
    QString text = textCursor.selectedText();

    QTextBoundaryFinder wordFinder(QTextBoundaryFinder::Word, text);
    wordFinder.setPosition(positionInBlock);
    int wordStart = wordFinder.toPreviousBoundary();

    int wordLength = wordFinder.toNextBoundary() - wordStart;

    return text.mid(wordStart, wordLength);
}

//---------------------------------------------------------

void TextEdit::pasteWithoutFormatting()
{
    QClipboard *clipboard = QApplication::clipboard();
    const QMimeData *mimeData = clipboard->mimeData();

    if (mimeData->hasText())
    {
        this->insertPlainText(clipboard->text());
    }
    else if (mimeData->hasHtml())
    {
        // strangely, this part is never reached

        QTextDocument document;
        document.setHtml(clipboard->text());
        this->insertPlainText(document.toPlainText());
    }
}
//---------------------------------------------------------

QStringList TextEdit::listSpellSuggestionsAtTextCursor() const
{
    return listSpellSuggestionsAt(this->textCursor().position());
}
//---------------------------------------------------------

void TextEdit::replaceWordAt(int cursorPosition, const QString &newWord)
{
    m_forceDisableCenterCursor = true;

    int wordLength = getWordUnderCursor(cursorPosition).length();

    //    qDebug() << "word" << getWordUnderCursor(cursorPosition);
    //    qDebug() << "wordLength" << wordLength;
    //    qDebug() << "newWord" << newWord;
    QTextCursor textCursor(this->document());
    textCursor.setPosition(cursorPosition);
    textCursor.movePosition(QTextCursor::StartOfWord);
    textCursor.setPosition(textCursor.position() + wordLength, QTextCursor::KeepAnchor);

    textCursor.insertText(newWord);

    m_forceDisableCenterCursor = false;
}

void TextEdit::mousePressEvent(QMouseEvent *event)
{
    m_mouse_button_down = true;
    QTextEdit::mousePressEvent(event);
}

void TextEdit::mouseReleaseEvent(QMouseEvent *event)
{

    if (event->button() == Qt::BackButton)
    {
        event->ignore();
        return;
    }
    if (event->button() == Qt::ForwardButton)
    {
        event->ignore();
        return;
    }

    m_mouse_button_down = false;
    QTextEdit::mouseReleaseEvent(event);
}

void TextEdit::mouseDoubleClickEvent(QMouseEvent *event)
{
    m_mouse_button_down = true;
    QTextEdit::mouseDoubleClickEvent(event);
}

void TextEdit::focusOutEvent(QFocusEvent *event)
{
    emit activeFocusChanged(false);
    m_mouse_button_down = false;
    QTextEdit::focusOutEvent(event);
}

void TextEdit::focusInEvent(QFocusEvent *event)
{
    emit activeFocusChanged(true);
    QTextEdit::focusInEvent(event);
}

//------------------------------------------------------

void TextEdit::keyPressEvent(QKeyEvent *event)
{

    // Keep formatting in new paragraphs
    if (event->matches(QKeySequence::InsertParagraphSeparator))
    {
        textCursor().insertBlock();
        event->accept();
        ensureCursorVisible();
        return;
    }

    QTextEdit::keyPressEvent(event);
}

void TextEdit::setupContextMenu()
{

    m_contextMenu = new QMenu("Context menu", this);
    this->setContextMenuPolicy(Qt::CustomContextMenu);
    connect(this, &TextEdit::customContextMenuRequested, this, &TextEdit::onCustomContextMenu);

    m_contextMenu->setMinimumSize(100, 50);

    m_addToUserDictAction = new QAction(this);
    m_connectionHolder = new QObject(this);

    m_cutAction = new QAction(QIcon(":/icons/backup/edit-cut.svg"), tr("Cut"), this);
    connect(m_cutAction, &QAction::triggered, this, [=]() { this->cut(); });

    m_copyAction = new QAction(QIcon(":/icons/backup/edit-copy.svg"), tr("Copy"), this);
    connect(m_copyAction, &QAction::triggered, this, [=]() { this->copy(); });
    m_pasteAction = new QAction(QIcon(":/icons/backup/edit-paste.svg"), tr("Paste"), this);
    connect(m_pasteAction, &QAction::triggered, this, [=]() { this->paste(); });
    m_pasteWithoutFormattingAction = new QAction(tr("Paste without formatting"), this);
    connect(m_pasteWithoutFormattingAction, &QAction::triggered, this, [=]() { this->pasteWithoutFormatting(); });
    m_createNote = new QAction(tr("Create a note"), this);
}

void TextEdit::onCustomContextMenu(const QPoint &point)
{
    // clear:
    delete m_connectionHolder;
    m_connectionHolder = new QObject(this);
    m_contextMenu->clear();

    int currentPosition = this->textCursor().position();
    int currentAnchor = this->textCursor().anchor();
    currentPosition = qMin(currentPosition, currentAnchor);
    currentAnchor = qMax(currentPosition, currentAnchor);
    int positionUnderCursor = this->cursorForPosition(point).position();

    // if outside selected text
    if (this->textCursor().hasSelection() && positionUnderCursor < currentPosition &&
        positionUnderCursor > currentAnchor)
    {
        QTextCursor cursor(this->document());
        cursor.setPosition(positionUnderCursor);
        this->setTextCursor(cursor);
    }
    else if (!this->textCursor().hasSelection())
    {
        QTextCursor cursor(this->document());
        cursor.setPosition(positionUnderCursor);
        this->setTextCursor(cursor);
    }

    QString selectedText = this->textCursor().selectedText();

    // spell check:

    if (selectedText.isEmpty() && m_projectId != -1 && this->isWordMisspelled(positionUnderCursor))
    {

        QStringList suggestions = this->listSpellSuggestionsAt(positionUnderCursor);

        for (const QString &suggestion : suggestions)
        {
            QAction *suggestAction = new QAction(suggestion, m_contextMenu);
            connect(suggestAction, &QAction::triggered, this,
                    [=]() { replaceWordAt(positionUnderCursor, suggestion); });
            m_contextMenu->addAction(suggestAction);
        }

        if (!suggestions.isEmpty())
        {
            m_contextMenu->addSeparator();
        }

        m_addToUserDictAction->setText(tr("Add \"%1\" to Dictionary").arg(getWordUnderCursor(positionUnderCursor)));
        m_contextMenu->addAction(m_addToUserDictAction);

        connect(m_addToUserDictAction, &QAction::triggered, m_connectionHolder, [=]() {
            QString wordUnderCursor = getWordUnderCursor(positionUnderCursor);
            dictCommands->addWordToProjectDict(m_projectId, wordUnderCursor);
        });

        m_contextMenu->addSeparator();
    }

    if (!selectedText.isEmpty())
    {
        // m_contextMenu->addSeparator();

        m_contextMenu->addAction(m_cutAction);
        m_contextMenu->addAction(m_copyAction);
    }

    m_contextMenu->addAction(m_pasteAction);
    m_contextMenu->addAction(m_pasteWithoutFormattingAction);

    if (m_projectId != -1)
    {
        m_contextMenu->addSeparator();
        m_contextMenu->addAction(m_createNote);

        connect(m_createNote, &QAction::triggered, m_connectionHolder, [=]() {
            bool ok = false;
            QString noteName = QInputDialog::getText(this, tr("Create a note"), "", QLineEdit::Normal,
                                                     selectedText.left(30).trimmed(), &ok);

            if (ok)
            {
                TreeItemAddress newNoteAddress = projectTreeCommands->addNote(m_projectId, noteName);
                ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
                viewManager->openViewInAnotherViewHolder("TEXT", newNoteAddress);
            }
        });
    }

    m_contextMenu->popup(this->viewport()->mapToGlobal(point));
}

int TextEdit::projectId() const
{
    return m_projectId;
}

void TextEdit::setProjectId(int newProjectId)
{
    m_projectId = newProjectId;
}

//------------------------------------------------------

bool TextEdit::canInsertFromMimeData(const QMimeData *source) const
{
    if (source->hasHtml())
    {
        return true;
    }

    if (source->hasText())
    {
        return true;
    }

    return false;
}

void TextEdit::insertFromMimeData(const QMimeData *source)
{
    if (source->hasHtml())
    {
        // remove style

        QString html = source->html();
        QStringList styleToRemoveList;
        styleToRemoveList << "font-family";
        styleToRemoveList << "font-size";
        //    styleToRemoveList << "font-style";
        styleToRemoveList << "line-height";
        styleToRemoveList << "margin-left";
        styleToRemoveList << "margin-right";
        styleToRemoveList << "margin-top";
        styleToRemoveList << "margin-bottom";
        styleToRemoveList << "-qt-block-indent";
        styleToRemoveList << "-qt-user-state";
        styleToRemoveList << "text-indent";
        styleToRemoveList << "color";

        for (const QString &style : qAsConst(styleToRemoveList))
            html.remove(QRegularExpression(style + ":.*?;?"));
        static QRegularExpression headers("<h[0-9].*?>|<h[0-9]>|</h[0-9]>");
        html.remove(headers);

        // remove align

        static QRegularExpression align("align=\\\".*?\\\"");
        html.remove(align);

        // remove div

        static QRegularExpression div("<div.*?>|</div>");
        html.remove(div);

        // remove table:

        static QRegularExpression table("<table.*?>|</table>|<div.*?>|</div>|<tbody>|</tbody>|<(tr|td)>|</(tr|td)>");
        html.remove(table);

        qDebug() << "html";

        // apply char format
        QTextDocument document;
        document.setHtml(html);
        QTextCursor textCursor(&document);
        textCursor.select(QTextCursor::SelectionType::Document);
        textCursor.mergeBlockFormat(m_blockFormat);

        QTextCharFormat charFormat;
        charFormat.setFont(this->font(), QTextCharFormat::FontPropertiesSpecifiedOnly);
        textCursor.mergeCharFormat(charFormat);
        QTextDocumentFragment fragment(&document);

        this->insertHtml(fragment.toHtml());

        // emit textPasted();
    }
    else if (source->hasText())
    {
        QTextEdit::insertFromMimeData(source);
        // emit textPasted();
    }
}

//---------------------------------------------

void TextEdit::setBlockFormat(const QTextBlockFormat &blockFormat)
{
    if (m_blockFormat != blockFormat)
    {
        m_blockFormat = blockFormat;

        QTextCursor textCursor = this->textCursor();
        textCursor.movePosition(QTextCursor::Start, QTextCursor::MoveAnchor);
        textCursor.movePosition(QTextCursor::End, QTextCursor::KeepAnchor);
        textCursor.mergeBlockFormat(blockFormat);
        this->document()->clearUndoRedoStacks();
    }
}

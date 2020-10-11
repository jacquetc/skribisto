

#include "documenthandler.h"

#include <QTextBlock>
#include <QTextObject>
#include <QTextCharFormat>
#include <QTextList>
#include <QImageReader>
#include <QTextDocumentWriter>


// #include "plmdata.h"

DocumentHandler::DocumentHandler(QObject *parent) :
    QObject(parent),
    m_textDoc(nullptr),
    m_formatPosition(-2),
    m_paperId(-1)
{}

QQuickTextDocument * DocumentHandler::textDocument() const
{
    return m_textDoc;
}

void DocumentHandler::setTextDocument(QQuickTextDocument *textDocument)
{
    if (m_textDoc) {
        disconnect(m_textDoc->textDocument());
    }

    m_textDoc = textDocument;
    emit textDocumentChanged();

    if (m_textDoc) {
        connect(m_textDoc->textDocument(),
                &QTextDocument::undoAvailable,
                this,
                &DocumentHandler::canUndoChanged);
        connect(m_textDoc->textDocument(),
                &QTextDocument::redoAvailable,
                this,
                &DocumentHandler::canRedoChanged);
        m_textCursor =
            textDocument->textDocument()->rootFrame()->firstCursorPosition();
        m_selectionCursor =
            textDocument->textDocument()->rootFrame()->firstCursorPosition();

        m_highlighter = new SKRHighlighter(m_textDoc->textDocument());

    } else {
        m_textCursor.setPosition(0);
    }
    emit cursorPositionChanged();
}

QStringList DocumentHandler::allFontFamilies() const
{
    QFontDatabase db;

    return db.families();
}

int DocumentHandler::cursorPosition() const
{
    return m_textCursor.position();
}

void DocumentHandler::setCursorPosition(int position)
{
    if ((m_formatPosition >= 0) && (m_textCursor.position() == position)) {
        m_textCursor.movePosition(QTextCursor::PreviousCharacter,
                                  QTextCursor::KeepAnchor,
                                  position - m_formatPosition);
        m_textCursor.setCharFormat(m_nextFormat);
    }
    m_textCursor.setPosition(position);
    m_formatPosition = -2;
    m_nextFormat     = m_textCursor.charFormat();

    emit formatChanged();
    emit cursorPositionChanged();
}

int DocumentHandler::selectionStart() const
{
    return m_selectionCursor.selectionStart();
}

void DocumentHandler::setSelectionStart(int selectionStart)
{
    m_selectionStart = selectionStart;
    m_selectionCursor.setPosition(m_selectionStart, QTextCursor::MoveAnchor);
    m_selectionCursor.setPosition(m_selectionEnd,   QTextCursor::KeepAnchor);
}

int DocumentHandler::selectionEnd() const
{
    return m_selectionCursor.selectionEnd();
}

void DocumentHandler::setSelectionEnd(int selectionEnd)
{
    m_selectionEnd = selectionEnd;
    m_selectionCursor.setPosition(m_selectionStart, QTextCursor::MoveAnchor);
    m_selectionCursor.setPosition(m_selectionEnd,   QTextCursor::KeepAnchor);
}

QString DocumentHandler::fontFamily() const
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        if (m_formatPosition != -2) {
            return m_nextFormat.font().family();
        }
        return m_textCursor.charFormat().font().family();
    } else {
        return m_selectionCursor.charFormat().font().family();
    }
}

void DocumentHandler::setFontFamily(const QString& fontFamily)
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        m_nextFormat.setFontFamily(fontFamily);
        m_formatPosition = m_textCursor.position();
    } else {
        QTextCharFormat f = m_selectionCursor.charFormat();
        f.setFontFamily(fontFamily);
        m_selectionCursor.mergeCharFormat(f);
    }
    emit formatChanged();
}

qreal DocumentHandler::fontSize() const
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        if (m_formatPosition != 2) {
            return m_nextFormat.fontPointSize();
        }
        return m_textCursor.charFormat().fontPointSize();
    } else {
        return m_selectionCursor.charFormat().fontPointSize();
    }
}

void DocumentHandler::setFontSize(qreal fontSize)
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        m_nextFormat.setFontPointSize(fontSize);
        m_formatPosition = m_textCursor.position();
    } else {
        QTextCharFormat f = m_selectionCursor.charFormat();
        f.setFontPointSize(fontSize);
        m_selectionCursor.mergeCharFormat(f);
    }
    emit formatChanged();
}

bool DocumentHandler::italic() const
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        if (m_formatPosition != -2) {
            return m_nextFormat.fontItalic();
        }
        return m_textCursor.charFormat().fontItalic();
    } else {
        int start = m_selectionCursor.anchor() + 1;
        int end   = m_selectionCursor.position();

        bool result = true;
        QTextCursor cursor(m_textDoc->textDocument());

        for (int i = start; i <= end; i++) {
            cursor.setPosition(i);
            result = cursor.charFormat().fontItalic();

            if (!result) {
                break;
            }
        }

        return result;
    }
}

void DocumentHandler::setItalic(bool italic)
{
    qDebug() << "set italic";

    if (m_selectionCursor.selectedText().isEmpty()) {
        m_nextFormat.setFontItalic(italic);
        m_formatPosition = m_textCursor.position();
    } else {
        QTextCharFormat f = m_selectionCursor.charFormat();
        f.setFontItalic(italic);
        m_selectionCursor.mergeCharFormat(f);

        // for test purpose:
        //        QUrl uri ("mydata://skibisto.svg"  );
        //        QImage image = QImageReader (":/pics/skribisto.svg").read();
        //        m_textDoc->textDocument()->addResource(
        // QTextDocument::ImageResource, uri, QVariant ( image ) );
        //        QTextImageFormat imageFormat;
        //        imageFormat.setName(uri.toString());
        //        imageFormat.setHeight(200);
        //        imageFormat.setWidth(200);
        //        imageFormat.setAnchor(true);
        //        imageFormat.setAnchorHref("http://qt.io");


        //        if(imageFormat.isValid()){
        //            m_textCursor.insertImage(imageFormat);
        //        }

        //        QTextDocumentWriter writer;
        //        writer.setFileName("/home/cyril/Documents/rrrrr.odt");
        //        writer.setFormat("odf");
        //        writer.write(m_textDoc->textDocument());
    }
    emit formatChanged();
}

bool DocumentHandler::bold() const
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        if (m_formatPosition != -2) {
            return m_nextFormat.fontWeight() >= QFont::Bold;
        }
        return m_textCursor.charFormat().fontWeight() >= QFont::Bold;
    } else {
        int start = m_selectionCursor.anchor() + 1;
        int end   = m_selectionCursor.position();

        bool result = true;
        QTextCursor cursor(m_textDoc->textDocument());

        for (int i = start; i <= end; i++) {
            cursor.setPosition(i);
            result = cursor.charFormat().fontWeight() >= QFont::Bold;

            if (!result) {
                break;
            }
        }

        return result;
    }
}

void DocumentHandler::setBold(bool bold)
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        qDebug() << "m_selectionCursor" << m_selectionCursor.selectedText();
        qDebug() << "m_selectionCursor" << m_selectionCursor.selectionStart();
        qDebug() << "m_selectionCursor" << m_selectionCursor.selectionEnd();
        m_nextFormat.setFontWeight(bold ? QFont::Bold : QFont::Normal);
        m_formatPosition = m_textCursor.position();
    } else {
        QTextCharFormat f = m_selectionCursor.charFormat();
        f.setFontWeight(bold ? QFont::Bold : QFont::Normal);
        m_selectionCursor.mergeCharFormat(f);
    }
    emit formatChanged();
}

bool DocumentHandler::underline() const
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        if (m_formatPosition != -2) {
            return m_nextFormat.fontUnderline();
        }
        return m_textCursor.charFormat().fontUnderline();
    } else {
        int start = m_selectionCursor.anchor() + 1;
        int end   = m_selectionCursor.position();

        bool result = true;
        QTextCursor cursor(m_textDoc->textDocument());

        for (int i = start; i <= end; i++) {
            cursor.setPosition(i);
            result = cursor.charFormat().fontUnderline();

            if (!result) {
                break;
            }
        }

        return result;
    }
}

void DocumentHandler::setUnderline(bool underline)
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        m_nextFormat.setFontUnderline(underline);
        m_formatPosition = m_textCursor.position();
    } else {
        QTextCharFormat f = m_selectionCursor.charFormat();
        f.setFontUnderline(underline);
        m_selectionCursor.mergeCharFormat(f);
    }
    emit formatChanged();
}

bool DocumentHandler::strikeout() const
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        if (m_formatPosition != -2) {
            return m_nextFormat.fontStrikeOut();
        }
        return m_textCursor.charFormat().fontStrikeOut();
    } else {
        int start = m_selectionCursor.anchor() + 1;
        int end   = m_selectionCursor.position();

        bool result = true;
        QTextCursor cursor(m_textDoc->textDocument());

        for (int i = start; i <= end; i++) {
            cursor.setPosition(i);
            result = cursor.charFormat().fontStrikeOut();

            if (!result) {
                break;
            }
        }

        return result;
    }
}

void DocumentHandler::setStrikeout(bool strikeout)
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        m_nextFormat.setFontStrikeOut(strikeout);
        m_formatPosition = m_textCursor.position();
    } else {
        QTextCharFormat f = m_selectionCursor.charFormat();
        f.setFontStrikeOut(strikeout);
        m_selectionCursor.mergeCharFormat(f);
    }
    emit formatChanged();
}

QColor DocumentHandler::color() const
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        if (m_formatPosition != -2) {
            return m_nextFormat.foreground().color();
        }
        return m_textCursor.charFormat().foreground().color();
    } else {
        return m_selectionCursor.charFormat().foreground().color();
    }
}

void DocumentHandler::setColor(const QColor& color)
{
    if (m_selectionCursor.selectedText().isEmpty()) {
        m_nextFormat.setForeground(QBrush(color));
        m_formatPosition = m_textCursor.position();
    } else {
        QTextCharFormat f = m_selectionCursor.charFormat();
        f.setForeground(QBrush(color));
        m_selectionCursor.mergeCharFormat(f);
    }
    emit formatChanged();
}

bool DocumentHandler::canUndo() const
{
    return m_textDoc != 0 && m_textDoc->textDocument()->isUndoAvailable();
}

bool DocumentHandler::canRedo() const
{
    return m_textDoc != 0 && m_textDoc->textDocument()->isRedoAvailable();
}

Qt::Alignment DocumentHandler::alignment() const
{
    return m_textCursor.blockFormat().alignment();
}

void DocumentHandler::setAlignment(Qt::Alignment alignment)
{
    QTextBlockFormat f = m_textCursor.blockFormat();

    f.setAlignment(alignment);
    m_textCursor.mergeBlockFormat(f);
    emit formatChanged();
}

bool DocumentHandler::bulletList() const
{
    return m_textCursor.currentList() &&
           m_textCursor.currentList()->format().style() == QTextListFormat::ListDisc;
}

void DocumentHandler::setBulletList(bool bulletList)
{
    if (this->bulletList() && !bulletList) {
        m_textCursor.beginEditBlock();
        m_textCursor.currentList()->remove(m_textCursor.block());
        unindentBlock();
        m_textCursor.endEditBlock();
    } else if (!this->bulletList() && bulletList) {
        QTextListFormat f;
        f.setStyle(QTextListFormat::ListDisc);
        m_textCursor.createList(f);
    }
    emit formatChanged();
}

bool DocumentHandler::numberedList() const
{
    return m_textCursor.currentList() &&
           m_textCursor.currentList()->format().style() == QTextListFormat::ListDecimal;
}

void DocumentHandler::setNumberedList(bool numberedList)
{
    if (this->numberedList() && !numberedList) {
        m_textCursor.beginEditBlock();
        m_textCursor.currentList()->remove(m_textCursor.block());
        unindentBlock();
        m_textCursor.endEditBlock();
    } else if (!this->numberedList() && numberedList) {
        QTextListFormat f;
        f.setStyle(QTextListFormat::ListDecimal);
        m_textCursor.createList(f);
    }
    emit formatChanged();
}

int DocumentHandler::paperId() const
{
    return m_paperId;
}

void DocumentHandler::setId(const int projectId, const int paperId)
{
    if ((paperId == -1) || (projectId  == -1)) {
        return;
    }
    m_projectId = projectId;
    m_paperId   = paperId;

    //    QString text = plmdata->sheetHub()->getContent(projectId, paperId);

    m_selectionCursor.select(QTextCursor::Document);

    //    m_selectionCursor.insertHtml(text);

    emit formatChanged();
    emit idChanged();
}

int DocumentHandler::projectId() const
{
    return m_projectId;
}

int DocumentHandler::maxCursorPosition() const
{
    return m_textDoc->textDocument()->characterCount();
}

qreal DocumentHandler::topMarginEverywhere() const
{
    return m_topMarginEverywhere;
}

void DocumentHandler::setTopMarginEverywhere(qreal topMargin)
{
    if (!m_textDoc) {
        return;
    }

    int previousPosition = m_textCursor.position();

    m_textCursor.setPosition(0);
    int posMax = m_textDoc->textDocument()->rootFrame()->lastCursorPosition().position();

    m_textCursor.setPosition(posMax, QTextCursor::KeepAnchor);

    QTextBlockFormat f = m_textCursor.blockFormat();

    f.setTopMargin(topMargin);
    m_textCursor.mergeBlockFormat(f);
    m_topMarginEverywhere = topMargin;

    m_textCursor.setPosition(previousPosition);

    emit topMarginEverywhereChanged(topMargin);
}

qreal DocumentHandler::indentEverywhere() const
{
    return m_indentEverywhere;
}

void DocumentHandler::setIndentEverywhere(qreal indent)
{
    if (!m_textDoc) {
        return;
    }

    int previousPosition = m_textCursor.position();

    m_textCursor.setPosition(0);
    int posMax = m_textDoc->textDocument()->rootFrame()->lastCursorPosition().position();

    m_textCursor.setPosition(posMax, QTextCursor::KeepAnchor);

    QTextBlockFormat f = m_textCursor.blockFormat();

    f.setTextIndent(indent);
    m_textCursor.mergeBlockFormat(f);
    m_indentEverywhere = indent;

    m_textCursor.setPosition(previousPosition);

    emit indentEverywhereChanged(indent);
}

void DocumentHandler::indentBlock()
{
    QTextBlockFormat f = m_textCursor.blockFormat();

    f.setIndent(f.indent() + 1);
    m_textCursor.setBlockFormat(f);
}

void DocumentHandler::unindentBlock()
{
    QTextBlockFormat f = m_textCursor.blockFormat();

    f.setIndent(f.indent() - 1);
    m_textCursor.setBlockFormat(f);
}

void DocumentHandler::undo()
{
    if (m_textDoc) {
        m_textDoc->textDocument()->undo();
    }
}

void DocumentHandler::redo()
{
    if (m_textDoc) {
        m_textDoc->textDocument()->redo();
    }
}

void DocumentHandler::addHorizontalLine()
{
    m_textCursor.beginEditBlock();
    m_textCursor.insertHtml("____________________");
    m_textDoc->textDocument()->setHtml(m_textDoc->textDocument()->toHtml().replace(QRegExp(
                                                                                       "____________________"),
                                                                                   "<hr/><p></p>"));
    m_textCursor.endEditBlock();
}

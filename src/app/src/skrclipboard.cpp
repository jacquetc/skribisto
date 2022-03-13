#include "skrclipboard.h"
#include "skrrootitem.h"
#include <QTextCursor>
#include <QTextDocument>
#include <QTextBlock>
#include <QGuiApplication>
#include <QClipboard>
#include <QMimeData>

SKRClipboard::SKRClipboard(QObject *parent) : QObject(parent)
{
    m_fontFamily    = qGuiApp->font().family().replace(", ", "");
    m_fontPointSize = qGuiApp->font().pointSize();
    m_textTopMargin = 2;
    m_textIndent    = 2;
}

void SKRClipboard::prepareAndSendLastClipboardText()
{
    QClipboard *clipboard     = QGuiApplication::clipboard();
    const QMimeData *mimeData = clipboard->mimeData();

    QTextDocument originalTextDoc;
    QTextDocument finalTextDoc;


    if (mimeData->hasHtml()) {
        QString subType  = "html";
        QString htmlText = clipboard->text(subType, QClipboard::Clipboard);
        originalTextDoc.setHtml(SKRRootItem::cleanUpHtml(htmlText));
    }
    else if (mimeData->hasText()) {
        QString subType   = "plain";
        QString plainText = clipboard->text(subType, QClipboard::Clipboard);
        originalTextDoc.setPlainText(plainText);
    }


    QTextCursor originalCursor(&originalTextDoc);
    QTextCursor finalCursor(&finalTextDoc);

    for (int i = 0; i < originalTextDoc.blockCount(); i++) {
        QTextBlock textBlock = originalTextDoc.findBlockByNumber(i);

        originalCursor.setPosition(textBlock.position());
        originalCursor.movePosition(QTextCursor::EndOfBlock, QTextCursor::KeepAnchor);

        finalCursor.movePosition(QTextCursor::End);
        finalCursor.insertFragment(originalCursor.selection());

        finalCursor.mergeBlockFormat(m_blockFormat);
        finalCursor.mergeCharFormat(m_charFormat);

        if (i < originalTextDoc.blockCount() - 1) {
            finalCursor.insertBlock(m_blockFormat, m_charFormat);
        }
    }


    qDebug() << "pasted from SKRClipboard";
    emit lastClipboardTextPrepared(QTextDocumentFragment(&finalTextDoc));
}

void SKRClipboard::createFormats() {
    QStringList families;

    families << m_fontFamily;

    m_charFormat.setFontFamilies(families);
    m_charFormat.setFontPointSize(m_fontPointSize);
    m_blockFormat.setTopMargin(m_textTopMargin);
    m_blockFormat.setTextIndent(m_textIndent);
}

DocumentHandler * SKRClipboard::documentHandler() const
{
    return m_documentHandler;
}

void SKRClipboard::setDocumentHandler(DocumentHandler *documentHandler)
{
    m_documentHandler = documentHandler;
    emit documentHandlerChanged(documentHandler);

    connect(this, &SKRClipboard::lastClipboardTextPrepared, documentHandler, &DocumentHandler::insertDocumentFragment);
}

int SKRClipboard::textIndent() const
{
    return m_textIndent;
}

void SKRClipboard::setTextIndent(int textIndent)
{
    m_textIndent = textIndent;
    this->createFormats();
    emit textIndentChanged(textIndent);
}

int SKRClipboard::textTopMargin() const
{
    return m_textTopMargin;
}

void SKRClipboard::setTextTopMargin(int textTopMargin)
{
    m_textTopMargin = textTopMargin;
    this->createFormats();
    emit textTopMarginChanged(textTopMargin);
}

int SKRClipboard::fontPointSize() const
{
    return m_fontPointSize;
}

void SKRClipboard::setFontPointSize(int fontPointSize)
{
    m_fontPointSize = fontPointSize;
    this->createFormats();
    emit fontPointSizeChanged(fontPointSize);
}

QString SKRClipboard::fontFamily() const
{
    return m_fontFamily;
}

void SKRClipboard::setFontFamily(const QString& fontFamily)
{
    m_fontFamily = fontFamily;
    this->createFormats();
    emit fontFamilyChanged(fontFamily);
}

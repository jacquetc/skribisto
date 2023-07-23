#include "document_handler.h"
#include "from_markdown.h"
#include "to_markdown.h"
#include <QTextCharFormat>
#include <QTextCursor>
#include <QTextDocument>
#include <stack>

QQuickTextDocument *DocumentHandler::quickTextDocument() const
{
    return m_quickTextDocument;
}

void DocumentHandler::setQuickTextDocument(QQuickTextDocument *quickTextDocument)
{
    m_quickTextDocument = quickTextDocument;
    m_textDocument = m_quickTextDocument->textDocument();

    // connect contentsChange signal to slot
    connect(m_textDocument, &QTextDocument::contentsChange, this, &DocumentHandler::onContentsChange);

    emit quickTextDocumentChanged();
}

void DocumentHandler::onContentsChange(int position, int charsRemoved, int charsAdded)
{
    // Get the block that was changed
    QTextBlock block = m_textDocument->findBlock(position);

    // Get the markdown string for the block
    QString markdown = Markdown::To::toMarkdown(block);

    // Replace the block with the markdown string
    QTextCursor cursor(block);
    cursor.select(QTextCursor::BlockUnderCursor);
    cursor.removeSelectedText();
    cursor.insertText(markdown);
}

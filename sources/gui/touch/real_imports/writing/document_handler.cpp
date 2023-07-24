#include "document_handler.h"
#include "from_markdown.h"
#include "to_markdown.h"
#include "writing/writing_controller.h"
#include <QTextCharFormat>
#include <QTextCursor>
#include <QTextDocument>
#include <stack>

DocumentHandler::DocumentHandler(QObject *parent)
{
    m_uuidList << QUuid::createUuid();
    m_uuidList << QUuid::createUuid();
}

QQuickTextDocument *DocumentHandler::quickTextDocument() const
{
    return m_quickTextDocument;
}

void DocumentHandler::setQuickTextDocument(QQuickTextDocument *quickTextDocument)
{
    m_quickTextDocument = quickTextDocument;
    m_textDocument = m_quickTextDocument->textDocument();

    m_textDocument->setUndoRedoEnabled(false);

    // connect contentsChange signal to slot
    connect(m_textDocument, &QTextDocument::contentsChange, this, &DocumentHandler::onContentsChange);

    emit quickTextDocumentChanged();
}

void DocumentHandler::onContentsChange(int position, int charsRemoved, int charsAdded)
{
    // Get the block that was changed
    QTextBlock block = m_textDocument->findBlock(position);

    // Get the markdown string for the block
    const QString &markdown = Markdown::To::toMarkdown(block);

    //    // Replace the block with the markdown string
    //    QTextCursor cursor(block);
    //    cursor.select(QTextCursor::BlockUnderCursor);
    //    cursor.removeSelectedText();
    //    cursor.insertText(markdown);

    UpdateSceneParagraphDTO dto;
    dto.setSceneUuid(m_uuidList.at(m_uuid));
    dto.setParagraphId(m_uuid);
    dto.setParagraphUuid(m_uuidList.at(m_uuid));
    dto.setText(markdown);
    dto.setCursorPosition(position);

    Writing::WritingController::instance()->updateSceneParagraph(dto);
}

int DocumentHandler::uuid() const
{

    return m_uuid;
}

void DocumentHandler::setUuid(int newUuid)
{
    m_uuid = newUuid;
}

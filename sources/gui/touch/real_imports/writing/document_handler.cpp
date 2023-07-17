#include "document_handler.h"

QQuickTextDocument *DocumentHandler::textDocument() const
{
    return m_quickTextDocument;
}

void DocumentHandler::setTextDocument(QQuickTextDocument *textDocument)
{
    m_quickTextDocument = textDocument;
    emit textDocumentChanged();
}

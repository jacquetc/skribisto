#include "minimapcanvas.h"


MinimapCanvas::MinimapCanvas(QQuickItem *parent) : QQuickPaintedItem(parent)
{}

// -------------------------------------------------------------------------

QQuickTextDocument * MinimapCanvas::textDocument() const
{
    return m_textDoc;
}

void MinimapCanvas::setTextDocument(QQuickTextDocument *textDocument)
{
    m_textDoc = textDocument;
    emit textDocumentChanged();

    if (m_textDoc) {
        m_textCursor = QTextCursor(m_textDoc->textDocument());
    }
}

// -----------------------------------------------------------

void MinimapCanvas::paint(QPainter *painter)
{
    if (!this->isComponentComplete()) {
        return;
    }
}

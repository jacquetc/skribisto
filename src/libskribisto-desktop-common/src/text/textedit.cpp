#include "textedit.h"

#include <QUuid>

TextEdit::TextEdit(QWidget *parent)
{
    m_uuid = QUuid::createUuid().toString();
}

QString TextEdit::uuid()
{
return m_uuid;
}

//QSize TextEdit::sizeHint() const
//{
//return QSize(800, 500);
//}


void TextEdit::wheelEvent(QWheelEvent *event)
{

    QTextEdit::wheelEvent(event);
}

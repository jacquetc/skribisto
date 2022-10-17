#include "textedit.h"

TextEdit::TextEdit(QWidget *parent)
{

}

//QSize TextEdit::sizeHint() const
//{
//return QSize(800, 500);
//}


void TextEdit::wheelEvent(QWheelEvent *event)
{

    QTextEdit::wheelEvent(event);
}

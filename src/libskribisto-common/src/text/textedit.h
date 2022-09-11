#ifndef TEXTEDIT_H
#define TEXTEDIT_H

#include <QTextEdit>

class TextEdit : public QTextEdit
{
public:
    TextEdit(QWidget *parent = nullptr);
//    QSize sizeHint() const override;
};

#endif // TEXTEDIT_H

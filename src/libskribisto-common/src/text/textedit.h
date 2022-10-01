#ifndef TEXTEDIT_H
#define TEXTEDIT_H

#include <QTextEdit>
#include "skribisto_common_global.h"

class SKRCOMMONEXPORT TextEdit : public QTextEdit
{
public:
    TextEdit(QWidget *parent = nullptr);
//    QSize sizeHint() const override;
};

#endif // TEXTEDIT_H

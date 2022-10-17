#ifndef TEXTEDIT_H
#define TEXTEDIT_H

#include "skribisto_desktop_common_global.h"
#include <QTextEdit>

class SKRDESKTOPCOMMONEXPORT TextEdit : public QTextEdit {
public:
  TextEdit(QWidget *parent = nullptr);
  //    QSize sizeHint() const override;

  // QWidget interface
public:
  void wheelEvent(QWheelEvent *event) override;
};

#endif // TEXTEDIT_H

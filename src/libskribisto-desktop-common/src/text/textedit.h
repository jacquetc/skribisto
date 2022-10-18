#ifndef TEXTEDIT_H
#define TEXTEDIT_H

#include "skribisto_desktop_common_global.h"
#include <QTextEdit>

class SKRDESKTOPCOMMONEXPORT TextEdit : public QTextEdit {
public:
  TextEdit(QWidget *parent = nullptr);
  QString uuid();
  //    QSize sizeHint() const override;

  // QWidget interface
public:
  void wheelEvent(QWheelEvent *event) override;

private:
  QString m_uuid;
};

#endif // TEXTEDIT_H

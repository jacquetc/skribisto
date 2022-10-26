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

  QAction *boldAction() const;
  QAction *italicAction() const;
  QAction *strikeAction() const;
  QAction *underlineAction() const;

private slots:
  void updateFontActions();

private:
  QString m_uuid;
  QAction *m_boldAction, *m_italicAction, *m_strikeAction, *m_underlineAction;
  QList<QMetaObject::Connection> m_actionConnectionsList;

  void connectActions();
  void disconnectActions();
};

#endif // TEXTEDIT_H

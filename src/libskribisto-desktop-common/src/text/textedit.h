#ifndef TEXTEDIT_H
#define TEXTEDIT_H

#include "skribisto_desktop_common_global.h"
#include <QEvent>
#include <QTextEdit>

class SKRDESKTOPCOMMONEXPORT TextEdit : public QTextEdit
{
    Q_OBJECT
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
    QAction *bulletListAction() const;
    QAction *centerCursorAction() const;

    void centerCursor(bool force = false);

public slots:
    void adaptScollBarRange(int min, int max);

private slots:
    void updateFontActions();

private:
    QString m_uuid;
    QAction *m_boldAction, *m_italicAction, *m_strikeAction, *m_underlineAction, *m_bulletListAction, *m_centerCursorAction;
    QList<QMetaObject::Connection> m_actionConnectionsList;
    bool m_mouse_button_down, m_always_center_cursor;

    void connectActions();
    void disconnectActions();

    // QWidget interface
protected:
    void mousePressEvent(QMouseEvent *event) override;
    void mouseReleaseEvent(QMouseEvent *event) override;
    void mouseDoubleClickEvent(QMouseEvent *event) override;
    void focusOutEvent(QFocusEvent *event) override;
};

#endif // TEXTEDIT_H

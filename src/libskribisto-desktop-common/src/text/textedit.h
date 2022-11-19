#ifndef TEXTEDIT_H
#define TEXTEDIT_H

#include "skribisto_desktop_common_global.h"
#include "text/highlighter.h"
#include <QEvent>
#include <QMenu>
#include <QTextEdit>

class SKRDESKTOPCOMMONEXPORT TextEdit : public QTextEdit
{
    Q_OBJECT
public:
    TextEdit(QWidget *parent = nullptr, int projectId = -1);
    QString uuid() const;
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

    // highlighting / spell check
    void setupHighlighter();
    void setSpellcheckerEnabled(bool value);
    QStringList listSpellSuggestionsAtTextCursor() const;
    bool isWordMisspelled(int cursorPosition);
    QStringList listSpellSuggestionsAt(int cursorPosition) const;
    void replaceWordAt(int cursorPosition, const QString &newWord);

    int projectId() const;
    void setProjectId(int newProjectId);

public slots:
    void adaptScollBarRange(int min, int max);

signals:
    void activeFocusChanged(bool value);

private slots:
    void updateFontActions();

    void onCustomContextMenu(const QPoint &point);
private:
    QString m_uuid;
    QAction *m_boldAction, *m_italicAction, *m_strikeAction, *m_underlineAction, *m_bulletListAction,
    *m_centerCursorAction, *m_addToUserDictAction, *m_cutAction, *m_copyAction, *m_pasteAction, *m_pasteWithoutFormattingAction,
    *m_createNote;
    QList<QMetaObject::Connection> m_actionConnectionsList;
    bool m_mouse_button_down, m_always_center_cursor;
    Highlighter *m_highlighter;
    QMenu *m_contextMenu;
    bool m_forceDisableCenterCursor;
    QObject *m_connectionHolder;
    int m_projectId;

    void connectActions();
    void disconnectActions();

    void setupContextMenu();
protected:
    void mousePressEvent(QMouseEvent *event) override;
    void mouseReleaseEvent(QMouseEvent *event) override;
    void mouseDoubleClickEvent(QMouseEvent *event) override;

    // QWidget interface
protected:
    void keyPressEvent(QKeyEvent *event) override;
    void focusOutEvent(QFocusEvent *event) override;
    void focusInEvent(QFocusEvent *event) override;

    // QTextEdit interface
protected:
    bool canInsertFromMimeData(const QMimeData *source) const override;
    void insertFromMimeData(const QMimeData *source) override;
};

#endif // TEXTEDIT_H

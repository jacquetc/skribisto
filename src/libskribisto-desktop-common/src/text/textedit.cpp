#include "textedit.h"

#include <QUuid>

TextEdit::TextEdit(QWidget *parent)
{
    m_uuid = QUuid::createUuid().toString();

    connect(this, &QTextEdit::cursorPositionChanged, this, &TextEdit::updateFontActions);

    m_italicAction = new QAction(QIcon(":/icons/backup/format-text-italic.svg"), tr("Italic"), this);
    m_italicAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    m_italicAction->setShortcut(QKeySequence(tr("Ctrl+I")));
    m_italicAction->setCheckable(true);

    m_boldAction = new QAction(QIcon(":/icons/backup/format-text-bold.svg"), tr("Bold"), this);
    m_boldAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    m_boldAction->setShortcut(QKeySequence(tr("Ctrl+B")));
    m_boldAction->setCheckable(true);

    m_strikeAction = new QAction(QIcon(":/icons/backup/format-text-strikethrough.svg"), tr("Strikethrough"), this);
    m_strikeAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    m_strikeAction->setShortcut(QKeySequence(tr("Ctrl+T")));
    m_strikeAction->setCheckable(true);


    m_underlineAction = new QAction(QIcon(":/icons/backup/format-text-underline.svg"), tr("Underline"), this);
    m_underlineAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
    m_underlineAction->setShortcut(QKeySequence(tr("Ctrl+U")));
    m_underlineAction->setCheckable(true);

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

void TextEdit::connectActions()
{
    m_actionConnectionsList.clear();

    m_actionConnectionsList << QObject::connect(m_italicAction, &QAction::toggled, this, [this](bool checked){
            QTextCharFormat format;
            format.setFontItalic(checked);
            this->mergeCurrentCharFormat(format);
    });

    m_actionConnectionsList << QObject::connect(m_boldAction, &QAction::toggled, this, [this](bool checked){
            QTextCharFormat format;
            format.setFontWeight(checked ? 700 : 400);
            this->mergeCurrentCharFormat(format);
    });

    m_actionConnectionsList << QObject::connect(m_underlineAction, &QAction::toggled, this, [this](bool checked){
            QTextCharFormat format;
            format.setFontUnderline(checked);
            this->mergeCurrentCharFormat(format);
    });


    m_actionConnectionsList << QObject::connect(m_strikeAction, &QAction::toggled, this, [this](bool checked){
            QTextCharFormat format;
            format.setFontStrikeOut(checked);
            this->mergeCurrentCharFormat(format);
    });

}


void TextEdit::disconnectActions()
{

    for (const QMetaObject::Connection& connection : qAsConst(m_actionConnectionsList)) {
        QObject::disconnect(connection);
    }

    m_actionConnectionsList.clear();
}


QAction *TextEdit::boldAction() const
{
    return m_boldAction;
}

void TextEdit::updateFontActions()
{
    this->disconnectActions();

    const QFont font = this->currentFont();

    m_italicAction->setChecked(font.italic());
    m_boldAction->setChecked(font.bold());
    m_underlineAction->setChecked(font.underline());
    m_strikeAction->setChecked(font.strikeOut());

    this->connectActions();
}

QAction *TextEdit::underlineAction() const
{
    return m_underlineAction;
}

QAction *TextEdit::strikeAction() const
{
    return m_strikeAction;
}

QAction *TextEdit::italicAction() const
{
    return m_italicAction;
}

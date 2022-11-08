#include "outlineitemdelegate.h"
#include "markdowntextdocument.h"
#include "text/textedit.h"
#include "treemodels/projecttreeitem.h"
#include "text/textbridge.h"

#include <QPainter>
#include <skrdata.h>

OutlineItemDelegate::OutlineItemDelegate(QObject *parent)
    : QStyledItemDelegate{parent}
{

}


void OutlineItemDelegate::paint(QPainter *painter, const QStyleOptionViewItem &option, const QModelIndex &index) const
{

    int projectId = index.data(ProjectTreeItem::ProjectIdRole).toInt();
    int treeItemId = index.data(ProjectTreeItem::TreeItemIdRole).toInt();

    MarkdownTextDocument document;
    document.setSkribistoMarkdown(skrdata->treeHub()->getSecondaryContent(projectId, treeItemId));
    TextEdit textEdit;
    textEdit.setDocument(&document);
    textEdit.document()->setDocumentMargin(0);
    textEdit.setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    textEdit.setFrameShape(QFrame::Shape::NoFrame);
    textEdit.setFrameShadow(QFrame::Shadow::Plain);
    textEdit.setLineWidth(0);
    //initStyleOption(&option, index);


    painter->save();
    //qDebug() << option.rect;

    option.widget->style()->drawControl(QStyle::CE_ItemViewItem, &option, painter);
    textEdit.setGeometry(option.rect);

    painter->translate(option.rect.x(), option.rect.y());
    textEdit.render(painter);
    painter->restore();
    //QStyledItemDelegate::paint(painter, option, index);



}

//---------------------------------------------------------------------------

QSize OutlineItemDelegate::sizeHint(const QStyleOptionViewItem &option, const QModelIndex &index) const
{
    //qDebug() << "ee" << option.rect;

        int projectId = index.data(ProjectTreeItem::ProjectIdRole).toInt();
        int treeItemId = index.data(ProjectTreeItem::TreeItemIdRole).toInt();

        MarkdownTextDocument document;
        document.setSkribistoMarkdown(skrdata->treeHub()->getSecondaryContent(projectId, treeItemId));
        TextEdit textEdit;
        textEdit.setDocument(&document);
        textEdit.document()->setDocumentMargin(0);
        textEdit.setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
        textEdit.setFrameShape(QFrame::Shape::NoFrame);
        textEdit.setFrameShadow(QFrame::Shadow::Plain);
        textEdit.setLineWidth(0);

        return document.size().toSize();

    return QStyledItemDelegate::sizeHint(option,index);
}

//---------------------------------------------------------------------------

QWidget *OutlineItemDelegate::createEditor(QWidget *parent, const QStyleOptionViewItem &option, const QModelIndex &index) const
{
    TextEdit *textEdit = new TextEdit(parent);

    //textEdit->document()->setDocumentMargin(0);
    textEdit->setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    textEdit->setFrameShape(QFrame::Shape::NoFrame);
    textEdit->setFrameShadow(QFrame::Shadow::Plain);
    textEdit->setLineWidth(0);
    textEdit->setFocusPolicy(Qt::FocusPolicy::StrongFocus);
    textEdit->setParent(parent);

    return textEdit;
}

//---------------------------------------------------------------------------

void OutlineItemDelegate::setEditorData(QWidget *editor, const QModelIndex &index) const
{
    int projectId = index.data(ProjectTreeItem::ProjectIdRole).toInt();
    int treeItemId = index.data(ProjectTreeItem::TreeItemIdRole).toInt();

    TextEdit * textEdit = qobject_cast<TextEdit*>(editor);

    MarkdownTextDocument *document = new MarkdownTextDocument(editor);
    document->setSkribistoMarkdown(skrdata->treeHub()->getSecondaryContent(projectId, treeItemId));
    textEdit->setDocument(document);
    textEdit->document()->setDocumentMargin(0);


    QString uniqueDocumentReference = QString("%1_%2_%3").arg(QString::number(projectId), QString::number(treeItemId), "secondary");
    textBridge->subscribeTextDocument(
                uniqueDocumentReference,
                textEdit->uuid(),
                document);

    QTextCursor textCursor  = textEdit->textCursor();
    textCursor.movePosition(QTextCursor::End);
    textEdit->setTextCursor(textCursor);
}

//---------------------------------------------------------------------------

void OutlineItemDelegate::setModelData(QWidget *editor, QAbstractItemModel *model, const QModelIndex &index) const
{
    int projectId = index.data(ProjectTreeItem::ProjectIdRole).toInt();
    int treeItemId = index.data(ProjectTreeItem::TreeItemIdRole).toInt();

    TextEdit *textEditor = qobject_cast<TextEdit *>(editor);
    MarkdownTextDocument *document = qobject_cast<MarkdownTextDocument *>(textEditor->document());

    skrdata->treeHub()->setSecondaryContent(projectId, treeItemId, document->toSkribistoMarkdown());

    emit editFinished(index);
}


//---------------------------------------------------------------------------

bool OutlineItemDelegate::eventFilter(QObject *watched, QEvent *event)
{
    return QStyledItemDelegate::eventFilter(watched, event);
}

//---------------------------------------------------------------------------

bool OutlineItemDelegate::editorEvent(QEvent *event, QAbstractItemModel *model, const QStyleOptionViewItem &option, const QModelIndex &index)
{


//    if(event->type() == QEvent::MouseButtonRelease)
//    {

//        return true;
//    }
    if(event->type() == QEvent::Wheel)
    {
        return true;
    }

    return QStyledItemDelegate::editorEvent(event, model,option,index);
}

//---------------------------------------------------------------------------

void OutlineItemDelegate::updateEditorGeometry(QWidget *editor, const QStyleOptionViewItem &option, const QModelIndex &index) const
{

editor->setGeometry(option.rect);
}

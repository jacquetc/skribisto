#include "outlineitemdelegate.h"
#include "text/markdowntextdocument.h"
#include "text/textedit.h"
#include "treemodels/projecttreeitem.h"
#include "text/textbridge.h"

#include <QPainter>
#include <QTreeView>
#include <QHeaderView>
#include <skrdata.h>
#include <QVBoxLayout>

OutlineItemDelegate::OutlineItemDelegate(QObject *parent)
    : QStyledItemDelegate{parent}
{

}


void OutlineItemDelegate::paint(QPainter *painter, const QStyleOptionViewItem &option, const QModelIndex &index) const
{
    //return QStyledItemDelegate::paint(painter, option, index);
//    QElapsedTimer timer;
//    timer.start();
//    qDebug() << "!" << timer.elapsed();

    QString content = index.data(ProjectTreeItem::SecondaryContentRole).toString();

    if(content.isEmpty()){
        return QStyledItemDelegate::paint(painter, option, index);
    }

    MarkdownTextDocument document;
    document.setSkribistoMarkdown(content);


    TextEdit textEdit;
    textEdit.setDocument(&document);
    textEdit.document()->setDocumentMargin(0);
    textEdit.setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    textEdit.setFrameShape(QFrame::Shape::NoFrame);
    textEdit.setFrameShadow(QFrame::Shadow::Plain);
    textEdit.setLineWidth(0);
    //initStyleOption(&option, index);


    painter->save();

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
//    return QStyledItemDelegate::sizeHint(option,index);


    QString content = index.data(ProjectTreeItem::SecondaryContentRole).toString();

    if(content.isEmpty()){
        return QStyledItemDelegate::sizeHint(option,index);
    }

    QTreeView *treeView = static_cast<QTreeView *>(this->parent());
    int sectionWidth = treeView->header()->sectionSize(1);

    MarkdownTextDocument document;
    document.setSkribistoMarkdown(content);
    TextEdit textEdit;
    textEdit.setFixedWidth(sectionWidth);
    textEdit.setDocument(&document);
    textEdit.document()->setDocumentMargin(0);
    textEdit.setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    textEdit.setFrameShape(QFrame::Shape::NoFrame);
    textEdit.setFrameShadow(QFrame::Shadow::Plain);
    textEdit.setLineWidth(0);

    return document.size().toSize();

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
    TreeItemAddress treeItemAddress = index.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>();

    TextEdit * textEdit = qobject_cast<TextEdit*>(editor);
    textEdit->setProjectId(treeItemAddress.projectId);

    MarkdownTextDocument *document = new MarkdownTextDocument(editor);
    document->setSkribistoMarkdown(index.data(ProjectTreeItem::SecondaryContentRole).toString());
    textEdit->setDocument(document);
    textEdit->document()->setDocumentMargin(0);


    QString uniqueDocumentReference = QString("%1_%2_%3").arg(QString::number(treeItemAddress.projectId), QString::number(treeItemAddress.itemId), "secondary");
    textBridge->subscribeTextDocument(
                uniqueDocumentReference,
                textEdit->uuid(),
                document);

    QTextCursor textCursor  = textEdit->textCursor();
    textCursor.movePosition(QTextCursor::End);
    textEdit->setTextCursor(textCursor);



    QSettings settings;

    // spellchecker :
    int projectId = index.data(ProjectTreeItem::ProjectIdRole).toInt();
    textEdit->setProjectId(projectId);
    textEdit->setupHighlighter();
    textEdit->setSpellcheckerEnabled(settings.value("common/spellChecker", true).toBool());

}

//---------------------------------------------------------------------------

void OutlineItemDelegate::setModelData(QWidget *editor, QAbstractItemModel *model, const QModelIndex &index) const
{
    TreeItemAddress treeItemAddress = index.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>();

    TextEdit *textEditor = qobject_cast<TextEdit *>(editor);
    MarkdownTextDocument *document = qobject_cast<MarkdownTextDocument *>(textEditor->document());

    skrdata->treeHub()->setSecondaryContent(treeItemAddress, document->toSkribistoMarkdown());


    QString uniqueDocumentReference = QString("%1_%2_%3").arg(QString::number(treeItemAddress.projectId), QString::number(treeItemAddress.itemId), "secondary");
    textBridge->unsubscribeTextDocument(
                uniqueDocumentReference,
                textEditor->uuid(),
                document);

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
//qDebug() << "updateEditorGeometry";
editor->setGeometry(option.rect);
}

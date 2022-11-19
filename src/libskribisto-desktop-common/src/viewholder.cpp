#include "viewholder.h"
#include "invoker.h"
#include "skrdata.h"
#include "ui_viewholder.h"
#include "viewmanager.h"
#include <QMimeData>

ViewHolder::ViewHolder(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::ViewHolder), m_uuid(QUuid::createUuid()), m_currentHistoryItem(nullptr)

{
    ui->setupUi(this);

// history:
    m_goBackAction = new QAction(QIcon(":/icons/backup/go-previous-view.svg"), tr("Back"), this);
    m_goBackAction->setShortcutContext(Qt::ShortcutContext::WidgetWithChildrenShortcut);
    m_goBackAction->setShortcut(QKeySequence(QKeySequence::Back));
    this->addAction(m_goBackAction);
    connect(m_goBackAction, &QAction::triggered, this, &ViewHolder::goBackInHistory);

    m_goForwardAction = new QAction(QIcon(":/icons/backup/go-next-view.svg"), tr("Forward"), this);
    m_goForwardAction->setShortcutContext(Qt::ShortcutContext::WidgetWithChildrenShortcut);
    m_goForwardAction->setShortcut(QKeySequence(QKeySequence::Forward));
    this->addAction(m_goForwardAction);
    connect(m_goForwardAction, &QAction::triggered, this, &ViewHolder::goForwardInHistory);
}

//--------------------------------------------------------------

ViewHolder::~ViewHolder()
{
    qDeleteAll(m_historyItemList);
    delete ui;
}

//--------------------------------------------------------------

QList<View *> ViewHolder::viewList() const
{
    return m_viewList;
}

//--------------------------------------------------------------

void ViewHolder::addView(View *view)
{
    emit viewtoBeAdded(view);

    ui->stackedWidget->addWidget(view);
    m_viewList.append(view);

    // history
    view->setHistoryActions(m_goBackAction, m_goForwardAction);
    connect(view, &View::addToHistoryCalled, this, &ViewHolder::addToHistory);

    ui->stackedWidget->setCurrentWidget(view);

    emit viewAdded(view);
}

//--------------------------------------------------------------

void ViewHolder::removeView(View *view)
{
    if(!m_viewList.contains(view)){
        return;
    }

    emit viewToBeRemoved(view);
    emit view->aboutToBeDestroyed();



    ui->stackedWidget->removeWidget(view);
    m_viewList.removeAll(view);
    view->hide();
    delete view;

    emit viewRemoved();
}

//--------------------------------------------------------------

void ViewHolder::removeViews(const TreeItemAddress &treeItemAddress)
{
    QList<View*> m_tempViewList = m_viewList;

    if(treeItemAddress.hasOnlyProjectId()){
        for(View *view : m_tempViewList){

            if(view->projectId() == treeItemAddress.projectId){
                this->removeView(view);
            }
        }

    }
    else {

        for(View *view : m_tempViewList){

            if(view->treeItemAddress() == treeItemAddress){
                this->removeView(view);
            }
        }
    }


}

//--------------------------------------------------------------

void ViewHolder::clear()
{
    QList<View*> m_tempViewList = m_viewList;
    for(View *view : m_tempViewList){
        this->removeView(view);
    }

    if(ui->stackedWidget->count() > 0){
        qFatal("clear() : ui->stackedWidget->count() > 0 ");
    }


}


//--------------------------------------------------------------

View *ViewHolder::currentView() const
{
    return static_cast<View *>(ui->stackedWidget->currentWidget());
}
//--------------------------------------------------------------

void ViewHolder::setCurrentView(View *view)
{
    if(m_viewList.contains(view)){
        ui->stackedWidget->setCurrentWidget(view);
    }
}

QUuid ViewHolder::uuid() const
{
    return m_uuid;
}

void ViewHolder::setUuid(const QUuid &newUuid)
{
    if (m_uuid == newUuid)
        return;
    m_uuid = newUuid;
    emit uuidChanged();
}

//--------------------------------------------------

QAction *ViewHolder::goBackAction() const
{
    return m_goBackAction;
}

//--------------------------------------------------

QAction *ViewHolder::goForwardAction() const
{
    return m_goForwardAction;
}

//--------------------------------------------------

void ViewHolder::addToHistory(View *view, const QVariantMap &parameters)
{
    HistoryItem *historyItem = new HistoryItem(view, parameters);


    if(!m_currentHistoryItem && m_historyItemList.isEmpty()){
        m_historyItemList << historyItem;

        m_goBackAction->setEnabled(false);
        m_goForwardAction->setEnabled(false);
    }

    else if(!m_historyItemList.isEmpty() && m_currentHistoryItem == m_historyItemList.last()){
        QString comparedParameterKey = m_currentHistoryItem->parameters().value("comparedParameterKey", QString()).toString();
        if(m_currentHistoryItem->parameters().value(comparedParameterKey, QString()) == parameters.value(comparedParameterKey, QString()) && m_currentHistoryItem->view() == view){
            delete historyItem;
            return;
        }

        m_historyItemList << historyItem;

        m_goBackAction->setEnabled(true);
        m_goForwardAction->setEnabled(false);
    }
    else if(!m_historyItemList.isEmpty() && m_currentHistoryItem != m_historyItemList.last()){
        QString comparedParameterKey = m_currentHistoryItem->parameters().value("comparedParameterKey", QString()).toString();
        if(m_currentHistoryItem->parameters().value(comparedParameterKey, QString()) == parameters.value(comparedParameterKey, QString()) && m_currentHistoryItem->view() == view){
            delete historyItem;
            return;
        }

        int index = m_historyItemList.indexOf(m_currentHistoryItem);
        m_historyItemList.insert(index + 1, historyItem);

        m_goBackAction->setEnabled(true);
        m_goForwardAction->setEnabled(true);
    }

    m_currentHistoryItem = historyItem;


}

//--------------------------------------------------------------

void ViewHolder::clearHistoryOfView(View *view)
{
    QList<HistoryItem *> listToKeep;
    QList<HistoryItem *> listToRemove;

    for(HistoryItem *historyItem  : m_historyItemList){
        if(historyItem->view() == view){
            listToRemove.append(historyItem);
        }
        else {
            listToKeep.append(historyItem);
        }
    }

    m_historyItemList = listToKeep;
    qDeleteAll(listToRemove);

    if(m_historyItemList.isEmpty()){
        m_currentHistoryItem = nullptr;

        m_goBackAction->setEnabled(false);
        m_goForwardAction->setEnabled(false);
    }
    else{
        m_currentHistoryItem = m_historyItemList.last();

        m_goBackAction->setEnabled(m_historyItemList.count() > 1);
        m_goForwardAction->setEnabled(false);
    }


}

//--------------------------------------------------------------

void ViewHolder::goBackInHistory()
{
    qDebug() << "goBackAction";
    int index = m_historyItemList.indexOf(m_currentHistoryItem);

    if(index == 1){
        m_currentHistoryItem = m_historyItemList.at(index - 1);
        this->setCurrentView(m_currentHistoryItem->view());
        m_currentHistoryItem->view()->applyHistoryParameters(m_currentHistoryItem->parameters());
        m_goBackAction->setEnabled(false);
        m_goForwardAction->setEnabled(true);
    }
    else if(index == 0){
        qFatal("goBackInHistory index == 0");
    }
    else {
        m_currentHistoryItem = m_historyItemList.at(index - 1);
        this->setCurrentView(m_currentHistoryItem->view());
        m_currentHistoryItem->view()->applyHistoryParameters(m_currentHistoryItem->parameters());
        m_goBackAction->setEnabled(true);
        m_goForwardAction->setEnabled(true);
    }

}

//--------------------------------------------------------------

void ViewHolder::goForwardInHistory()
{
    int index = m_historyItemList.indexOf(m_currentHistoryItem);

    if(index == m_historyItemList.count() - 2){
        m_currentHistoryItem = m_historyItemList.at(index + 1);
        this->setCurrentView(m_currentHistoryItem->view());
        m_currentHistoryItem->view()->applyHistoryParameters(m_currentHistoryItem->parameters());
        m_goBackAction->setEnabled(true);
        m_goForwardAction->setEnabled(false);
    }
    else if(index == m_historyItemList.count() - 1){
        qFatal("goForwardInHistory index == last");
    }
    else {
        m_currentHistoryItem = m_historyItemList.at(index + 1);
        this->setCurrentView(m_currentHistoryItem->view());
        m_currentHistoryItem->view()->applyHistoryParameters(m_currentHistoryItem->parameters());
        m_goBackAction->setEnabled(true);
        m_goForwardAction->setEnabled(true);
    }
}

//--------------------------------------------------------------
//--------------------------------------------------------------
//--------------------------------------------------------------

HistoryItem::HistoryItem(View *view, const QVariantMap &parameters):
    m_uuid(QUuid::createUuid()),
    m_date(QDateTime::currentDateTime()),
    m_view(view),
    m_parameters(parameters)
{

}
//--------------------------------------------------------------

QDateTime HistoryItem::date() const
{
    return m_date;
}

//--------------------------------------------------------------

View *HistoryItem::view() const
{
    return m_view;
}

//--------------------------------------------------------------

QVariantMap HistoryItem::parameters() const
{
    return m_parameters;
}

//--------------------------------------------------------------

bool HistoryItem::operator==(const HistoryItem &otherHistoryItem) const
{
    return this->m_uuid == otherHistoryItem.m_uuid;
}


void ViewHolder::dropEvent(QDropEvent *event)
{
    if(event->mimeData()->hasFormat("application/x-navigationtreeitem-list")){

        QList< TreeItemAddress >  sourceTreeItemAddresses;
        QByteArray byteArray = event->mimeData()->data("application/x-navigationtreeitem-list");
        QDataStream stream(&byteArray, QIODevice::ReadOnly);
        stream >> sourceTreeItemAddresses;


        QString type = skrdata->treeHub()->getType(sourceTreeItemAddresses.first());

        ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
        viewManager->openViewAt(this, type, sourceTreeItemAddresses.first());

        this->setCursor(QCursor(Qt::ArrowCursor));
        event->acceptProposedAction();

    }
}


void ViewHolder::dragEnterEvent(QDragEnterEvent *event)
{
   if(event->mimeData()->hasFormat("application/x-navigationtreeitem-list")){
       this->setCursor(QCursor(Qt::DragCopyCursor));
      event->acceptProposedAction();
   }
}

void ViewHolder::dragLeaveEvent(QDragLeaveEvent *event)
{
//   QGuiApplication::restoreOverrideCursor();
   this->setCursor(QCursor(Qt::ArrowCursor));
//        event->accept();

}


void ViewHolder::dragMoveEvent(QDragMoveEvent *event)
{
//    if(event->mimeData()->hasFormat("application/x-navigationtreeitem-list")){

//        this->setCursor(QCursor(Qt::DragCopyCursor));
//    }
}

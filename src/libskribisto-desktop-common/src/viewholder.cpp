#include "viewholder.h"
#include "ui_viewholder.h"

ViewHolder::ViewHolder(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::ViewHolder), m_uuid(QUuid::createUuid())

{
    ui->setupUi(this);



}

//--------------------------------------------------------------

ViewHolder::~ViewHolder()
{
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

void ViewHolder::removeViews(int projectId, int treeItemId)
{
    QList<View*> m_tempViewList = m_viewList;

    if(treeItemId == -1){
        for(View *view : m_tempViewList){

            if(view->projectId() == projectId){
                this->removeView(view);
            }
        }

    }
    else {

        for(View *view : m_tempViewList){

            if(view->projectId() == projectId && view->treeItemId() == treeItemId){
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

//--------------------------------------------------------------

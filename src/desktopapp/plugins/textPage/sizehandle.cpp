#include "sizehandle.h"

SizeHandle::SizeHandle(QWidget *parent)
    : QWidget{parent}
{

    this->setMouseTracking(true);
    this->setAutoFillBackground(true);
    this->setBackgroundRole(QPalette::Button);

}




void SizeHandle::mouseMoveEvent(QMouseEvent *event)
{
    if (event->buttons() & Qt::LeftButton){

        int delta = event->pos().x() - m_oldPoint.x();
        emit moved(delta);

        m_oldPoint = event->pos();

        event->accept();


    }
}


void SizeHandle::mousePressEvent(QMouseEvent *event)
{
    m_oldPoint = event->pos();
}


void SizeHandle::enterEvent(QEnterEvent *event)
{
    this->setBackgroundRole(QPalette::ButtonText);
}

void SizeHandle::leaveEvent(QEvent *event)
{
    this->setBackgroundRole(QPalette::Window);
}

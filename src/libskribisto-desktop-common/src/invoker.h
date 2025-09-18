#pragma once


#include <QString>
#include <QWidget>



 template <class Object> Object* invoke(QWidget *widget, const QString &objectName) {
    return widget->window()->findChild<Object*>(objectName);
}




#ifndef INVOKER_H
#define INVOKER_H

#include <QString>
#include <QWidget>



 template <class Object> Object* invoke(QWidget *widget, const QString &objectName) {
    return widget->window()->findChild<Object*>(objectName);
}


#endif // INVOKER_H

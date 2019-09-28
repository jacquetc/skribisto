#include "plmerrorhub.h"

#include <QMetaType>
#include <QtDebug>
#include <QMessageLogger>

PLMErrorHub::PLMErrorHub(QObject *parent) : QObject(parent)
{
qRegisterMetaType<QList<PLMError> >("QList<PLMError>");
}

//QList<QString> PLMErrorHub::getlatestError() const
//{
//    QList<QString> finalList;

//    if(m_errorList.isEmpty())
//        return finalList;

//    PLMError error = m_errorList.last();
//    finalList = error.getAll();
//    return finalList;
//}

//QList<QString> PLMErrorHub::getError(int index) const
//{
//    QList<QString> finalList;

//    if(m_errorList.isEmpty())
//        return finalList;

//    PLMError error = m_errorList.at(index);
//    finalList = error.getAll();
//    return finalList;
//}

//int PLMErrorHub::count()
//{
//    return m_errorList.count();
//}

//void PLMErrorHub::clear()
//{
//    m_errorList.clear();
//}

//void PLMErrorHub::addError(const QString &errorCode, const QString &origin, const QString &message)
//{
//    m_errorList << PLMError(errorCode, origin, message);
//    emit errorSent(errorCode, origin, message);
//    emit errorSent();
//    qCritical() << errorCode << " : " << origin << " : " << message;
//}


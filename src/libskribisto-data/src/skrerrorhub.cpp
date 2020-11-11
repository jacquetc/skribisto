#include "skrerrorhub.h"

#include <QMetaType>
#include <QtDebug>
#include <QMessageLogger>

SKRErrorHub::SKRErrorHub(QObject *parent) : QObject(parent)
{
    qRegisterMetaType<QList<SKRResult> >("QList<SKRResult>");
}

void SKRErrorHub::addError(SKRResult result)
{
    m_resultList.append(result);


    // sent result signal for notification

    emit sendNotification(SKRResult::Critical, result.getErrorCodeList().join("\n"));
qCritical() << "\nCRITICAL\n" <<  result.getErrorCodeList().join("\n") << "\n\n"
            << result.getDataHash().values();

}

// QList<QString> SKRErrorHub::getlatestError() const
// {
//    QList<QString> finalList;

//    if(m_errorList.isEmpty())
//        return finalList;

//    SKRResult result = m_errorList.last();
//    finalList = result.getAll();
//    return finalList;
// }

// QList<QString> SKRErrorHub::getError(int index) const
// {
//    QList<QString> finalList;

//    if(m_errorList.isEmpty())
//        return finalList;

//    SKRResult result = m_errorList.at(index);
//    finalList = result.getAll();
//    return finalList;
// }

// int SKRErrorHub::count()
// {
//    return m_errorList.count();
// }

// void SKRErrorHub::clear()
// {
//    m_errorList.clear();
// }

// void SKRErrorHub::addError(const QString &errorCode, const QString &origin,
// const QString &message)
// {
//    m_errorList << SKRResult(errorCode, origin, message);
//    emit errorSent(errorCode, origin, message);
//    emit errorSent();
//    qCritical() << errorCode << " : " << origin << " : " << message;
// }

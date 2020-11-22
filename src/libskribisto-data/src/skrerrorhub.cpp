#include "skrerrorhub.h"

#include <QMetaType>
#include <QtDebug>
#include <QMessageLogger>
#include <QMetaEnum>

SKRErrorHub::SKRErrorHub(QObject *parent) : QObject(parent)
{
    qRegisterMetaType<QList<SKRResult> >("QList<SKRResult>");
}

void SKRErrorHub::addError(SKRResult result)
{
    m_resultList.append(result);


    // sent result signal for notification

    QStringList errorCodeList = result.getErrorCodeList();
    QList<QHash<QString, QVariant> > dataHashList = result.getDataHashList();

    QMetaEnum statusMetaEnum = QMetaEnum::fromType<SKRResult::Status>();
    QString consoleString = "\n" + QString(statusMetaEnum.valueToKey(result.getStatus())) + " : \n";

    for(int i = 0 ; i < errorCodeList.count(); i++){

        // fill string for console :

        QString errorCode = errorCodeList.at(i);

        consoleString += "    " + errorCode + "\n";

        QHash<QString, QVariant> dataHash = dataHashList.at(i);


        QHash<QString, QVariant>::const_iterator h = dataHash.constBegin();
        while (h != dataHash.constEnd()) {

            QString keyString = h.key();

            QString valueString = h.value().toString();

            if(valueString == ""){
                valueString = "unreadable value";
            }

            consoleString += "        " + keyString + " : " + valueString  + "\n";


            ++h;
        }
        //TODO: create notification  content :

    }

    emit sendNotification(SKRResult::Critical, consoleString);


    for(const QString &consoleStr : consoleString.split("\n", Qt::KeepEmptyParts)){
        qDebug() << consoleStr;
    }
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

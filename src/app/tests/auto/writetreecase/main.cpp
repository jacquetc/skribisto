#include <QtQuickTest>
#include <QQmlEngine>
#include <QQmlContext>

#include "skrdata.h"

class TestSetup : public QObject
{
public:
    TestSetup() {}

public slots:
    void qmlEngineAvailable(QQmlEngine *engine)
    {
        SKRData *data = new SKRData(qApp);
        qDebug() << "vvvvv";
        engine->rootContext()->setContextProperty("skrData", data);
    }
};

QUICK_TEST_MAIN_WITH_SETUP(writetreecase, TestSetup)

#include "tst_writetreecase.moc"

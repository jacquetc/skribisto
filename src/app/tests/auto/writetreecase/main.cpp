#include <QtQuickTest>
#include <QQmlEngine>
#include <QQmlContext>

#include "plmdata.h"

class TestSetup : public QObject
{
public:
    TestSetup() {}

public slots:
    void qmlEngineAvailable(QQmlEngine *engine)
    {
        PLMData *data = new PLMData(qApp);
        qDebug() << "vvvvv";
        engine->rootContext()->setContextProperty("plmData", data);
    }
};

QUICK_TEST_MAIN_WITH_SETUP(writetreecase, TestSetup)

#include "tst_writetreecase.moc"

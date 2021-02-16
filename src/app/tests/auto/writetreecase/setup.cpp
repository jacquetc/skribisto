#include <QtQuickTest>
#include <QQmlEngine>
#include <QQmlContext>

#include "plmdata.h"

class Setup : public QObject
{
public:
    Setup() {}

public slots:
    void qmlEngineAvailable(QQmlEngine *engine);

};

QUICK_TEST_MAIN_WITH_SETUP(writetreecase, Setup)

void Setup::qmlEngineAvailable(QQmlEngine *engine)
{
    PLMData *data = new PLMData(qApp);
    qDebug() << "vvvvv";
    engine->rootContext()->setContextProperty("plmData", data);
}
//#include "tst_writetreecase.moc"

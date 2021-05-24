#include <QtQuickTest>
#include <QQmlEngine>
#include <QQmlContext>

#include "skrdata.h"

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
    SKRData *data = new SKRData(qApp);
    qDebug() << "vvvvv";
    engine->rootContext()->setContextProperty("skrData", data);
}
//#include "tst_writetreecase.moc"

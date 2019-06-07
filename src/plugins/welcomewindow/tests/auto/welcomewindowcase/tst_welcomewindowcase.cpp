#include <QString>
#include <QtTest>
#include <QDebug>

#include "plmwindow.h"
#include "plmbasewindow.h"

class WelcomeWindowCase : public QObject
{
    Q_OBJECT

public:
    WelcomeWindowCase();

public slots:
private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void displayWindow();
private:
};

WelcomeWindowCase::WelcomeWindowCase()
{
}

void WelcomeWindowCase::initTestCase()
{

}

void WelcomeWindowCase::cleanupTestCase()
{

}

void WelcomeWindowCase::init()
{
}

void WelcomeWindowCase::cleanup()
{

}

void WelcomeWindowCase::displayWindow()
{
    PLMWindow window(nullptr, "welcomeWindow");

    //QCOMPARE(window.windowTitle(), tr("Welcome"));

}
QTEST_MAIN(WelcomeWindowCase)

#include "tst_welcomewindowcase.moc"

#include <QString>
#include <QtTest>
#include <QDebug>

#include "plmdata.h"

class OpenProjectCase : public QObject
{
    Q_OBJECT

public:
    OpenProjectCase();

public slots:
private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();
    void testCase1_data();
    void testCase1();

    void openOneProject();
    void openTwoProjects();
    void closeOneProject();
    void closeTwoProjects();

    void getPath();

    void getProjectIdList();
    void getLastLoaded();
private:
    PLMData *m_data;
    QString m_testProjectPath;

};

OpenProjectCase::OpenProjectCase()
{
}

void OpenProjectCase::initTestCase()
{
    m_testProjectPath = ":/testfiles/plume_test_project.sqlite";
}

void OpenProjectCase::cleanupTestCase()
{
}

void OpenProjectCase::init()
{
    m_data = new PLMData(this);
}

void OpenProjectCase::cleanup()
{
    QSignalSpy spy(plmdata->projectHub(), SIGNAL(allProjectsClosed()));
    plmdata->projectHub()->closeAllProjects();
    QCOMPARE(spy.count(), 1);
    m_data->deleteLater();
}

void OpenProjectCase::openOneProject()
{
    QSignalSpy spy(plmdata->projectHub(), SIGNAL(projectLoaded(int)));
    plmdata->projectHub()->loadProject(m_testProjectPath);
    QCOMPARE(spy.count(), 1);
}

void OpenProjectCase::closeOneProject()
{
    QSignalSpy spy(plmdata->projectHub(), SIGNAL(projectLoaded(int)));
    plmdata->projectHub()->loadProject(m_testProjectPath);
    QCOMPARE(spy.count(), 1);
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal
    int projectId = arguments.at(0).toInt();
    QSignalSpy spy2(plmdata->projectHub(), SIGNAL(projectClosed(int)));
    plmdata->projectHub()->closeProject(projectId);
    QCOMPARE(spy2.count(), 1);
}

void OpenProjectCase::openTwoProjects()
{
    QSignalSpy spy(plmdata->projectHub(), SIGNAL(projectLoaded(int)));
    plmdata->projectHub()->loadProject(m_testProjectPath);
    QCOMPARE(spy.count(), 1);
    QList<QVariant> arguments = spy.takeFirst(); // take the first signal
    int projectId1 = arguments.at(0).toInt();
    QSignalSpy spy2(plmdata->projectHub(), SIGNAL(projectLoaded(int)));
    plmdata->projectHub()->loadProject(m_testProjectPath);
    QCOMPARE(spy.count(), 1);
    QList<QVariant> arguments2 = spy2.takeFirst(); // take the first signal
    int projectId2 = arguments2.at(0).toInt();
    QVERIFY(projectId1 < projectId2);
}


void OpenProjectCase::closeTwoProjects()
{
    openTwoProjects();
    QSignalSpy spy1(plmdata->projectHub(), SIGNAL(projectClosed(int)));
    plmdata->projectHub()->closeProject(1);
    QCOMPARE(spy1.count(), 1);
    QSignalSpy spy2(plmdata->projectHub(), SIGNAL(projectClosed(int)));
    plmdata->projectHub()->closeProject(2);
    QCOMPARE(spy2.count(), 1);
}


//------------------------------------------------------------------------------------

void OpenProjectCase::getPath()
{
    openOneProject();
    QList<int> idList = plmdata->projectHub()->getProjectIdList();

    if (idList.isEmpty()) {
        qDebug() << "no project id";
        QVERIFY(true == false);
        return;
    }

    QString value = plmdata->projectHub()->getPath(idList.first());
    QCOMPARE(value, QString(m_testProjectPath));
}

void OpenProjectCase::getProjectIdList()
{
    openOneProject();
    QList<int> list = plmdata->projectHub()->getProjectIdList();
    QVERIFY(!list.isEmpty());
}


void OpenProjectCase::getLastLoaded()
{
    openOneProject();
    int id = plmdata->projectHub()->getLastLoaded();
    QVERIFY(id > 0);
}




void OpenProjectCase::testCase1_data()
{
    QTest::addColumn<QString>("data");
    QTest::newRow("0") << QString();
}

void OpenProjectCase::testCase1()
{
    QFETCH(QString, data);
    QVERIFY2(true, "Failure");
}

QTEST_GUILESS_MAIN(OpenProjectCase)

#include "tst_openprojectcase.moc"
